use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum CompareType {
    Regex,
    Char,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub id: Uuid,
    pub name: String,
    pub regex: String,
    pub compare_type: CompareType,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum SymbolsRegister {
    Void,
    Error,
    Digits,
    Lowercase,
    Uppercase,
    Space,
    OpenParentheses,
    CloseParentheses,
    DoubleCuotes,
    Plus,
    Hyphen,
    Asterisk,
    Slash,
    Comma,
    Semicolon,
    Dot,
    //Especials
    Double
}

pub fn get_new_symbol(symbol: SymbolsRegister) -> Symbol{
    match symbol {
        SymbolsRegister::Void => Symbol {
            id: Uuid::new_v4(),
            name: String::from("Void"),
            regex: String::from(""),
            compare_type: CompareType::Char,
        },
        SymbolsRegister::Error => Symbol {
            id: Uuid::new_v4(),
            name: String::from("Error"),
            regex: String::from(""),
            compare_type: CompareType::Char,
        },
        SymbolsRegister::Digits => Symbol {
            id: Uuid::new_v4(),
            name: String::from("Digits"),
            regex: String::from(r"\d"),
            compare_type: CompareType::Regex,
        },
        SymbolsRegister::Lowercase => Symbol {
            id: Uuid::new_v4(),
            name: String::from("Lowercase"),
            regex: String::from(r"[a-z]"),
            compare_type: CompareType::Regex,
        },
        SymbolsRegister::Uppercase => Symbol {
            id: Uuid::new_v4(),
            name: String::from("Uppercase"),
            regex: String::from("[A-Z]"),
            compare_type: CompareType::Regex,
        },
        SymbolsRegister::Space => Symbol {
            id: Uuid::new_v4(),
            name: String::from("Space"),
            regex: String::from(" "),
            compare_type: CompareType::Char,
        },
        SymbolsRegister::OpenParentheses => Symbol {
            id: Uuid::new_v4(),
            name: String::from("OpenParentheses"),
            regex: String::from("("),
            compare_type: CompareType::Char,
        },
        SymbolsRegister::CloseParentheses => Symbol {
            id: Uuid::new_v4(),
            name: String::from("CloseParentheses"),
            regex: String::from(")"),
            compare_type: CompareType::Char,
        },
        SymbolsRegister::DoubleCuotes => Symbol {
            id: Uuid::new_v4(),
            name: String::from("DoubleCuotes"),
            regex: String::from('"'),
            compare_type: CompareType::Char,
        },
        SymbolsRegister::Plus => Symbol {
            id: Uuid::new_v4(),
            name: String::from("Plus"),
            regex: String::from("+"),
            compare_type: CompareType::Char,
        },
        SymbolsRegister::Hyphen => Symbol {
            id: Uuid::new_v4(),
            name: String::from("Hyphen"),
            regex: String::from("-"),
            compare_type: CompareType::Char,
        },
        SymbolsRegister::Asterisk => Symbol {
            id: Uuid::new_v4(),
            name: String::from("Asterisk"),
            regex: String::from("*"),
            compare_type: CompareType::Char,
        },
        SymbolsRegister::Slash => Symbol {
            id: Uuid::new_v4(),
            name: String::from("Slash"),
            regex: String::from("/"),
            compare_type: CompareType::Char,
        },
        SymbolsRegister::Comma => Symbol {
            id: Uuid::new_v4(),
            name: String::from("Comma"),
            regex: String::from(","),
            compare_type: CompareType::Char,
        },
        SymbolsRegister::Semicolon => Symbol {
            id: Uuid::new_v4(),
            name: String::from("Semicolon"),
            regex: String::from(";"),
            compare_type: CompareType::Char,
        },
        SymbolsRegister::Dot => Symbol {
            id: Uuid::new_v4(),
            name: String::from("Dot"),
            regex: String::from("."),
            compare_type: CompareType::Char,
        },
        SymbolsRegister::Double => Symbol {
            id: Uuid::new_v4(),
            name: String::from("Double"),
            regex: String::from(""),
            compare_type: CompareType::Char,
        },
    }
}

