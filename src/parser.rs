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
        todo!()
    }

    fn parse_func_body(&mut self) -> Result<Vec<ParserToken>, ()> {
        let func_body = self
            .take_while(|token| *token != LexerToken::EndBlock)
            .map(|token| self.parse_token(token))
            .collect();

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
                return {
                    ParserToken::Function {
                        identifier: fn_name.clone(),
                        content: self.parse_func_body().unwrap(),
                    }
                }
            }
            _ => panic!("fn does not match"),
        }

        todo!()
    }

    fn parse_expression() {}

    fn parse_assignment(&mut self) -> Result<ParserToken, ()> {
        match self.window(0, 5) {
            [Some(LexerToken::LetDeclaration | LexerToken::ConstDeclaration), Some(LexerToken::Identifier(variable_name)), Some(LexerToken::Equals), Some(LexerToken::StringLiteral(value) | LexerToken::Number(value)), Some(LexerToken::EndStatement)] =>
            {
                let assigment_type = self.peek_nth(0).unwrap().clone();
                let assigment_type = if assigment_type == LexerToken::LetDeclaration {
                    AssigmentType::Let
                } else {
                    AssigmentType::Const
                };

                let value = if let LexerToken::StringLiteral(_) =
                    LexerToken::StringLiteral("hello".to_string())
                {
                    ExpressionKind::StringLiteral(value.clone())
                } else {
                    ExpressionKind::Number(value.clone())
                };

                return Ok(ParserToken::Assignment {
                    assigment_type,
                    identifier: variable_name.clone(),
                    source: value,
                });
            }
            _ => Err(()),
        }
    }
}
