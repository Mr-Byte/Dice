use super::{
    error::SyntaxError,
    lexer::{Lexer, Token, TokenKind},
    Assignment, AssignmentOperator, Binary, BinaryOperator, Block, Break, Continue, FnDecl, FunctionCall, IfExpression,
    LitAnonymousFn, LitBool, LitFloat, LitIdent, LitInt, LitList, LitNull, LitObject, LitString, LitUnit, Return,
    SyntaxNode, SyntaxNodeId, SyntaxTree, Unary, UnaryOperator, VarDecl, WhileLoop,
};
use crate::{FieldAccess, ForLoop, Index, OpDecl, SafeAccess, Span};
use id_arena::Arena;

type SyntaxNodeResult = Result<SyntaxNodeId, SyntaxError>;

type PrefixParser = fn(&mut Parser, can_assign: bool) -> Result<SyntaxNodeId, SyntaxError>;
type InfixParser =
    fn(&mut Parser, lhs: SyntaxNodeId, can_assign: bool, span: Span) -> Result<SyntaxNodeId, SyntaxError>;

#[derive(Default)]
struct ParserRule {
    prefix: Option<PrefixParser>,
    infix: Option<InfixParser>,
    precedence: RulePrecedence,
}

impl ParserRule {
    fn new(prefix: Option<PrefixParser>, infix: Option<InfixParser>, precedence: RulePrecedence) -> Self {
        Self {
            prefix,
            infix,
            precedence,
        }
    }
}

impl ParserRule {
    fn for_token(token: &Token) -> Result<ParserRule, SyntaxError> {
        let rule = match token.kind {
            // Empty rules
            TokenKind::RightSquare => ParserRule::default(),
            TokenKind::RightParen => ParserRule::default(),
            TokenKind::RightCurly => ParserRule::default(),
            TokenKind::Semicolon => ParserRule::default(),
            TokenKind::Comma => ParserRule::default(),
            TokenKind::Colon => ParserRule::default(),
            TokenKind::Assign => ParserRule::default(),
            TokenKind::MulAssign => ParserRule::default(),
            TokenKind::DivAssign => ParserRule::default(),
            TokenKind::AddAssign => ParserRule::default(),
            TokenKind::SubAssign => ParserRule::default(),

            // Literals
            TokenKind::Integer(_) => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::Float(_) => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::String(_) => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::Null => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::False => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::True => ParserRule::new(Some(Parser::literal), None, RulePrecedence::Primary),
            TokenKind::Identifier(_) => ParserRule::new(Some(Parser::variable), None, RulePrecedence::Primary),

            // If expression
            TokenKind::If => ParserRule::new(Some(Parser::if_expression), None, RulePrecedence::None),

            // Objects
            TokenKind::Object => ParserRule::new(Some(Parser::object), None, RulePrecedence::Primary),
            TokenKind::LeftSquare => {
                ParserRule::new(Some(Parser::list), Some(Parser::index_access), RulePrecedence::Call)
            }
            TokenKind::Dot => ParserRule::new(None, Some(Parser::field_access), RulePrecedence::Call),
            TokenKind::SafeDot => ParserRule::new(None, Some(Parser::safe_field_access), RulePrecedence::Call),

            // Grouping
            TokenKind::LeftParen => {
                ParserRule::new(Some(Parser::grouping), Some(Parser::fn_call), RulePrecedence::Call)
            }

            // Block expressions
            TokenKind::LeftCurly => ParserRule::new(Some(Parser::block_expression), None, RulePrecedence::None),

            // Operators
            TokenKind::Pipeline => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Pipeline),
            TokenKind::Coalesce => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Coalesce),
            TokenKind::ExclusiveRange => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Range),
            TokenKind::InclusiveRange => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Range),
            TokenKind::LazyAnd => ParserRule::new(None, Some(Parser::binary), RulePrecedence::And),
            TokenKind::Pipe => ParserRule::new(Some(Parser::anonymous_fn), Some(Parser::binary), RulePrecedence::Or),
            TokenKind::Equal => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Comparison),
            TokenKind::NotEqual => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Comparison),
            TokenKind::Greater => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Comparison),
            TokenKind::GreaterEqual => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Comparison),
            TokenKind::Less => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Comparison),
            TokenKind::LessEqual => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Comparison),
            TokenKind::Star => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Factor),
            TokenKind::Slash => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Factor),
            TokenKind::Remainder => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Factor),
            TokenKind::Plus => ParserRule::new(None, Some(Parser::binary), RulePrecedence::Term),
            TokenKind::Minus => ParserRule::new(Some(Parser::unary), Some(Parser::binary), RulePrecedence::Term),
            TokenKind::DiceRoll => ParserRule::new(Some(Parser::unary), Some(Parser::binary), RulePrecedence::DiceRoll),
            TokenKind::Not => ParserRule::new(Some(Parser::unary), None, RulePrecedence::Unary),

            // Setup reserved keywords and sequence with a parser that returns a friendly error.

            // End of input
            TokenKind::EndOfInput => ParserRule::new(None, None, RulePrecedence::None),
            _ => return Err(SyntaxError::UnexpectedToken(token.clone())),
        };

        Ok(rule)
    }
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum RulePrecedence {
    None,
    Assignment,
    Pipeline,
    Coalesce,
    Range,
    Or,
    And,
    Comparison,
    Term,
    Factor,
    DiceRoll,
    Unary,
    Call,
    Object,
    Primary,
}

impl RulePrecedence {
    fn increment(self) -> Self {
        match self {
            RulePrecedence::None => RulePrecedence::Assignment,
            RulePrecedence::Assignment => RulePrecedence::Pipeline,
            RulePrecedence::Pipeline => RulePrecedence::Coalesce,
            RulePrecedence::Coalesce => RulePrecedence::Range,
            RulePrecedence::Range => RulePrecedence::Or,
            RulePrecedence::Or => RulePrecedence::And,
            RulePrecedence::And => RulePrecedence::Comparison,
            RulePrecedence::Comparison => RulePrecedence::Term,
            RulePrecedence::Term => RulePrecedence::Factor,
            RulePrecedence::Factor => RulePrecedence::DiceRoll,
            RulePrecedence::DiceRoll => RulePrecedence::Unary,
            RulePrecedence::Unary => RulePrecedence::Call,
            RulePrecedence::Call => RulePrecedence::Object,
            RulePrecedence::Object => RulePrecedence::Primary,
            RulePrecedence::Primary => RulePrecedence::Primary,
        }
    }
}

impl Default for RulePrecedence {
    fn default() -> Self {
        Self::None
    }
}

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
        let mut next_token = self.lexer.peek();
        let span_start = next_token.span();
        let mut trailing_expression = None;

        while !matches!(next_token.kind, TokenKind::EndOfInput | TokenKind::RightCurly) {
            let expression = match next_token.kind {
                TokenKind::If => self.if_expression(false)?,
                TokenKind::While => self.while_statement()?,
                TokenKind::For => self.for_statement()?,
                TokenKind::Let => self.variable_decl()?,
                TokenKind::Function => self.fn_decl()?,
                TokenKind::Operator => self.op_decl()?,
                TokenKind::Return | TokenKind::Break | TokenKind::Continue => self.control_flow()?,
                _ => self.expression()?,
            };

            next_token = self.lexer.peek();

            if matches!(next_token.kind, TokenKind::EndOfInput | TokenKind::RightCurly) {
                trailing_expression = Some(expression);
                break;
            }

            if next_token.kind == TokenKind::Semicolon {
                self.lexer.consume(TokenKind::Semicolon)?;
                next_token = self.lexer.peek();
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
        let next_token = self.lexer.peek();
        let rule = ParserRule::for_token(&next_token)?;
        let mut node = rule
            .prefix
            .map(|prefix| prefix(self, precedence <= RulePrecedence::Assignment))
            .unwrap_or_else(|| Err(SyntaxError::UnexpectedToken(next_token.clone())))?;

        loop {
            let span_start = next_token.span();
            let next_token = self.lexer.peek();
            let rule = ParserRule::for_token(&next_token)?;

            if precedence > rule.precedence {
                break;
            }

            node = rule
                .infix
                .map(|infix| infix(self, node, precedence <= RulePrecedence::Assignment, span_start))
                .unwrap_or_else(|| Err(SyntaxError::UnexpectedToken(next_token)))?;
        }

        Ok(node)
    }

    fn if_expression(&mut self, _: bool) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::If)?.span();
        let condition = self.expression()?;
        let primary = self.block_expression(false)?;
        let secondary = if self.lexer.peek().kind == TokenKind::Else {
            self.lexer.consume(TokenKind::Else)?;

            match self.lexer.peek().kind {
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
        let token = self.lexer.next();

        let node = match token.kind {
            TokenKind::Break => SyntaxNode::Break(Break { span: token.span() }),
            TokenKind::Continue => SyntaxNode::Continue(Continue { span: token.span() }),
            TokenKind::Return => {
                let result = if self.lexer.peek().kind != TokenKind::Semicolon {
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
            _ => return Err(SyntaxError::UnexpectedToken(token)),
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
        let next_token = self.lexer.next();
        let span_start = next_token.span();
        let lhs_expression = if let TokenKind::Identifier(name) = next_token.kind {
            self.arena
                .alloc(SyntaxNode::LitIdent(LitIdent { name, span: span_start }))
        } else {
            return Err(SyntaxError::UnexpectedToken(next_token));
        };

        self.parse_assignment(lhs_expression, can_assign, span_start)
    }

    fn variable_decl(&mut self) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::Let)?.span();

        let is_mutable = if self.lexer.peek().kind == TokenKind::Mut {
            self.lexer.consume(TokenKind::Mut)?;
            true
        } else {
            false
        };

        let token = self.lexer.next();
        let name = if let TokenKind::Identifier(name) = token.kind {
            name
        } else {
            return Err(SyntaxError::UnexpectedToken(token));
        };

        self.lexer.consume(TokenKind::Assign)?;
        let expr = self.expression()?;
        let span_end = self.lexer.current().span();
        let node = SyntaxNode::VarDecl(VarDecl {
            name,
            is_mutable,
            expr,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn anonymous_fn(&mut self, _: bool) -> SyntaxNodeResult {
        let span_start = self.lexer.peek().span();
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
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn fn_decl(&mut self) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::Function)?.span();
        let name_token = self.lexer.next();
        let name = match name_token.kind {
            TokenKind::Identifier(ref name) => name.clone(),
            _ => return Err(SyntaxError::UnexpectedToken(name_token)),
        };
        let args = self.parse_args(TokenKind::LeftParen, TokenKind::RightParen)?;

        if args.len() > (u8::MAX as usize) {
            return Err(SyntaxError::FnTooManyArguments(name, name_token.span()));
        }

        let body = self.block_expression(false)?;
        let span_end = self.lexer.current().span();
        let node = SyntaxNode::FnDecl(FnDecl {
            name,
            args,
            body,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn op_decl(&mut self) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::Operator)?.span();
        self.lexer.consume(TokenKind::Hash)?;
        let name_token = self.lexer.next();
        let name = match name_token.kind {
            TokenKind::Identifier(ref name) => name.clone(),
            _ => return Err(SyntaxError::UnexpectedToken(name_token)),
        };
        let args = self.parse_args(TokenKind::LeftParen, TokenKind::RightParen)?;

        if args.len() > (u8::MAX as usize) {
            return Err(SyntaxError::FnTooManyArguments(name, name_token.span()));
        }

        let body = self.block_expression(false)?;
        let span_end = self.lexer.current().span();
        let node = SyntaxNode::OpDecl(OpDecl {
            name,
            args,
            body,
            span: span_start + span_end,
        });

        Ok(self.arena.alloc(node))
    }

    fn parse_args(
        &mut self,
        open_token_kind: TokenKind,
        close_token_kind: TokenKind,
    ) -> Result<Vec<String>, SyntaxError> {
        self.lexer.consume(open_token_kind)?;

        let mut args = Vec::new();

        while self.lexer.peek().kind != close_token_kind {
            let (_, arg_name) = self.lexer.consume_ident()?;
            args.push(arg_name);

            if self.lexer.peek().kind == TokenKind::Comma {
                self.lexer.next();
            } else if self.lexer.peek().kind != close_token_kind {
                return Err(SyntaxError::UnexpectedToken(self.lexer.next()));
            }
        }

        self.lexer.consume(close_token_kind)?;
        Ok(args)
    }

    fn binary(&mut self, lhs: SyntaxNodeId, _: bool, span_start: Span) -> SyntaxNodeResult {
        let token = self.lexer.next();
        let rule = ParserRule::for_token(&token)?;
        let operator = match token.kind {
            TokenKind::Pipeline => BinaryOperator::Pipeline,
            TokenKind::Coalesce => BinaryOperator::Coalesce,
            TokenKind::ExclusiveRange => BinaryOperator::RangeExclusive,
            TokenKind::InclusiveRange => BinaryOperator::RangeInclusive,
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
        let rhs = self.parse_precedence(rule.precedence.increment())?;
        let node = SyntaxNode::Binary(Binary {
            operator,
            lhs_expression: lhs,
            rhs_expression: rhs,
            span: span_start + token.span(),
        });

        Ok(self.arena.alloc(node))
    }

    fn unary(&mut self, _: bool) -> SyntaxNodeResult {
        let token = self.lexer.next();
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

    fn safe_field_access(&mut self, lhs: SyntaxNodeId, _: bool, span_start: Span) -> SyntaxNodeResult {
        self.lexer.consume(TokenKind::SafeDot)?;

        let (_, field) = self.lexer.consume_ident()?;
        let span_end = self.lexer.current().span();
        let node = SyntaxNode::SafeAccess(SafeAccess {
            expression: lhs,
            field,
            span: span_start + span_end,
        });
        let lhs_expression = self.arena.alloc(node);

        Ok(lhs_expression)
    }

    fn grouping(&mut self, _: bool) -> SyntaxNodeResult {
        let span_start = self.lexer.consume(TokenKind::LeftParen)?.span();

        if self.lexer.peek().kind == TokenKind::RightParen {
            let span_end = self.lexer.consume(TokenKind::RightParen)?.span();

            let node = SyntaxNode::LitUnit(LitUnit {
                span: span_start + span_end,
            });
            Ok(self.arena.alloc(node))
        } else {
            let expression = self.expression()?;
            // TODO: Detect trailing commas and produce a tuple instead of a group?
            // How to support single-element tuples?
            // Do like rust and require a singular trailing comma for single element tuples!
            self.lexer.consume(TokenKind::RightParen)?;

            Ok(expression)
        }
    }

    fn fn_call(&mut self, lhs: SyntaxNodeId, _: bool, span_start: Span) -> SyntaxNodeResult {
        self.lexer.consume(TokenKind::LeftParen)?;

        let mut args = Vec::new();

        while self.lexer.peek().kind != TokenKind::RightParen {
            let value = self.parse_precedence(RulePrecedence::Assignment)?;
            args.push(value);

            if self.lexer.peek().kind == TokenKind::Comma {
                self.lexer.next();
            } else if self.lexer.peek().kind != TokenKind::RightParen {
                return Err(SyntaxError::UnexpectedToken(self.lexer.next()));
            }
        }

        let span_end = self.lexer.consume(TokenKind::RightParen)?.span();
        let node = SyntaxNode::FunctionCall(FunctionCall {
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

        while self.lexer.peek().kind != TokenKind::RightCurly {
            let key = match self.lexer.next().kind {
                TokenKind::String(value) => value.trim_matches('"').to_owned(),
                TokenKind::Integer(value) => value.to_string(),
                TokenKind::Identifier(value) => value,
                _ => return Err(SyntaxError::UnexpectedToken(self.lexer.current().clone())),
            };

            self.lexer.consume(TokenKind::Colon)?;
            let value = self.parse_precedence(RulePrecedence::Assignment)?;

            if self.lexer.peek().kind == TokenKind::Comma {
                self.lexer.next();
            } else if self.lexer.peek().kind != TokenKind::RightCurly {
                return Err(SyntaxError::UnexpectedToken(self.lexer.next()));
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

        while self.lexer.peek().kind != TokenKind::RightSquare {
            let value = self.parse_precedence(RulePrecedence::Assignment)?;

            if self.lexer.peek().kind == TokenKind::Comma {
                self.lexer.next();
            } else if self.lexer.peek().kind != TokenKind::RightSquare {
                return Err(SyntaxError::UnexpectedToken(self.lexer.next()));
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
        let token = self.lexer.next();
        let span = token.span();
        let literal = match token.kind {
            TokenKind::Integer(value) => SyntaxNode::LitInt(LitInt { value, span }),
            TokenKind::Float(value) => SyntaxNode::LitFloat(LitFloat { value, span }),
            TokenKind::String(value) => SyntaxNode::LitString(LitString {
                value: value.trim_matches('"').to_owned(),
                span,
            }),
            TokenKind::False => SyntaxNode::LitBool(LitBool { value: false, span }),
            TokenKind::True => SyntaxNode::LitBool(LitBool { value: true, span }),
            TokenKind::Null => SyntaxNode::LitNull(LitNull { span }),
            _ => return Err(SyntaxError::UnexpectedToken(token.clone())),
        };

        Ok(self.arena.alloc(literal))
    }

    fn parse_assignment(
        &mut self,
        lhs_expression: SyntaxNodeId,
        can_assign: bool,
        span_start: Span,
    ) -> SyntaxNodeResult {
        let next_token_kind = self.lexer.peek().kind;
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
    use crate::{error::SyntaxError, Binary, BinaryOperator, Block, LitInt, SyntaxNode, Unary, UnaryOperator};

    #[test]
    fn test_parse_integer() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("5").parse()?;
        let root = syntax_tree.get(syntax_tree.root()).unwrap();

        if let SyntaxNode::Block(Block {
            trailing_expression: Some(block),
            ..
        }) = root
        {
            let node = syntax_tree.get(*block);

            assert!(matches!(node, Some(SyntaxNode::LitInt(LitInt { value: 5, .. }))));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_unary_minus() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("-5").parse()?;
        let root = syntax_tree.get(syntax_tree.root()).unwrap();

        if let SyntaxNode::Block(Block {
            trailing_expression: Some(block),
            ..
        }) = root
        {
            let node = syntax_tree.get(*block);

            assert!(matches!(
                node,
                Some(SyntaxNode::Unary(Unary { operator: UnaryOperator::Negate, .. }))
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_binary_minus() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("5 - 5").parse()?;
        let root = syntax_tree.get(syntax_tree.root()).unwrap();

        if let SyntaxNode::Block(Block {
            trailing_expression: Some(block),
            ..
        }) = root
        {
            let node = syntax_tree.get(*block);

            assert!(matches!(
                node,
                Some(SyntaxNode::Binary(Binary { operator: BinaryOperator::Subtract, .. }))
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_binary_minus_with_unary_minus() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("5 - -5").parse()?;
        let root = syntax_tree.get(syntax_tree.root()).unwrap();

        if let SyntaxNode::Block(Block {
            trailing_expression: Some(block),
            ..
        }) = root
        {
            let node = syntax_tree.get(*block);

            assert!(matches!(
                node,
                Some(SyntaxNode::Binary(Binary { operator: BinaryOperator::Subtract, .. }))
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_binary_precedence_multiply_right() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("5 - 5 * 5").parse()?;
        let root = syntax_tree.get(syntax_tree.root()).unwrap();

        if let SyntaxNode::Block(Block {
            trailing_expression: Some(block),
            ..
        }) = root
        {
            let node = syntax_tree.get(*block);

            assert!(matches!(
                node,
                Some(SyntaxNode::Binary(Binary { operator: BinaryOperator::Subtract, .. }))
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_binary_precedence_multiply_left() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("5 * 5 - 5").parse()?;
        let root = syntax_tree.get(syntax_tree.root()).unwrap();

        if let SyntaxNode::Block(Block {
            trailing_expression: Some(block),
            ..
        }) = root
        {
            let node = syntax_tree.get(*block);

            assert!(matches!(
                node,
                Some(SyntaxNode::Binary(Binary { operator: BinaryOperator::Subtract, .. }))
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_grouping() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("5 * (5 - 5)").parse()?;
        let root = syntax_tree.get(syntax_tree.root()).unwrap();

        if let SyntaxNode::Block(Block {
            trailing_expression: Some(block),
            ..
        }) = root
        {
            let node = syntax_tree.get(*block);

            assert!(matches!(
                node,
                Some(SyntaxNode::Binary(Binary{ operator: BinaryOperator::Multiply, .. }))
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_unary_die() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("d8").parse()?;
        let root = syntax_tree.get(syntax_tree.root()).unwrap();

        if let SyntaxNode::Block(Block {
            trailing_expression: Some(block),
            ..
        }) = root
        {
            let node = syntax_tree.get(*block);

            assert!(matches!(
                node,
                Some(SyntaxNode::Unary(Unary { operator: UnaryOperator::DiceRoll, .. }))
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_binary_dice() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("6d8").parse()?;
        let root = syntax_tree.get(syntax_tree.root()).unwrap();

        if let SyntaxNode::Block(Block {
            trailing_expression: Some(block),
            ..
        }) = root
        {
            let node = syntax_tree.get(*block);

            assert!(matches!(
                node,
                Some(SyntaxNode::Binary(Binary { operator: BinaryOperator::DiceRoll, .. }))
            ));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_object_expression() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("object { x: 50, y: 30 }").parse()?;
        let root = syntax_tree.get(syntax_tree.root()).unwrap();

        if let SyntaxNode::Block(Block {
            trailing_expression: Some(block),
            ..
        }) = root
        {
            let node = syntax_tree.get(*block);

            assert!(matches!(node, Some(SyntaxNode::LitObject(_))));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }

    #[test]
    fn test_parse_list_expression() -> Result<(), SyntaxError> {
        let syntax_tree = Parser::new("[x, y, 1, 1*2, object {}]").parse()?;
        let root = syntax_tree.get(syntax_tree.root()).unwrap();

        if let SyntaxNode::Block(Block {
            trailing_expression: Some(block),
            ..
        }) = root
        {
            let node = syntax_tree.get(*block);

            assert!(matches!(node, Some(SyntaxNode::LitList(_))));
        } else {
            panic!("Root element is not a block.")
        }

        Ok(())
    }
}
