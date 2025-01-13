#[derive(Debug, PartialEq, Clone)]
pub enum Kind {
    Eof,
    Comment,
    Illegal,

    // 1 Character Tokens
    Colon,
    Comma,
    Period,
    Semicolon,
    Dash,
    Less,
    Greater,
    Plus,
    Asterisk,
    Equal,
    Bang,
    /// @
    At,
    Hash,
    Percent,
    Ampersand,
    Slash,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Question,
    Pipe,
    /// ~
    Tilde,
    /// ^
    Caret,
    // 2 Character Tokens
    DoubleEqual,
    BangEqual,
    DoubleAmpersand,
    DoublePipe,
    DoubleSlash,
    DoubleAsterisk,
    LessEq,
    GreaterEq,
    DoublePlus,
    DoubleDash,
    DoubleLess,
    DoubleGreater,
    /// <|
    Assign,
    /// =>
    FatArrow,
    /// ->
    Arrow,
    /// {|
    LBraPipe,
    /// |}
    RBraPipe,
    /// (|
    LParPipe,
    /// |)
    RParPipe,
    /// (<
    LParArrow,
    /// >)
    RParArrow,
    /// $$
    DoubleDollar,
    /// ^=
    CaretAssign,
    /// &=
    AmpersandAssign,
    /// |=
    PipeAssign,
    /// +=
    PlusAssign,
    /// -=
    DashAssign,
    /// /=
    SlashAssign,
    /// %=
    PercentAssign,
    /// *=
    AsteriskAssign,
    /// ::
    DoubleColon,
    /// ..
    DoublePeriod,
    /// <>
    LessGreater,

    // 3 Character Tokens
    /// ...
    Ellipsis,
    /// >>=
    DoubleGreaterAssign,
    /// <<=
    DoubleLessAssign,
    /// **=
    DoubleAsteriskAssign,
    /// //=
    DoubleSlashAssign,
    /// ..=
    DoublePeriodAssign,
    // Literals
    StringLiteral,
    BooleanLiteral,
    IntegerLiteral,
    FloatLiteral,
    // Keywords
    DeferKw,
    FuncKw,
    /// |>
    ReturnKw,
    DoKw,
    BreakKw,
    ContinueKw,
    IfKw,
    ElseKw,
    ForKw,
    InKw,
    LoopKw,
    WhileKw,
    RepeatKw,
    UntilKw,
    MutKw,
    MatchKw,
    SizeofKw,
    PubKw,
    ModuleKw,
    ImportKw,
    ConstKw,
    VarKw,

    // Primitive types
    PrimitiveType,
    // void
    // VoidType,
    // nil
    // NilType,
    // uint
    // UIntType,
    // int
    // IntType,
    // float
    // FloatType,
    // bool
    // BoolType,
    // str
    // StrType,
    /// Identifier
    Identifier,
}

impl Kind {
    pub fn is_binary_op(&self) -> bool {
        matches!(
            self,
            Kind::Plus
                | Kind::Dash
                | Kind::DoubleAsterisk
                | Kind::DoubleSlash
                | Kind::Slash
                | Kind::Percent
                | Kind::Asterisk
                | Kind::LessGreater
                | Kind::DoubleGreater
                | Kind::DoubleLess
                | Kind::GreaterEq
                | Kind::Greater
                | Kind::LessEq
                | Kind::Less
                | Kind::Ampersand
                | Kind::Pipe
                | Kind::Caret
                | Kind::Equal
                | Kind::BangEqual
                | Kind::DoubleAmpersand
                | Kind::DoublePipe
        )
    }
}
