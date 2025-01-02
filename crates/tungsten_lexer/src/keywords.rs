use crate::Kind;

pub const KEYWORDS: &[&str] = &[
    "defer", "func", "do", "break", "continue", "if", "else", "for", "in", "loop", "while",
    "repeat", "until", "match", "sizeof", "pub", "module", "import", "const", "var",
];

pub fn is_keyword(value: &str) -> bool {
    if value == "|>" {
        return true;
    }

    KEYWORDS.contains(&value)
}

pub fn str_to_keyword_kind(value: &str) -> Option<Kind> {
    match value {
        "defer" => Some(Kind::DeferKw),
        "func" => Some(Kind::FuncKw),
        "do" => Some(Kind::DoKw),
        "break" => Some(Kind::BreakKw),
        "|>" => Some(Kind::ReturnKw),
        "continue" => Some(Kind::ContinueKw),
        "if" => Some(Kind::IfKw),
        "else" => Some(Kind::ElseKw),
        "for" => Some(Kind::ForKw),
        "in" => Some(Kind::InKw),
        "loop" => Some(Kind::LoopKw),
        "while" => Some(Kind::WhileKw),
        "repeat" => Some(Kind::RepeatKw),
        "until" => Some(Kind::UntilKw),
        "match" => Some(Kind::MatchKw),
        "sizeof" => Some(Kind::SizeofKw),
        "pub" => Some(Kind::PubKw),
        "module" => Some(Kind::ModuleKw),
        "import" => Some(Kind::ImportKw),
        "const" => Some(Kind::ConstKw),
        "var" => Some(Kind::VarKw),

        _ => None,
    }
}
