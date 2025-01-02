#[derive(Debug, PartialEq)]
pub enum Kind {
    Eof,
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
    /// $$
    DoubleDollar,
    /// ^=
    CaretAssign,
    /// |=
    AmpersandAssign,
    /// &=
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
    MatchKw,
    SizeofKw,
    PubKw,
    ModuleKw,
    ImportKw,
    ConstKw,
    VarKw,

    // Primitive types
    /// void
    VoidType,
    /// nil
    NilType,
    /// uint
    UIntType,
    /// int
    IntType,
    /// float
    FloatType,
    /// bool
    BoolType,
    /// str
    StrType,

    /// Identifier
    Identifier,
}
