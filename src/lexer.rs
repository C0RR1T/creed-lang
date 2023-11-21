enum LexerToken<'a> {
    FnDeclaration,
    Identifier(&'a str),
    LetDeclaration,
    ConstDeclaration,

    BeginBlock,
    EndBlock,
    EndStatement,

    Equals,
    If,
    GreaterThan,
    LessThan,
    GreaterEqualThan,
    LessEqualThen,

    ReturnFn,
    ReturnBlock,

    // Literals
    String(&'a str),
    Number(Number),
    Boolean(bool),
}

enum Number {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}

struct Lexer<'a> {
    input: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn lex(&self) -> Vec<LexerToken> {
        todo!("Implement this")
    }
}
