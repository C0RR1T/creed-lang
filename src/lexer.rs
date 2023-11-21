use std::str::CharIndices;

use peekmore::{PeekMore, PeekMoreIterator};
use unicode_xid::UnicodeXID;

#[derive(Eq, PartialEq, Clone, Debug)]
enum LexerToken {
    // Declarations
    FnDeclaration,
    Identifier(String),
    LetDeclaration,
    ConstDeclaration,

    BeginBlock,
    EndBlock,
    EndStatement,
    OpenParen,
    CloseParen,

    // Control flow
    If,
    Then,
    Else,

    // Comparisons
    Equals,
    GreaterThan,
    LessThan,

    ReturnFn,
    Use,

    // Literals
    StringLiteral(String),
    Number(u128),
    Boolean(bool),
}

impl LexerToken {
    pub fn from_string(input: &str) -> Self {
        match input {
            "if" => LexerToken::If,
            "fn" => LexerToken::FnDeclaration,
            "let" => LexerToken::LetDeclaration,
            identifier => LexerToken::Identifier(identifier.to_string()),
        }
    }
}

struct Lexer<'a> {
    input: PeekMoreIterator<CharIndices<'a>>,
}
type CharIndex = (usize, char);

impl<'a> Lexer<'a> {
    fn next(&mut self) -> Option<CharIndex> {
        self.input.next()
    }

    fn peek(&mut self) -> Option<&CharIndex> {
        self.input.peek_next()
    }

    fn peek_nth(&mut self, amount: usize) -> Option<&CharIndex> {
        self.input.peek_nth(amount)
    }

    fn consume_elements(&mut self, amount: usize) {
        for _ in 0..amount {
            self.next();
        }
    }

    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.char_indices().peekmore(),
        }
    }

    pub fn lex(&mut self) -> Vec<LexerToken> {
        let mut out = Vec::new();
        while let Some((_idx, char)) = self.next() {
            if char.is_whitespace() {
                continue;
            }
            match char {
                ';' => out.push(LexerToken::EndStatement),
                '{' => out.push(LexerToken::BeginBlock),
                '}' => out.push(LexerToken::EndBlock),
                '(' => out.push(LexerToken::OpenParen),
                ')' => out.push(LexerToken::CloseParen),
                '=' => out.push(LexerToken::Equals),
                '>' => out.push(LexerToken::GreaterThan),
                '<' => out.push(LexerToken::LessThan),
                pot_number if pot_number.is_numeric() => out.push(LexerToken::Number(
                    self.collect_to_number(pot_number).unwrap(),
                )),
                ident if ident.is_xid_start() => {
                    out.push(LexerToken::from_string(&self.collect_to_string(ident)))
                }
                unknown => panic!("tf is this: {unknown}"),
            }
        }

        out
    }

    fn collect_to_string(&mut self, first_char: char) -> String {
        let mut out = String::from(first_char);
        let mut idx = 0;
        while let Some((_, char)) = self.peek_nth(idx) {
            if !char.is_xid_continue() {
                break;
            }

            out.push(*char);
            idx += 1;
        }
        self.consume_elements(out.len() - 1);
        out
    }

    fn collect_to_number(&mut self, first_digit: char) -> Option<u128> {
        let mut out = String::from(first_digit);
        let mut idx = 0;
        while let Some((_, char)) = self.peek_nth(idx) {
            if !char.is_numeric() {
                break;
            }
            out.push(*char);

            idx += 1;
        }
        self.consume_elements(out.len() - 1);
        out.parse().ok()
    }
}

#[cfg(test)]
const TEST_FILE: &str = include_str!("../tests/simple.cd");

#[test]
fn lex_example() {
    let output = Lexer::new(TEST_FILE).lex();

    assert_eq!(
        &output,
        &[
            LexerToken::FnDeclaration,
            LexerToken::Identifier("main".to_string()),
            LexerToken::OpenParen,
            LexerToken::CloseParen,
            LexerToken::BeginBlock,
            LexerToken::LetDeclaration,
            LexerToken::Identifier("test".to_string()),
            LexerToken::Equals,
            LexerToken::Number(5),
            LexerToken::EndStatement,
            LexerToken::If,
            LexerToken::Identifier("test".to_string()),
            LexerToken::GreaterThan,
            LexerToken::Number(5),
            LexerToken::BeginBlock,
            LexerToken::EndBlock,
            LexerToken::EndBlock
        ]
    )
}
