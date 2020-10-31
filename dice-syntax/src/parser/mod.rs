mod rules;

use super::{
    lexer::{Lexer, TokenKind},
    Assignment, AssignmentOperator, Binary, BinaryOperator, Block, Break, Continue, ExportDecl, FnCall, FnDecl,
    IfExpression, LitAnonymousFn, LitBool, LitFloat, LitIdent, LitInt, LitList, LitNull, LitObject, LitString, LitUnit,
    Return, SyntaxNode, SyntaxNodeId, SyntaxTree, Unary, UnaryOperator, VarDecl, WhileLoop,
};
use crate::{
    parser::rules::{ParserRule, RulePrecedence},
    AbstractFnDecl, ClassDecl, ErrorPropagate, FieldAccess, FnArg, ForLoop, ImportDecl, Index, Loop, NullPropagate,
    OpDecl, OverloadedOperator, TraitDecl, TraitImpl, TypeAnnotation, VarDeclKind,
};
use dice_error::{span::Span, syntax_error::SyntaxError};
use id_arena::Arena;

pub type SyntaxNodeResult = Result<SyntaxNodeId, SyntaxError>;

pub struct Parser {
    lexer: Lexer,
    arena: Arena<SyntaxNode>,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let lexer = Lexer::from_str(input);
        let arena = Arena::new();

        Self { lexer, arena }
    }

    // TODO: Have this return a collection of parse errors.
    pub fn parse(mut self) -> Result<SyntaxTree, SyntaxError> {
        let root = self.expression_sequence()?;

        Ok(SyntaxTree::new(root, self.arena))
    }

    fn expression_sequence(&mut self) -> SyntaxNodeResult {
        let mut expressions = Vec::new();
        let mut next_token = self.lexer.peek()?;
        let span_start = next_token.span();
        let mut trailing_expression = None;

        while !matches!(next_token.kind, TokenKind::EndOfInput | TokenKind::RightCurly) {
            let expression = match next_token.kind {
                TokenKind::If => self.if_expression(false)?,
                TokenKind::Loop => self.loop_statement()?,
                TokenKind::While => self.while_statement()?,
                TokenKind::For => self.for_statement()?,
                TokenKind::Let => self.var_decl()?,
                TokenKind::Function => self.fn_decl()?,
                TokenKind::Operator => self.op_decl()?,
                TokenKind::Class => self.class_decl()?,
                TokenKind::Trait => self.trait_decl()?,
                TokenKind::Impl => self.trait_impl()?,
                TokenKind::Import => self.import_decl()?,
                TokenKind::Export => self.export_decl()?,
                TokenKind::Return | TokenKind::Break | TokenKind::Continue => self.control_flow()?,
                _ => self.expression()?,
            };

            next_token = self.lexer.peek()?;

            if matches!(next_token.kind, TokenKind::EndOfInput | TokenKind::RightCurly) {
                trailing_expression = Some(expression);
                break;
            }

            if next_token.kind == TokenKind::Semicolon {
                self.lexer.consume(TokenKind::Semicolon)?;
                next_token = self.lexer.peek()?;
            }

            expressions.push(expression);
        }

        let span_end = next_token.span();
        let node = SyntaxNode::Block(Block {
            expressions,
            trailing_expression,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn expression(&mut self) -> SyntaxNodeResult {
        self.parse_precedence(RulePrecedence::Assignment)
    }

    fn parse_precedence(&mut self, precedence: RulePrecedence) -> SyntaxNodeResult {
        let next_token = self.lexer.peek()?;
        let rule = ParserRule::for_token(&next_token)?;
        let mut node = rule
            .prefix
            .map(|prefix| prefix(self, precedence <= RulePrecedence::Assignment))
            .unwrap_or_else(|| Err(next_token.clone().into()))?;

        loop {
            let span_start = next_token.span();
            let next_token = self.lexer.peek()?;
            let rule = ParserRule::for_token(&next_token)?;

            if let Some(postfix_precedence) = rule.postfix_precedence {
                if precedence > postfix_precedence {
                    break;
                }

                node = rule
                    .postfix
                    .map(|postfix| postfix(self, node, precedence <= RulePrecedence::Assignment, span_start))
                    .unwrap_or_else(|| Err(next_token.into()))?;

                continue;
            }

            if precedence > rule.infix_precedence {
                break;
            }

            node = rule
                .infix
                .map(|infix| infix(self, node, precedence <= RulePrecedence::Assignment, span_start))
                .unwrap_or_else(|| Err(next_token.into()))?;
        }

        Ok(node)
    }

    fn if_expression(&mut self, _: bool) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::If)?.span();
        let condition = self.expression()?;
        let primary = self.block_expression(false)?;
        let secondary = if self.lexer.peek()?.kind == TokenKind::Else {
            self.lexer.consume(TokenKind::Else)?;

            match self.lexer.peek()?.kind {
                TokenKind::If => Some(self.if_expression(false)?),
                TokenKind::LeftCurly => Some(self.block_expression(false)?),
                _ => None,
            }
        } else {
            None
        };
        let span_end = self.lexer.current().span();
        let node = SyntaxNode::IfExpression(IfExpression {
            condition,
            primary,
            secondary,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn loop_statement(&mut self) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::Loop)?.span();
        let body = self.block_expression(false)?;
        let span_end = self.lexer.current().span();
        let node = SyntaxNode::Loop(Loop {
            body,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn while_statement(&mut self) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::While)?.span();
        let condition = self.expression()?;
        let body = self.block_expression(false)?;
        let span_end = self.lexer.current().span();
        let node = SyntaxNode::WhileLoop(WhileLoop {
            condition,
            body,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn for_statement(&mut self) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::For)?.span();
        let (_, variable) = self.lexer.consume_ident()?;
        self.lexer.consume(TokenKind::In)?;
        let source = self.expression()?;
        let body = self.block_expression(false)?;
        let span_end = self.lexer.current().span();
        let node = SyntaxNode::ForLoop(ForLoop {
            variable,
            source,
            body,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn control_flow(&mut self) -> SyntaxNodeResult {
        let token = self.lexer.next()?;

        let node = match token.kind {
            TokenKind::Break => SyntaxNode::Break(Break { span: token.span() }),
            TokenKind::Continue => SyntaxNode::Continue(Continue { span: token.span() }),
            TokenKind::Return => {
                let result = if self.lexer.peek()?.kind != TokenKind::Semicolon {
                    Some(self.expression()?)
                } else {
                    None
                };
                let span_end = self.lexer.current().span();

                SyntaxNode::Return(Return {
                    result,
                    span: token.span() + span_end,
                })
            }
            _ => return Err(token.into()),
        };

        Ok(self.arena.alloc(node))
    }

    fn block_expression(&mut self, _: bool) -> SyntaxNodeResult {
        self.lexer.consume(TokenKind::LeftCurly)?;
        let expressions = self.expression_sequence()?;
        self.lexer.consume(TokenKind::RightCurly)?;

        Ok(expressions)
    }

    fn variable(&mut self, can_assign: bool) -> SyntaxNodeResult {
        let next_token = self.lexer.next()?;
        let span_start = next_token.span();
        let lhs_expression = match next_token.kind {
            TokenKind::Identifier(name) => self
                .arena
                .alloc(SyntaxNode::LitIdent(LitIdent { name, span: span_start })),
            _ => return Err(next_token.into()),
        };

        self.parse_assignment(lhs_expression, can_assign, span_start)
    }

    fn import_decl(&mut self) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::Import)?.span();
        let next_token = self.lexer.peek()?;
        let module_import = if next_token.kind == TokenKind::Star {
            self.lexer.consume(TokenKind::Star)?;
            self.lexer.consume(TokenKind::As)?;
            let (_, module_import) = self.lexer.consume_ident()?;

            if self.lexer.peek()?.kind == TokenKind::Comma {
                self.lexer.consume(TokenKind::Comma)?;
            }

            Some(module_import)
        } else {
            None
        };

        let item_imports = if self.lexer.peek()?.kind == TokenKind::LeftCurly {
            self.parse_fields(TokenKind::LeftCurly, TokenKind::RightCurly)?
        } else {
            Vec::new()
        };

        if module_import.is_none() && item_imports.is_empty() {
            todo!("Imports are required error.");
        }

        self.lexer.consume(TokenKind::From)?;

        let (token, relative_path) = self.lexer.consume_string()?;
        let span_end = token.span();
        let node = SyntaxNode::ImportDecl(ImportDecl {
            module_import,
            item_imports,
            relative_path,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn export_decl(&mut self) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::Export)?.span();

        let node = match self.lexer.peek()?.kind {
            TokenKind::Let => self.var_decl()?,
            TokenKind::Function => self.fn_decl()?,
            TokenKind::Class => self.class_decl()?,
            TokenKind::Identifier(name) => self.arena.alloc(SyntaxNode::LitIdent(LitIdent {
                name,
                span: self.lexer.peek()?.span(),
            })),
            _ => unreachable!("Unsupported export type encountered."),
        };

        let span_end = self.lexer.current().span();
        let node = SyntaxNode::ExportDecl(ExportDecl {
            export: node,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn var_decl(&mut self) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::Let)?.span();

        let is_mutable = if self.lexer.peek()?.kind == TokenKind::Mut {
            self.lexer.consume(TokenKind::Mut)?;
            true
        } else {
            false
        };

        let next_token = self.lexer.peek()?;
        let kind = match next_token.kind {
            TokenKind::Identifier(name) => {
                self.lexer.next()?;
                VarDeclKind::Singular(name)
            }
            TokenKind::LeftCurly => {
                VarDeclKind::Destructured(self.parse_fields(TokenKind::LeftCurly, TokenKind::RightCurly)?)
            }
            _ => return Err(next_token.into()),
        };

        self.lexer.consume(TokenKind::Assign)?;
        let expr = self.expression()?;
        let span_end = self.lexer.current().span();
        let node = SyntaxNode::VarDecl(VarDecl {
            kind,
            is_mutable,
            expr,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn anonymous_fn(&mut self, _: bool) -> SyntaxNodeResult {
        let span_start = self.lexer.peek()?.span();
        let args = self.parse_args(TokenKind::Pipe, TokenKind::Pipe)?;

        if args.len() > (u8::MAX as usize) {
            return Err(SyntaxError::AnonymousFnTooManyArguments(
                span_start + self.lexer.current().span(),
            ));
        }

        let body = self.expression()?;
        let span_end = self.lexer.current().span();
        let node = SyntaxNode::LitAnonymousFn(LitAnonymousFn {
            args,
            body,
            // TODO: Parse return type annotations for anonymous functions.
            return_: None,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn fn_decl(&mut self) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::Function)?.span();
        let name_token = self.lexer.next()?;
        let name = match name_token.kind {
            TokenKind::Identifier(ref name) => name.clone(),
            _ => return Err(name_token.into()),
        };
        let args = self.parse_args(TokenKind::LeftParen, TokenKind::RightParen)?;

        if args.len() > (u8::MAX as usize) {
            return Err(SyntaxError::FnTooManyArguments(name, name_token.span()));
        }

        let return_ = self.parse_return()?;
        let body = self.block_expression(false)?;
        let span_end = self.lexer.current().span();
        let node = SyntaxNode::FnDecl(FnDecl {
            name,
            args,
            body,
            return_,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn trait_fn_decl(&mut self) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::Function)?.span();
        let name_token = self.lexer.next()?;
        let name = match name_token.kind {
            TokenKind::Identifier(ref name) => name.clone(),
            _ => return Err(name_token.into()),
        };
        let args = self.parse_args(TokenKind::LeftParen, TokenKind::RightParen)?;

        if args.len() > (u8::MAX as usize) {
            return Err(SyntaxError::FnTooManyArguments(name, name_token.span()));
        }

        let return_ = self.parse_return()?;
        let next_token = self.lexer.peek()?;

        let node = if next_token.kind == TokenKind::LeftCurly {
            let body = self.block_expression(false)?;
            let span_end = self.lexer.current().span();
            SyntaxNode::FnDecl(FnDecl {
                name,
                args,
                body,
                return_,
                span: span_start + span_end,
            })
        } else {
            self.lexer.consume(TokenKind::Semicolon)?;
            let span_end = self.lexer.current().span();
            SyntaxNode::AbstractFnDecl(AbstractFnDecl {
                name,
                args,
                return_,
                span: span_start + span_end,
            })
        };

        Ok(self.arena.alloc(node))
    }

    fn op_decl(&mut self) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::Operator)?.span();
        let operator_token = self.lexer.next()?;
        let mut operator = match operator_token.kind {
            TokenKind::DiceRoll => OverloadedOperator::DiceRoll,
            TokenKind::Star => OverloadedOperator::Multiply,
            TokenKind::Slash => OverloadedOperator::Divide,
            TokenKind::Remainder => OverloadedOperator::Remainder,
            TokenKind::Plus => OverloadedOperator::Add,
            TokenKind::Minus => OverloadedOperator::Subtract,
            TokenKind::Greater => OverloadedOperator::GreaterThan,
            TokenKind::GreaterEqual => OverloadedOperator::GreaterThanEquals,
            TokenKind::Less => OverloadedOperator::LessThan,
            TokenKind::LessEqual => OverloadedOperator::LessThanEquals,
            TokenKind::Equal => OverloadedOperator::Equals,
            TokenKind::NotEqual => OverloadedOperator::NotEquals,
            TokenKind::RangeExclusive => OverloadedOperator::RangeExclusive,
            TokenKind::RangeInclusive => OverloadedOperator::RangeInclusive,
            _ => return Err(operator_token.into()),
        };
        let args = self.parse_args(TokenKind::LeftParen, TokenKind::RightParen)?;

        // NOTE: If the operator is a dice roll and only has one argument, reassign to DieRoll operator.
        // Otherwise enforce that the operator has two arguments.
        // TODO: Include other unary prefix and postfix operators.
        if operator == OverloadedOperator::DiceRoll && args.len() == 1 {
            operator = OverloadedOperator::DieRoll
        } else if args.len() != 2 {
            return Err(SyntaxError::FnTooManyArguments(
                operator.to_string(),
                operator_token.span(),
            ));
        }

        let return_ = self.parse_return()?;
        let body = self.block_expression(false)?;
        let span_end = self.lexer.current().span();
        let node = SyntaxNode::OpDecl(OpDecl {
            operator,
            args,
            body,
            return_,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn parse_return(&mut self) -> Result<Option<TypeAnnotation>, SyntaxError> {
        if self.lexer.peek()?.kind == TokenKind::Arrow {
            self.lexer.consume(TokenKind::Arrow)?;
            let (token, name) = self.lexer.consume_ident()?;
            let is_nullable = if self.lexer.peek()?.kind == TokenKind::QuestionMark {
                self.lexer.consume(TokenKind::QuestionMark)?;
                true
            } else {
                false
            };
            let name = LitIdent {
                name,
                span: token.span(),
            };

            Ok(Some(TypeAnnotation { name, is_nullable }))
        } else {
            Ok(None)
        }
    }

    fn parse_args(
        &mut self,
        open_token_kind: TokenKind,
        close_token_kind: TokenKind,
    ) -> Result<Vec<FnArg>, SyntaxError> {
        self.lexer.consume(open_token_kind)?;

        let mut args = Vec::new();

        while self.lexer.peek()?.kind != close_token_kind {
            let (token, name) = self.lexer.consume_ident()?;
            let span = token.span();

            let type_ = if self.lexer.peek()?.kind == TokenKind::Colon {
                self.lexer.consume(TokenKind::Colon)?;
                let (_, name) = self.lexer.consume_ident()?;
                let is_nullable = if self.lexer.peek()?.kind == TokenKind::QuestionMark {
                    self.lexer.consume(TokenKind::QuestionMark)?;
                    true
                } else {
                    false
                };
                let name = LitIdent {
                    name,
                    span: token.span(),
                };

                Some(TypeAnnotation { name, is_nullable })
            } else {
                None
            };

            args.push(FnArg {
                name,
                type_,
                span: span + self.lexer.current().span(),
            });

            if self.lexer.peek()?.kind == TokenKind::Comma {
                self.lexer.next()?;
            } else if self.lexer.peek()?.kind != close_token_kind {
                return Err(self.lexer.next()?.into());
            }
        }

        self.lexer.consume(close_token_kind)?;
        Ok(args)
    }

    fn parse_fields(
        &mut self,
        open_token_kind: TokenKind,
        close_token_kind: TokenKind,
    ) -> Result<Vec<String>, SyntaxError> {
        self.lexer.consume(open_token_kind)?;

        let mut args = Vec::new();

        while self.lexer.peek()?.kind != close_token_kind {
            let (_, name) = self.lexer.consume_ident()?;
            args.push(name);

            if self.lexer.peek()?.kind == TokenKind::Comma {
                self.lexer.next()?;
            } else if self.lexer.peek()?.kind != close_token_kind {
                return Err(self.lexer.next()?.into());
            }
        }

        self.lexer.consume(close_token_kind)?;
        Ok(args)
    }

    fn class_decl(&mut self) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::Class)?.span();
        let (_, name) = self.lexer.consume_ident()?;
        self.lexer.consume(TokenKind::LeftCurly)?;

        let mut next_token = self.lexer.peek()?;
        let mut associated_items = Vec::new();

        while !matches!(next_token.kind, TokenKind::RightCurly) {
            let expression = match next_token.kind {
                TokenKind::Function => self.fn_decl()?,
                TokenKind::Operator => self.op_decl()?,
                _ => return Err(SyntaxError::UnexpectedToken(next_token.to_string(), next_token.span())),
            };

            associated_items.push(expression);

            next_token = self.lexer.peek()?;

            if matches!(next_token.kind, TokenKind::RightCurly) {
                break;
            }
        }

        let span_end = self.lexer.consume(TokenKind::RightCurly)?.span();
        let class_decl = ClassDecl {
            name,
            associated_items,
            span: span_start + span_end,
        };

        let node = self.arena.alloc(SyntaxNode::ClassDecl(class_decl));

        Ok(node)
    }

    fn trait_decl(&mut self) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::Trait)?.span();
        let (_, name) = self.lexer.consume_ident()?;
        self.lexer.consume(TokenKind::LeftCurly)?;

        let mut next_token = self.lexer.peek()?;
        let mut associated_items = Vec::new();

        while !matches!(next_token.kind, TokenKind::RightCurly) {
            let expression = match next_token.kind {
                TokenKind::Function => self.trait_fn_decl()?,
                _ => return Err(SyntaxError::UnexpectedToken(next_token.to_string(), next_token.span())),
            };

            associated_items.push(expression);

            next_token = self.lexer.peek()?;

            if matches!(next_token.kind, TokenKind::RightCurly) {
                break;
            }
        }

        let span_end = self.lexer.consume(TokenKind::RightCurly)?.span();
        let trait_decl = TraitDecl {
            name,
            associated_items,
            span: span_start + span_end,
        };

        let node = self.arena.alloc(SyntaxNode::TraitDecl(trait_decl));

        Ok(node)
    }

    fn trait_impl(&mut self) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::Impl)?.span();
        let (_, trait_) = self.lexer.consume_ident()?;
        self.lexer.consume(TokenKind::For)?;
        let (_, target) = self.lexer.consume_ident()?;
        let name = if self.lexer.peek()?.kind == TokenKind::As {
            self.lexer.consume(TokenKind::As)?;
            let (_, name) = self.lexer.consume_ident()?;
            Some(name)
        } else {
            None
        };
        self.lexer.consume(TokenKind::LeftCurly)?;

        let mut next_token = self.lexer.peek()?;
        let mut associated_items = Vec::new();

        while !matches!(next_token.kind, TokenKind::RightCurly) {
            let expression = match next_token.kind {
                TokenKind::Function => self.fn_decl()?,
                _ => return Err(SyntaxError::UnexpectedToken(next_token.to_string(), next_token.span())),
            };

            associated_items.push(expression);
            next_token = self.lexer.peek()?;

            if matches!(next_token.kind, TokenKind::RightCurly) {
                break;
            }
        }

        let span_end = self.lexer.consume(TokenKind::RightCurly)?.span();
        let trait_impl = TraitImpl {
            trait_,
            target,
            name,
            associated_items,
            span: span_start + span_end,
        };

        Ok(self.arena.alloc(SyntaxNode::TraitImpl(trait_impl)))
    }

    fn binary(&mut self, lhs: SyntaxNodeId, _: bool, span_start: Span) -> SyntaxNodeResult {
        let token = self.lexer.next()?;
        let rule = ParserRule::for_token(&token)?;
        let operator = match token.kind {
            TokenKind::Is => BinaryOperator::Is,
            TokenKind::Pipeline => BinaryOperator::Pipeline,
            TokenKind::Coalesce => BinaryOperator::Coalesce,
            TokenKind::RangeExclusive => BinaryOperator::RangeExclusive,
            TokenKind::RangeInclusive => BinaryOperator::RangeInclusive,
            TokenKind::LazyAnd => BinaryOperator::LogicalAnd,
            TokenKind::Pipe => {
                self.lexer.consume(TokenKind::Pipe)?;
                BinaryOperator::LogicalOr
            }
            TokenKind::Equal => BinaryOperator::Equals,
            TokenKind::NotEqual => BinaryOperator::NotEquals,
            TokenKind::Greater => BinaryOperator::GreaterThan,
            TokenKind::GreaterEqual => BinaryOperator::GreaterThanEquals,
            TokenKind::Less => BinaryOperator::LessThan,
            TokenKind::LessEqual => BinaryOperator::LessThanEquals,
            TokenKind::Plus => BinaryOperator::Add,
            TokenKind::Minus => BinaryOperator::Subtract,
            TokenKind::Star => BinaryOperator::Multiply,
            TokenKind::Slash => BinaryOperator::Divide,
            TokenKind::Remainder => BinaryOperator::Remainder,
            TokenKind::DiceRoll => BinaryOperator::DiceRoll,
            _ => unreachable!(),
        };
        let rhs = self.parse_precedence(rule.infix_precedence.increment())?;
        let node = SyntaxNode::Binary(Binary {
            operator,
            lhs_expression: lhs,
            rhs_expression: rhs,
            span: span_start + token.span(),
        });

        Ok(self.arena.alloc(node))
    }

    fn unary(&mut self, _: bool) -> SyntaxNodeResult {
        let token = self.lexer.next()?;
        let child_node_id = self.parse_precedence(RulePrecedence::Unary)?;
        let operator = match token.kind {
            TokenKind::Minus => UnaryOperator::Negate,
            TokenKind::Not => UnaryOperator::Not,
            TokenKind::DiceRoll => UnaryOperator::DiceRoll,
            _ => unreachable!(),
        };
        let node = SyntaxNode::Unary(Unary {
            operator,
            expression: child_node_id,
            span: token.span(),
        });

        Ok(self.arena.alloc(node))
    }

    fn null_propagate(&mut self, expression: SyntaxNodeId, _: bool, span_start: Span) -> SyntaxNodeResult {
        let span_end = self.lexer.consume(TokenKind::QuestionMark)?.span();
        let node = SyntaxNode::NullPropagate(NullPropagate {
            expression,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn error_propagate(&mut self, expression: SyntaxNodeId, _: bool, span_start: Span) -> SyntaxNodeResult {
        let span_end = self.lexer.consume(TokenKind::ErrorPropagate)?.span();
        let node = SyntaxNode::ErrorPropagate(ErrorPropagate {
            expression,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn index_access(&mut self, expression: SyntaxNodeId, can_assign: bool, span_start: Span) -> SyntaxNodeResult {
        self.lexer.consume(TokenKind::LeftSquare)?;
        let index_expression = self.expression()?;
        self.lexer.consume(TokenKind::RightSquare)?;

        let span_end = self.lexer.current().span();
        let node = SyntaxNode::Index(Index {
            expression,
            index_expression,
            span: span_start + span_end,
        });
        let lhs_expression = self.arena.alloc(node);

        self.parse_assignment(lhs_expression, can_assign, span_start)
    }

    fn field_access(&mut self, lhs: SyntaxNodeId, can_assign: bool, span_start: Span) -> SyntaxNodeResult {
        self.lexer.consume(TokenKind::Dot)?;

        let (_, field) = self.lexer.consume_ident()?;
        let span_end = self.lexer.current().span();
        let node = SyntaxNode::FieldAccess(FieldAccess {
            expression: lhs,
            field,
            span: span_start + span_end,
        });
        let lhs_expression = self.arena.alloc(node);

        self.parse_assignment(lhs_expression, can_assign, span_start)
    }

    fn grouping(&mut self, _: bool) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::LeftParen)?.span();

        if self.lexer.peek()?.kind == TokenKind::RightParen {
            let span_end = self.lexer.consume(TokenKind::RightParen)?.span();

            let node = SyntaxNode::LitUnit(LitUnit {
                span: span_start + span_end,
            });
            Ok(self.arena.alloc(node))
        } else {
            let expression = self.expression()?;
            self.lexer.consume(TokenKind::RightParen)?;

            Ok(expression)
        }
    }

    fn fn_call(&mut self, lhs: SyntaxNodeId, _: bool, span_start: Span) -> SyntaxNodeResult {
        self.lexer.consume(TokenKind::LeftParen)?;

        let mut args = Vec::new();

        while self.lexer.peek()?.kind != TokenKind::RightParen {
            let value = self.parse_precedence(RulePrecedence::Assignment)?;
            args.push(value);

            if self.lexer.peek()?.kind == TokenKind::Comma {
                self.lexer.next()?;
            } else if self.lexer.peek()?.kind != TokenKind::RightParen {
                return Err(self.lexer.next()?.into());
            }
        }

        let span_end = self.lexer.consume(TokenKind::RightParen)?.span();
        let node = SyntaxNode::FnCall(FnCall {
            target: lhs,
            args,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn object(&mut self, _: bool) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::Object)?.span();
        self.lexer.consume(TokenKind::LeftCurly)?;

        let mut properties = Vec::new();

        while self.lexer.peek()?.kind != TokenKind::RightCurly {
            let key = match self.lexer.next()?.kind {
                TokenKind::String(value) => value,
                TokenKind::Integer(value) => value.to_string(),
                TokenKind::Identifier(value) => value,
                _ => return Err(self.lexer.current().clone().into()),
            };

            self.lexer.consume(TokenKind::Colon)?;
            let value = self.parse_precedence(RulePrecedence::Assignment)?;

            if self.lexer.peek()?.kind == TokenKind::Comma {
                self.lexer.next()?;
            } else if self.lexer.peek()?.kind != TokenKind::RightCurly {
                return Err(self.lexer.next()?.into());
            }

            properties.push((key, value));
        }

        let span_end = self.lexer.consume(TokenKind::RightCurly)?.span();

        let node = self.arena.alloc(SyntaxNode::LitObject(LitObject {
            items: properties,
            span: span_start + span_end,
        }));

        Ok(node)
    }

    fn list(&mut self, _: bool) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::LeftSquare)?.span();

        let mut values = Vec::new();

        while self.lexer.peek()?.kind != TokenKind::RightSquare {
            let value = self.parse_precedence(RulePrecedence::Assignment)?;

            if self.lexer.peek()?.kind == TokenKind::Comma {
                self.lexer.next()?;
            } else if self.lexer.peek()?.kind != TokenKind::RightSquare {
                return Err(self.lexer.next()?.into());
            }

            values.push(value);
        }

        let span_end = self.lexer.consume(TokenKind::RightSquare)?.span();

        let node = self.arena.alloc(SyntaxNode::LitList(LitList {
            items: values,
            span: span_start + span_end,
        }));

        Ok(node)
    }

    fn literal(&mut self, _: bool) -> SyntaxNodeResult {
        let token = self.lexer.next()?;
        let span = token.span();
        let literal = match token.kind {
            TokenKind::Integer(value) => SyntaxNode::LitInt(LitInt { value, span }),
            TokenKind::Float(value) => SyntaxNode::LitFloat(LitFloat { value, span }),
            TokenKind::String(value) => SyntaxNode::LitString(LitString { value, span }),
            TokenKind::False => SyntaxNode::LitBool(LitBool { value: false, span }),
            TokenKind::True => SyntaxNode::LitBool(LitBool { value: true, span }),
            TokenKind::Null => SyntaxNode::LitNull(LitNull { span }),
            _ => return Err(token.into()),
        };

        Ok(self.arena.alloc(literal))
    }

    fn parse_assignment(
        &mut self,
        lhs_expression: SyntaxNodeId,
        can_assign: bool,
        span_start: Span,
    ) -> SyntaxNodeResult {
        let next_token_kind = self.lexer.peek()?.kind;
        let is_assignment = matches!(
            next_token_kind,
            TokenKind::Assign
                | TokenKind::MulAssign
                | TokenKind::DivAssign
                | TokenKind::AddAssign
                | TokenKind::SubAssign
        );

        if can_assign && is_assignment {
            let kind = self
                .lexer
                .consume_one_of(&[
                    TokenKind::Assign,
                    TokenKind::MulAssign,
                    TokenKind::DivAssign,
                    TokenKind::AddAssign,
                    TokenKind::SubAssign,
                ])?
                .kind;

            let rhs_expression = self.expression()?;
            let span_end = self.lexer.current().span();
            let operator = match kind {
                TokenKind::Assign => AssignmentOperator::Assignment,
                TokenKind::MulAssign => AssignmentOperator::MulAssignment,
                TokenKind::DivAssign => AssignmentOperator::DivAssignment,
                TokenKind::AddAssign => AssignmentOperator::AddAssignment,
                TokenKind::SubAssign => AssignmentOperator::SubAssignment,
                kind => unreachable!("Unexpected token {:?} encountered.", kind),
            };

            Ok(self.arena.alloc(SyntaxNode::Assignment(Assignment {
                operator,
                lhs_expression,
                rhs_expression,
                span: span_start + span_end,
            })))
        } else {
            Ok(lhs_expression)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use crate::{Binary, BinaryOperator, Block, LitInt, SyntaxNode, Unary, UnaryOperator};
    use dice_error::syntax_error::SyntaxError;

    #[test]
    fn test_parse_integer() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("5").parse()?;
        let root = syntax_tree.get(syntax_tree.root());

        if let SyntaxNode::Block(Block {
            trailing_expression: Some(block),
            ..
        }) = root
        {
            let node = syntax_tree.get(*block);

            assert!(matches!(node, SyntaxNode::LitInt(LitInt { value: 5, .. })));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_unary_minus() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("-5").parse()?;
        let root = syntax_tree.get(syntax_tree.root());

        if let SyntaxNode::Block(Block {
            trailing_expression: Some(block),
            ..
        }) = root
        {
            let node = syntax_tree.get(*block);

            assert!(matches!(
                node,
                SyntaxNode::Unary(Unary {
                    operator: UnaryOperator::Negate,
                    ..
                })
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_binary_minus() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("5 - 5").parse()?;
        let root = syntax_tree.get(syntax_tree.root());

        if let SyntaxNode::Block(Block {
            trailing_expression: Some(block),
            ..
        }) = root
        {
            let node = syntax_tree.get(*block);

            assert!(matches!(
                node,
                SyntaxNode::Binary(Binary {
                    operator: BinaryOperator::Subtract,
                    ..
                })
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_binary_minus_with_unary_minus() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("5 - -5").parse()?;
        let root = syntax_tree.get(syntax_tree.root());

        if let SyntaxNode::Block(Block {
            trailing_expression: Some(block),
            ..
        }) = root
        {
            let node = syntax_tree.get(*block);

            assert!(matches!(
                node,
                SyntaxNode::Binary(Binary {
                    operator: BinaryOperator::Subtract,
                    ..
                })
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_binary_precedence_multiply_right() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("5 - 5 * 5").parse()?;
        let root = syntax_tree.get(syntax_tree.root());

        if let SyntaxNode::Block(Block {
            trailing_expression: Some(block),
            ..
        }) = root
        {
            let node = syntax_tree.get(*block);

            assert!(matches!(
                node,
                SyntaxNode::Binary(Binary {
                    operator: BinaryOperator::Subtract,
                    ..
                })
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_binary_precedence_multiply_left() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("5 * 5 - 5").parse()?;
        let root = syntax_tree.get(syntax_tree.root());

        if let SyntaxNode::Block(Block {
            trailing_expression: Some(block),
            ..
        }) = root
        {
            let node = syntax_tree.get(*block);

            assert!(matches!(
                node,
                SyntaxNode::Binary(Binary {
                    operator: BinaryOperator::Subtract,
                    ..
                })
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_grouping() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("5 * (5 - 5)").parse()?;
        let root = syntax_tree.get(syntax_tree.root());

        if let SyntaxNode::Block(Block {
            trailing_expression: Some(block),
            ..
        }) = root
        {
            let node = syntax_tree.get(*block);

            assert!(matches!(
                node,
                SyntaxNode::Binary(Binary {
                    operator: BinaryOperator::Multiply,
                    ..
                })
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_unary_die() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("d8").parse()?;
        let root = syntax_tree.get(syntax_tree.root());

        if let SyntaxNode::Block(Block {
            trailing_expression: Some(block),
            ..
        }) = root
        {
            let node = syntax_tree.get(*block);

            assert!(matches!(
                node,
                SyntaxNode::Unary(Unary {
                    operator: UnaryOperator::DiceRoll,
                    ..
                })
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_binary_dice() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("6d8").parse()?;
        let root = syntax_tree.get(syntax_tree.root());

        if let SyntaxNode::Block(Block {
            trailing_expression: Some(block),
            ..
        }) = root
        {
            let node = syntax_tree.get(*block);

            assert!(matches!(
                node,
                SyntaxNode::Binary(Binary {
                    operator: BinaryOperator::DiceRoll,
                    ..
                })
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_object_expression() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("#{ x: 50, y: 30 }").parse()?;
        let root = syntax_tree.get(syntax_tree.root());

        if let SyntaxNode::Block(Block {
            trailing_expression: Some(block),
            ..
        }) = root
        {
            let node = syntax_tree.get(*block);

            assert!(matches!(node, SyntaxNode::LitObject(_)));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_list_expression() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("[x, y, 1, 1*2, #{}]").parse()?;
        let root = syntax_tree.get(syntax_tree.root());

        if let SyntaxNode::Block(Block {
            trailing_expression: Some(block),
            ..
        }) = root
        {
            let node = syntax_tree.get(*block);

            assert!(matches!(node, SyntaxNode::LitList(_)));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }
}
