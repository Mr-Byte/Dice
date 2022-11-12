mod rules;

use std::num::{ParseFloatError, ParseIntError};

use super::{
    lexer::{Lexer, TokenKind},
    Assignment, AssignmentOperator, Binary, BinaryOperator, Block, Break, Continue, ExportDecl, FnCall, FnDecl,
    IfExpression, LitAnonymousFn, LitBool, LitFloat, LitIdent, LitInt, LitList, LitNull, LitObject, LitString, LitUnit,
    Prefix, Return, SyntaxNode, SyntaxNodeId, SyntaxTree, UnaryOperator, VarDecl, WhileLoop,
};
use crate::{
    lexer::Token,
    parser::rules::{ParseResult, ParserRules, Precedence},
    ClassDecl, ErrorPropagate, FieldAccess, FnArg, ForLoop, ImportDecl, Index, Is, Loop, NullPropagate, OpDecl,
    OverloadedOperator, SuperAccess, SuperCall, TypeAnnotation, VarDeclKind,
};
use dice_core::{
    error::{
        codes::{
            FUNCTION_HAS_TOO_MANY_ARGUMENTS, INVALID_FLOAT_VALUE, INVALID_IMPORT_USAGE, INVALID_INTEGER_VALUE,
            UNEXPECTED_TOKEN,
        },
        context::{
            Context, ContextKind, IMPORT_REQUIRES_ITEMS_TO_BE_IMPORTED, IMPORT_REQUIRES_ITEMS_TO_BE_IMPORTED_HELP,
        },
        Error, ResultExt,
    },
    protocol::{
        error::{IS_OK, RESULT},
        ProtocolSymbol,
    },
    source::Source,
    span::Span,
    tags,
};
use id_arena::Arena;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    arena: Arena<SyntaxNode>,
    rules: ParserRules<'a>,
    source: &'a Source,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a Source) -> Self {
        let lexer = Lexer::from_source(source);
        let arena = Arena::new();

        Self {
            lexer,
            arena,
            rules: ParserRules::new(),
            source,
        }
    }

    pub fn parse(mut self) -> Result<SyntaxTree, Error> {
        let root = self.expression_sequence()?;

        Ok(SyntaxTree::new(root, self.arena))
    }

    fn expression_sequence(&mut self) -> ParseResult {
        let mut expressions = Vec::new();
        let mut next_token = self.lexer.peek()?;
        let span_start = next_token.span;
        let mut trailing_expression = None;

        while !matches!(next_token.kind, TokenKind::EndOfInput | TokenKind::RightCurly) {
            let expression = match next_token.kind {
                TokenKind::Loop => self.loop_statement()?,
                TokenKind::While => self.while_statement()?,
                TokenKind::For => self.for_statement()?,
                TokenKind::Let => self.var_decl()?,
                TokenKind::Function => self.fn_decl()?,
                TokenKind::Operator => self.op_decl()?,
                TokenKind::Class => self.class_decl()?,
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

            expressions.push(expression);
        }

        let span_end = next_token.span;
        let node = SyntaxNode::Block(Block {
            expressions,
            trailing_expression,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn expression(&mut self) -> ParseResult {
        self.parse_precedence(Precedence::Assignment)
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> ParseResult {
        let next_token = self.lexer.peek()?;
        let rule = self.rules.for_token(&next_token).with_source(|| self.source.clone())?;
        // TODO: Handle prefix precedence.
        let mut node = rule
            .prefix
            .map(|(prefix, _)| prefix(self, precedence <= Precedence::Assignment))
            .unwrap_or_else({
                let next_token = next_token.clone();
                || self.unexpected_token(next_token.kind, self.rules.prefix_tokens(), next_token.span)
            })?;

        loop {
            let span_start = next_token.span;
            let next_token = self.lexer.peek()?;
            let rule = self.rules.for_token(&next_token).with_source(|| self.source.clone())?;

            if let Some((postfix, postfix_precedence)) = rule.postfix {
                if precedence > postfix_precedence {
                    break;
                }

                node = postfix(self, node, precedence <= Precedence::Assignment, span_start)?;

                continue;
            }

            if let Some((infix, infix_precedence)) = rule.infix {
                if precedence > infix_precedence {
                    break;
                }

                node = infix(self, node, precedence <= Precedence::Assignment, span_start)?;
            } else {
                break;
            }
        }

        Ok(node)
    }

    fn if_expression(&mut self, _: bool) -> ParseResult {
        let span_start = self.lexer.consume(TokenKind::If)?.span;
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
        let span_end = self.lexer.current().span;
        let node = SyntaxNode::IfExpression(IfExpression {
            condition,
            primary,
            secondary,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn loop_statement(&mut self) -> ParseResult {
        let span_start = self.lexer.consume(TokenKind::Loop)?.span;
        let body = self.block_expression(false)?;
        let span_end = self.lexer.current().span;
        let node = SyntaxNode::Loop(Loop {
            body,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn while_statement(&mut self) -> ParseResult {
        let span_start = self.lexer.consume(TokenKind::While)?.span;
        let condition = self.expression()?;
        let body = self.block_expression(false)?;
        let span_end = self.lexer.current().span;
        let node = SyntaxNode::WhileLoop(WhileLoop {
            condition,
            body,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn for_statement(&mut self) -> ParseResult {
        let span_start = self.lexer.consume(TokenKind::For)?.span;
        let (_, variable) = self.lexer.consume_ident()?;
        self.lexer.consume(TokenKind::In)?;
        let source = self.expression()?;
        let body = self.block_expression(false)?;
        let span_end = self.lexer.current().span;
        let node = SyntaxNode::ForLoop(ForLoop {
            variable,
            source,
            body,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn control_flow(&mut self) -> ParseResult {
        let token = self.lexer.next()?.clone();

        let node = match token.kind {
            TokenKind::Break => SyntaxNode::Break(Break { span: token.span }),
            TokenKind::Continue => SyntaxNode::Continue(Continue { span: token.span }),
            TokenKind::Return => {
                let result = if self.lexer.peek()?.kind != TokenKind::RightCurly {
                    Some(self.expression()?)
                } else {
                    None
                };
                let span_end = self.lexer.current().span;

                SyntaxNode::Return(Return {
                    result,
                    span: token.span + span_end,
                })
            }
            kind => self.unexpected_token(
                kind,
                &[TokenKind::Break, TokenKind::Continue, TokenKind::Return],
                token.span,
            )?,
        };

        Ok(self.arena.alloc(node))
    }

    fn block_expression(&mut self, _: bool) -> ParseResult {
        self.lexer.consume(TokenKind::LeftCurly)?;
        let expressions = self.expression_sequence()?;
        self.lexer.consume(TokenKind::RightCurly)?;

        Ok(expressions)
    }

    fn variable(&mut self, can_assign: bool) -> ParseResult {
        let next_token = self.lexer.next()?;
        let span_start = next_token.span;
        let lhs_expression = match next_token.kind.clone() {
            TokenKind::Identifier => self
                .arena
                .alloc(SyntaxNode::LitIdent(LitIdent::synthesize(next_token.slice, span_start))),
            kind => self.unexpected_token(kind, &[TokenKind::Identifier], span_start)?,
        };

        self.parse_assignment(lhs_expression, can_assign, span_start)
    }

    fn import_decl(&mut self) -> ParseResult {
        let span_start = self.lexer.consume(TokenKind::Import)?.span;
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
            return Err(Error::new(INVALID_IMPORT_USAGE)
                .with_source(self.source.clone())
                .with_span(span_start)
                .push_context(Context::new(IMPORT_REQUIRES_ITEMS_TO_BE_IMPORTED, ContextKind::Note))
                .push_context(Context::new(
                    IMPORT_REQUIRES_ITEMS_TO_BE_IMPORTED_HELP,
                    ContextKind::Help,
                )));
        }

        self.lexer.consume(TokenKind::From)?;

        let (token, relative_path) = self.lexer.consume_string()?;
        let span_end = token.span;
        let node = SyntaxNode::ImportDecl(ImportDecl {
            module_import,
            item_imports,
            relative_path: process_string(relative_path, StringMode::Raw)
                .with_source(|| self.source.clone())
                .with_span(|| span_end)?,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn export_decl(&mut self) -> ParseResult {
        let span_start = self.lexer.consume(TokenKind::Export)?.span;

        let next = self.lexer.peek()?;
        let node = match next.kind {
            TokenKind::Let => self.var_decl()?,
            TokenKind::Function => self.fn_decl()?,
            TokenKind::Class => self.class_decl()?,
            TokenKind::Identifier => self
                .arena
                .alloc(SyntaxNode::LitIdent(LitIdent::synthesize(next.slice, next.span))),
            kind => self.unexpected_token(
                kind,
                &[
                    TokenKind::Let,
                    TokenKind::Function,
                    TokenKind::Class,
                    TokenKind::Identifier,
                ],
                next.span,
            )?,
        };

        let span_end = self.lexer.current().span;
        let node = SyntaxNode::ExportDecl(ExportDecl {
            export: node,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn var_decl(&mut self) -> ParseResult {
        let span_start = self.lexer.consume(TokenKind::Let)?.span;

        let is_mutable = if self.lexer.peek()?.kind == TokenKind::Mut {
            self.lexer.consume(TokenKind::Mut)?;
            true
        } else {
            false
        };

        let next_token = self.lexer.peek()?;
        let kind = match next_token.kind {
            TokenKind::Identifier => {
                self.lexer.next()?;
                VarDeclKind::Singular(next_token.slice.to_owned())
            }
            TokenKind::LeftCurly => {
                VarDeclKind::Destructured(self.parse_fields(TokenKind::LeftCurly, TokenKind::RightCurly)?)
            }
            kind => self.unexpected_token(kind, &[TokenKind::Identifier, TokenKind::LeftCurly], next_token.span)?,
        };

        self.lexer.consume(TokenKind::Assign)?;
        let expr = self.expression()?;
        let span_end = self.lexer.current().span;
        let node = SyntaxNode::VarDecl(VarDecl {
            kind,
            is_mutable,
            expr,
            type_: None,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn anonymous_fn(&mut self, _: bool) -> ParseResult {
        let span_start = self.lexer.peek()?.span;
        let args = self.parse_args(TokenKind::Pipe, TokenKind::Pipe)?;

        if args.len() > (u8::MAX as usize) {
            return Err(Error::new(FUNCTION_HAS_TOO_MANY_ARGUMENTS)
                .with_source(self.source.clone())
                .with_span(span_start + self.lexer.current().span));
        }

        let body = self.expression()?;
        let span_end = self.lexer.current().span;
        let node = SyntaxNode::LitAnonymousFn(LitAnonymousFn {
            args,
            body,
            // TODO: Parse return type annotations for anonymous functions.
            return_: None,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn fn_decl(&mut self) -> ParseResult {
        let span_start = self.lexer.consume(TokenKind::Function)?.span;
        let (name_token, name) = self.lexer.consume_ident()?;
        let name = LitIdent::synthesize(name, name_token.span);
        let args = self.parse_args(TokenKind::LeftParen, TokenKind::RightParen)?;

        if args.len() > (u8::MAX as usize) {
            return Err(Error::new(FUNCTION_HAS_TOO_MANY_ARGUMENTS)
                .with_source(self.source.clone())
                .with_span(span_start + self.lexer.current().span));
        }

        let return_ = self.parse_return()?;
        let body = self.block_expression(false)?;
        let span_end = self.lexer.current().span;
        let node = SyntaxNode::FnDecl(FnDecl {
            name,
            args,
            body,
            return_,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn op_decl(&mut self) -> ParseResult {
        let span_start = self.lexer.consume(TokenKind::Operator)?.span;
        let operator_token = self.lexer.next()?;
        let operator = match operator_token.kind.clone() {
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
            kind => self.unexpected_token(
                kind,
                &[
                    TokenKind::Star,
                    TokenKind::Slash,
                    TokenKind::Remainder,
                    TokenKind::Plus,
                    TokenKind::Minus,
                    TokenKind::Greater,
                    TokenKind::GreaterEqual,
                    TokenKind::Less,
                    TokenKind::LessEqual,
                    TokenKind::Equal,
                    TokenKind::NotEqual,
                    TokenKind::RangeExclusive,
                    TokenKind::RangeInclusive,
                ],
                span_start,
            )?,
        };
        let args = self.parse_args(TokenKind::LeftParen, TokenKind::RightParen)?;
        let return_ = self.parse_return()?;
        let body = self.block_expression(false)?;
        let span_end = self.lexer.current().span;
        let node = SyntaxNode::OpDecl(OpDecl {
            operator,
            args,
            body,
            return_,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn parse_return(&mut self) -> Result<Option<TypeAnnotation>, Error> {
        if self.lexer.peek()?.kind == TokenKind::Arrow {
            self.parse_type_annotation(TokenKind::Arrow).map(Some)
        } else {
            Ok(None)
        }
    }

    fn parse_args(&mut self, open_token_kind: TokenKind, close_token_kind: TokenKind) -> Result<Vec<FnArg>, Error> {
        self.lexer.consume(open_token_kind)?;

        let mut args = Vec::new();

        while self.lexer.peek()?.kind != close_token_kind {
            let (token, name) = self.lexer.consume_ident()?;
            let span = token.span;

            let type_ = if self.lexer.peek()?.kind == TokenKind::Colon {
                Some(self.parse_type_annotation(TokenKind::Colon)?)
            } else {
                None
            };

            args.push(FnArg {
                name,
                type_,
                span: span + self.lexer.current().span,
            });

            let next = self.lexer.peek()?;
            if next.kind == TokenKind::Comma {
                self.lexer.next()?;
            } else if next.kind != close_token_kind {
                self.unexpected_token(next.kind, &[close_token_kind.clone()], next.span)?;
            }
        }

        self.lexer.consume(close_token_kind)?;
        Ok(args)
    }

    fn parse_type_annotation(&mut self, delimiter: TokenKind) -> Result<TypeAnnotation, Error> {
        let span_start = self.lexer.consume(delimiter)?.span;
        let (name_token, name) = self.lexer.consume_ident()?;
        let ident_span = name_token.span;
        let is_nullable = if self.lexer.peek()?.kind == TokenKind::QuestionMark {
            self.lexer.consume(TokenKind::QuestionMark)?;
            true
        } else {
            false
        };
        let name = LitIdent {
            identifier: name,
            span: ident_span,
        };
        let span_end = self.lexer.current().span;

        Ok(TypeAnnotation {
            name,
            is_nullable,
            span: span_start + span_end,
        })
    }

    fn parse_fields(&mut self, open_token_kind: TokenKind, close_token_kind: TokenKind) -> Result<Vec<String>, Error> {
        self.lexer.consume(open_token_kind)?;

        let mut args = Vec::new();

        while self.lexer.peek()?.kind != close_token_kind {
            let (_, name) = self.lexer.consume_ident()?;
            args.push(name);

            let next = self.lexer.peek()?;
            if next.kind == TokenKind::Comma {
                self.lexer.next()?;
            } else if next.kind != close_token_kind {
                self.unexpected_token(next.kind, &[close_token_kind.clone()], next.span)?;
            }
        }

        self.lexer.consume(close_token_kind)?;
        Ok(args)
    }

    fn class_decl(&mut self) -> ParseResult {
        let span_start = self.lexer.consume(TokenKind::Class)?.span;
        let (name_token, name) = self.lexer.consume_ident()?;
        let name = LitIdent {
            identifier: name,
            span: name_token.span,
        };
        let base = if self.lexer.peek()?.kind == TokenKind::Colon {
            self.lexer.consume(TokenKind::Colon)?;
            Some(self.expression()?)
        } else {
            None
        };

        self.lexer.consume(TokenKind::LeftCurly)?;

        let mut next_token = self.lexer.peek()?;
        let mut associated_items = Vec::new();

        while !matches!(next_token.kind, TokenKind::RightCurly) {
            let expression = match next_token.kind {
                TokenKind::Function => self.fn_decl()?,
                TokenKind::Operator => self.op_decl()?,
                kind => self.unexpected_token(kind, &[TokenKind::Function, TokenKind::Operator], next_token.span)?,
            };

            associated_items.push(expression);

            next_token = self.lexer.peek()?;

            if matches!(next_token.kind, TokenKind::RightCurly) {
                break;
            }
        }

        let span_end = self.lexer.consume(TokenKind::RightCurly)?.span;
        let class_decl = ClassDecl {
            name,
            associated_items,
            base,
            span: span_start + span_end,
        };

        let node = self.arena.alloc(SyntaxNode::ClassDecl(class_decl));

        Ok(node)
    }

    fn binary_operator(&mut self, lhs: SyntaxNodeId, _: bool, span_start: Span) -> ParseResult {
        let token = self.lexer.next()?.clone();
        let operator = match token.kind {
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
            _ => unreachable!(),
        };

        let rule = self.rules.for_token(&token).with_source(|| self.source.clone())?;
        let precedence = rule.infix.expect("Invalid infix rule.").1.increment();
        let rhs = self.parse_precedence(precedence)?;

        let node = SyntaxNode::Binary(Binary {
            operator,
            lhs_expression: lhs,
            rhs_expression: rhs,
            span: span_start + token.span,
        });

        Ok(self.arena.alloc(node))
    }

    fn is_operator(&mut self, lhs: SyntaxNodeId, _: bool, span_start: Span) -> ParseResult {
        let type_annotation = self.parse_type_annotation(TokenKind::Is)?;
        let node = SyntaxNode::Is(Is {
            value: lhs,
            type_: type_annotation,
            span: span_start + self.lexer.current().span,
        });

        Ok(self.arena.alloc(node))
    }

    fn prefix_operator(&mut self, _: bool) -> ParseResult {
        let token = self.lexer.next()?.clone();
        let child_node_id = self.parse_precedence(Precedence::Unary)?;
        let operator = match token.kind {
            TokenKind::Minus => UnaryOperator::Negate,
            TokenKind::Not => UnaryOperator::Not,
            _ => unreachable!(),
        };
        let node = SyntaxNode::Prefix(Prefix {
            operator,
            expression: child_node_id,
            span: token.span,
        });

        Ok(self.arena.alloc(node))
    }

    // TODO: Combine postfix operators into a single parser?
    fn null_propagate(&mut self, expression: SyntaxNodeId, _: bool, span_start: Span) -> ParseResult {
        let span_end = self.lexer.consume(TokenKind::QuestionMark)?.span;
        let node = SyntaxNode::NullPropagate(NullPropagate {
            expression,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn error_propagate(&mut self, expression: SyntaxNodeId, _: bool, span_start: Span) -> ParseResult {
        let span_end = self.lexer.consume(TokenKind::ErrorPropagate)?.span;
        let node = SyntaxNode::ErrorPropagate(ErrorPropagate {
            expression,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn error_coalesce(&mut self, expression: SyntaxNodeId, _: bool, span_start: Span) -> ParseResult {
        self.lexer.consume(TokenKind::ErrorCoalesce)?;
        let (_, error_name) = self.lexer.consume_ident()?;
        let block = self.block_expression(false)?;
        let span_end = self.lexer.current().span;
        let span = span_start + span_end;

        // NOTE: The form `expr or e { ...block }` can be lowered into the form:
        // let #result = expr
        // if #result.is_ok {
        //     #result.value
        // } else {
        //     let e = #result.value
        //     ...block
        // }
        // The following code generates the corresponding AST for the lowered form.

        let result_ty = "Result";
        let result_var = "#result";

        let result_temp = SyntaxNode::VarDecl(VarDecl {
            kind: VarDeclKind::Singular(String::from(result_var)),
            is_mutable: false,
            expr: expression,
            type_: Some(TypeAnnotation {
                name: LitIdent::synthesize(result_ty, span_start),
                is_nullable: false,
                span: span_start,
            }),
            span,
        });
        let result_temp = self.arena.alloc(result_temp);
        let result_lit = SyntaxNode::LitIdent(LitIdent::synthesize(result_var, span));
        let result_lit = self.arena.alloc(result_lit);
        let condition = SyntaxNode::FieldAccess(FieldAccess {
            expression: result_lit,
            field: IS_OK.get().as_string(),
            span,
        });
        let condition = self.arena.alloc(condition);
        let result_access = SyntaxNode::FieldAccess(FieldAccess {
            expression: result_lit,
            field: RESULT.get().as_string(),
            span,
        });
        let result_access = self.arena.alloc(result_access);
        let error_var = SyntaxNode::VarDecl(VarDecl {
            kind: VarDeclKind::Singular(error_name),
            is_mutable: false,
            expr: result_access,
            type_: None,
            span,
        });
        let error_var = self.arena.alloc(error_var);
        let error_fallback = SyntaxNode::Block(Block {
            expressions: vec![error_var],
            trailing_expression: Some(block),
            span,
        });
        let error_fallback = self.arena.alloc(error_fallback);
        let result_unwrap = SyntaxNode::IfExpression(IfExpression {
            condition,
            primary: result_access,
            secondary: Some(error_fallback),
            span,
        });
        let result_unwrap = self.arena.alloc(result_unwrap);

        let node = SyntaxNode::Block(Block {
            expressions: vec![result_temp],
            trailing_expression: Some(result_unwrap),
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn index_access(&mut self, expression: SyntaxNodeId, can_assign: bool, span_start: Span) -> ParseResult {
        self.lexer.consume(TokenKind::LeftSquare)?;
        let index_expression = self.expression()?;
        let span_end = self.lexer.consume(TokenKind::RightSquare)?.span;

        let node = SyntaxNode::Index(Index {
            expression,
            index_expression,
            span: span_start + span_end,
        });
        let lhs_expression = self.arena.alloc(node);

        self.parse_assignment(lhs_expression, can_assign, span_start)
    }

    fn field_access(&mut self, lhs: SyntaxNodeId, can_assign: bool, span_start: Span) -> ParseResult {
        self.lexer.consume(TokenKind::Dot)?;

        let (_, field) = self.lexer.consume_ident()?;
        let span_end = self.lexer.current().span;
        let node = SyntaxNode::FieldAccess(FieldAccess {
            expression: lhs,
            field,
            span: span_start + span_end,
        });
        let lhs_expression = self.arena.alloc(node);

        self.parse_assignment(lhs_expression, can_assign, span_start)
    }

    fn super_access(&mut self, can_assign: bool) -> ParseResult {
        let span_start = self.lexer.consume(TokenKind::Super)?.span;
        let next_token = self.lexer.peek()?;

        match next_token.kind {
            TokenKind::Dot | TokenKind::LeftSquare => self.super_method_access(can_assign, span_start),
            TokenKind::LeftParen => self.super_call(span_start),
            kind => self.unexpected_token(
                kind,
                &[TokenKind::Dot, TokenKind::LeftSquare, TokenKind::LeftParen],
                span_start,
            ),
        }
    }

    fn grouping(&mut self, _: bool) -> ParseResult {
        let span_start = self.lexer.consume(TokenKind::LeftParen)?.span;

        if self.lexer.peek()?.kind == TokenKind::RightParen {
            let span_end = self.lexer.consume(TokenKind::RightParen)?.span;

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

    fn fn_call(&mut self, lhs: SyntaxNodeId, _: bool, span_start: Span) -> ParseResult {
        let mut args = Vec::new();

        if self.lexer.peek()?.kind == TokenKind::BackslashArg {
            let Token { span, slice, .. } = self.lexer.next()?;
            let span = *span;

            let backslash_arg = SyntaxNode::LitString(LitString {
                value: process_string(slice, StringMode::Raw)
                    .with_source(|| self.source.clone())
                    .with_span(|| span)?,
                span,
            });
            let backslash_arg = self.arena.alloc(backslash_arg);

            args.push(backslash_arg);
        } else {
            self.lexer.consume(TokenKind::LeftParen)?;

            while self.lexer.peek()?.kind != TokenKind::RightParen {
                let value = self.parse_precedence(Precedence::Assignment)?;
                args.push(value);

                let next = self.lexer.peek()?;
                if next.kind == TokenKind::Comma {
                    self.lexer.next()?;
                } else if next.kind != TokenKind::RightParen {
                    self.unexpected_token(next.kind, &[TokenKind::RightParen], next.span)?;
                }
            }

            self.lexer.consume(TokenKind::RightParen)?;
        }

        let span_end = self.lexer.current().span;
        let node = SyntaxNode::FnCall(FnCall {
            target: lhs,
            args,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn super_call(&mut self, span_start: Span) -> ParseResult {
        self.lexer.consume(TokenKind::LeftParen)?;

        let mut args = Vec::new();

        while self.lexer.peek()?.kind != TokenKind::RightParen {
            let value = self.parse_precedence(Precedence::Assignment)?;
            args.push(value);

            let next = self.lexer.peek()?;
            if next.kind == TokenKind::Comma {
                self.lexer.next()?;
            } else if next.kind != TokenKind::RightParen {
                self.unexpected_token(next.kind, &[TokenKind::RightParen], next.span)?;
            }
        }

        let span_end = self.lexer.consume(TokenKind::RightParen)?.span;
        let node = SyntaxNode::SuperCall(SuperCall {
            args,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn super_method_access(&mut self, _: bool, span_start: Span) -> ParseResult {
        let super_class = if self.lexer.peek()?.kind == TokenKind::LeftSquare {
            self.lexer.consume(TokenKind::LeftSquare)?;
            let (_, ident) = self.lexer.consume_ident()?;
            self.lexer.consume(TokenKind::RightSquare)?;

            Some(ident)
        } else {
            None
        };

        self.lexer.consume(TokenKind::Dot)?;

        let (_, field) = self.lexer.consume_ident()?;
        let span_end = self.lexer.current().span;
        let node = SyntaxNode::SuperAccess(SuperAccess {
            field,
            super_class,
            span: span_start + span_end,
        });

        let super_call = self.arena.alloc(node);

        Ok(super_call)
    }

    fn object(&mut self, _: bool) -> ParseResult {
        let span_start = self.lexer.consume(TokenKind::Object)?.span;
        self.lexer.consume(TokenKind::LeftCurly)?;

        let mut properties = Vec::new();

        while self.lexer.peek()?.kind != TokenKind::RightCurly {
            let next = self.lexer.next()?.clone();
            let key = match next.kind {
                TokenKind::String | TokenKind::Integer | TokenKind::Identifier => next.slice.to_owned(),
                kind => self.unexpected_token(
                    kind,
                    &[TokenKind::String, TokenKind::Integer, TokenKind::Identifier],
                    next.span,
                )?,
            };

            self.lexer.consume(TokenKind::Colon)?;
            let value = self.parse_precedence(Precedence::Assignment)?;

            let next = self.lexer.peek()?;
            if next.kind == TokenKind::Comma {
                self.lexer.next()?;
            } else if next.kind != TokenKind::RightCurly {
                self.unexpected_token(next.kind, &[TokenKind::RightCurly], next.span)?;
            }
            properties.push((key, value));
        }

        let span_end = self.lexer.consume(TokenKind::RightCurly)?.span;

        let node = self.arena.alloc(SyntaxNode::LitObject(LitObject {
            items: properties,
            span: span_start + span_end,
        }));

        Ok(node)
    }

    fn list(&mut self, _: bool) -> ParseResult {
        let span_start = self.lexer.consume(TokenKind::LeftSquare)?.span;

        let mut values = Vec::new();

        while self.lexer.peek()?.kind != TokenKind::RightSquare {
            let value = self.parse_precedence(Precedence::Assignment)?;

            let next = self.lexer.peek()?;
            if next.kind == TokenKind::Comma {
                self.lexer.next()?;
            } else if next.kind != TokenKind::RightSquare {
                self.unexpected_token(next.kind, &[TokenKind::RightSquare], next.span)?;
            }

            values.push(value);
        }

        let span_end = self.lexer.consume(TokenKind::RightSquare)?.span;

        let node = self.arena.alloc(SyntaxNode::LitList(LitList {
            items: values,
            span: span_start + span_end,
        }));

        Ok(node)
    }

    fn literal(&mut self, _: bool) -> ParseResult {
        let token = self.lexer.next()?;
        let span = token.span;
        let literal = match token.kind.clone() {
            TokenKind::Integer => SyntaxNode::LitInt(LitInt {
                // TODO: Better handle integer literals that can't be parsed.
                value: token.slice.parse().map_err(|err: ParseIntError| {
                    Error::new(INVALID_INTEGER_VALUE)
                        .with_span(span)
                        .with_source(self.source.clone())
                        .with_tags(tags! {
                            message => err.to_string()
                        })
                })?,
                span,
            }),
            TokenKind::Float => SyntaxNode::LitFloat(LitFloat {
                value: token.slice.parse().map_err(|err: ParseFloatError| {
                    Error::new(INVALID_FLOAT_VALUE)
                        .with_span(span)
                        .with_source(self.source.clone())
                        .with_tags(tags! {
                            message => err.to_string()
                        })
                })?,
                span,
            }),
            TokenKind::String => SyntaxNode::LitString(LitString {
                value: process_string(token.slice, StringMode::Escaped)
                    .with_source(|| self.source.clone())
                    .with_span(|| span)?,
                span,
            }),
            TokenKind::False => SyntaxNode::LitBool(LitBool { value: false, span }),
            TokenKind::True => SyntaxNode::LitBool(LitBool { value: true, span }),
            TokenKind::Null => SyntaxNode::LitNull(LitNull { span }),
            kind => self.unexpected_token(
                kind,
                &[
                    TokenKind::Integer,
                    TokenKind::Float,
                    TokenKind::String,
                    TokenKind::False,
                    TokenKind::True,
                    TokenKind::Null,
                ],
                span,
            )?,
        };

        Ok(self.arena.alloc(literal))
    }

    fn parse_assignment(&mut self, lhs_expression: SyntaxNodeId, can_assign: bool, span_start: Span) -> ParseResult {
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
                .kind
                .clone();

            let rhs_expression = self.expression()?;
            let span_end = self.lexer.current().span;
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

    fn unexpected_token<R>(&self, actual: TokenKind, expected: &[TokenKind], span: Span) -> Result<R, Error> {
        let mut token_list = expected.to_vec();
        token_list.sort_by_key(|key| key.clone());
        let token_list = token_list
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");

        Err(Error::new(UNEXPECTED_TOKEN)
            .with_span(span)
            .with_source(self.source.clone())
            .with_tags(tags! {
                actual => actual.to_string(),
                expected => token_list
            }))
    }
}

enum StringMode {
    Raw,
    Escaped,
}

fn process_string(input: &str, _mode: StringMode) -> Result<String, Error> {
    // TODO: Process escape sequences.

    Ok(input[1..input.len() - 1].to_owned())
}
