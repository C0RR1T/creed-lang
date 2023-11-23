use std::iter::TakeWhile;
use std::vec::IntoIter;

use peekmore::{PeekMore, PeekMoreIterator};

use crate::lexer::LexerToken;

#[derive(Debug, Clone, Eq, PartialEq)]
enum ParserToken {
    Function {
        identifier: String,
        content: Vec<ParserToken>,
    },
    Expr(ExpressionKind),
    Assignment {
        identifier: String,
        assigment_type: AssigmentType,
        source: ExpressionKind,
    },
    Block(Vec<ParserToken>),
}
#[derive(Debug, Clone, Eq, PartialEq)]
enum ExpressionKind {
    Identifier(String),
    Comparison {
        left: Box<ExpressionKind>,
        right: Box<ExpressionKind>,
        cpm: ComparisonKind,
    },
    IfShorthand {
        cond: Box<ExpressionKind>,
        then: Box<ExpressionKind>,
        otherwise: Box<ExpressionKind>,
    },
    IfCondition {
        cond: Box<ExpressionKind>,
        then: Vec<ParserToken>,
    },
    StringLiteral(String),
    Number(String),
    AnonFunction {
        content: Vec<ParserToken>,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum AssigmentType {
    Const,
    Let,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum ComparisonKind {
    Equal,
    GreaterThan,
    GreaterEqualThan,
    LessThan,
    LessEqualThan,
    NotEqual,
}

struct Parser {
    lexer_tokens: PeekMoreIterator<IntoIter<LexerToken>>,
}

impl Parser {
    fn next(&mut self) -> Option<LexerToken> {
        self.lexer_tokens.next()
    }

    fn peek(&mut self) -> Option<&LexerToken> {
        self.lexer_tokens.peek()
    }

    fn peek_nth(&mut self, idx: usize) -> Option<&LexerToken> {
        self.lexer_tokens.peek_nth(idx)
    }

    fn consume_elements(&mut self, count: usize) {
        for _ in 0..count {
            self.next();
        }
    }

    fn take_while<P: Fn(&LexerToken) -> bool>(
        &mut self,
        predicate: P,
    ) -> TakeWhile<&mut PeekMoreIterator<IntoIter<LexerToken>>, P> {
        self.lexer_tokens.by_ref().take_while(predicate)
    }

    fn window(&mut self, from: usize, to: usize) -> &[Option<LexerToken>] {
        self.lexer_tokens.peek_range(from, to)
    }

    pub fn new(tokens: Vec<LexerToken>) -> Self {
        Self {
            lexer_tokens: tokens.into_iter().peekmore(),
        }
    }

    pub fn parse(&mut self) -> Vec<ParserToken> {
        let mut out = Vec::new();
        while let Some(token) = self.next() {
            out.push(self.parse_token(token))
        }
        out
    }

    fn parse_token(&mut self, first_token: LexerToken) -> ParserToken {
        match first_token {
            LexerToken::FnDeclaration => self.parse_function(),
            LexerToken::LetDeclaration | LexerToken::ConstDeclaration => {
                self.parse_assignment().unwrap()
            }
            LexerToken::If => {}
            LexerToken::ReturnFn => {}
            _ => panic!("unexpected token"),
        }
    }

    fn parse_if(&mut self) -> Result<ParserToken, ()> {}

    fn parse_func_body(&mut self) -> Result<Vec<ParserToken>, ()> {
        self.consume_elements(5);
        let mut func_body = Vec::new();
        while let Some(token) = self.next() {
            if token == LexerToken::EndBlock {
                break;
            }

            func_body.push(self.parse_token(token));
        }

        // The iterator still contains items, this means `take_while` had to stop due to an EndBlock
        if self.lexer_tokens.peek_nth(0).is_some() {
            Ok(func_body)
        } else {
            Err(())
        }
    }

    fn parse_function(&mut self) -> ParserToken {
        match self.window(1, 4) {
            [Some(LexerToken::Identifier(fn_name)), Some(LexerToken::OpenParen), Some(LexerToken::CloseParen), Some(LexerToken::BeginBlock)] => {
                ParserToken::Function {
                    identifier: fn_name.clone(),
                    content: self.parse_func_body().unwrap(),
                }
            }
            _ => panic!("fn does not match"),
        }
    }

    fn parse_expression(&mut self, first_token: &LexerToken) -> ParserToken {
        match first_token {
            LexerToken::FnDeclaration => {}
            LexerToken::Identifier(_) => {}
            LexerToken::LetDeclaration => {}
            LexerToken::ConstDeclaration => {}
            LexerToken::BeginBlock => {}
            LexerToken::EndBlock => {}
            LexerToken::EndStatement => {}
            LexerToken::OpenParen => {}
            LexerToken::CloseParen => {}
            LexerToken::If => {}
            LexerToken::Then => {}
            LexerToken::Else => {}
            LexerToken::Equals => {}
            LexerToken::GreaterThan => {}
            LexerToken::LessThan => {}
            LexerToken::ReturnFn => {}
            LexerToken::Use => {}
            LexerToken::StringLiteral(_) => {}
            LexerToken::Number(_) => {}
            LexerToken::Boolean(_) => {}
        }

        todo!()
    }

    fn parse_assignment(&mut self) -> Result<ParserToken, ()> {
        match self.window(0, 5) {
            [Some(assignment_type), Some(LexerToken::Identifier(variable_name)), Some(LexerToken::Equals), Some(value_assigment), Some(LexerToken::EndStatement)]
                if (*assignment_type == LexerToken::LetDeclaration
                    || *assignment_type == LexerToken::ConstDeclaration)
                    && (matches!(*value_assigment, LexerToken::StringLiteral(_))
                        || matches!(*value_assigment, LexerToken::Number(_))) =>
            {
                let assigment_type = if *assignment_type == LexerToken::LetDeclaration {
                    AssigmentType::Let
                } else {
                    AssigmentType::Const
                };

                let value = if let LexerToken::StringLiteral(value) = value_assigment {
                    ExpressionKind::StringLiteral(value.clone())
                } else if let LexerToken::Number(value) = value_assigment {
                    ExpressionKind::Number(value.clone())
                } else {
                    unreachable!()
                };

                Ok(ParserToken::Assignment {
                    assigment_type,
                    identifier: variable_name.clone(),
                    source: value,
                })
            }
            _ => Err(()),
        }
    }
}

fn parse_expression(
    iter: &mut PeekMoreIterator<IntoIter<LexerToken>>,
    first_token: &LexerToken,
) -> Result<ParserToken, ()> {
    match first_token {
        LexerToken::FnDeclaration => {}
        LexerToken::Identifier(_) => {}
        LexerToken::LetDeclaration => {}
        LexerToken::ConstDeclaration => {}
        LexerToken::BeginBlock => {}
        LexerToken::EndBlock => {}
        LexerToken::EndStatement => {}
        LexerToken::OpenParen => {}
        LexerToken::CloseParen => {}
        LexerToken::If => {}
        LexerToken::Then => {}
        LexerToken::Else => {}
        LexerToken::Equals => {}
        LexerToken::GreaterThan => {}
        LexerToken::LessThan => {}
        LexerToken::ReturnFn => {}
        LexerToken::Use => {}
        LexerToken::StringLiteral(_) => {}
        LexerToken::Number(_) => {}
        LexerToken::Boolean(_) => {}
    }

    todo!()
}
