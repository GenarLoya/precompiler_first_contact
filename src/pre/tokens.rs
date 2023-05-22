#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenRegister {
    Error,
    Pending,
    Integer,
    Double,
    Uppercase,
    Lowercase,
    Space,
    OpenParentheses,
    CloseParentheses,
    DoubleCuotes,
    Add,
    Substract,
    Multiplier,
    Divide,
    Comma,
    Semicolon,
    Dot,
    //Reserved functions
    Print,
    Println,
    Addfn,
}

pub fn get_resfunc(container: &mut TokenContainer){
    container.token = match container.text.as_str() {
        //Reserved functions
        "print" => TokenRegister::Print,
        "println" => TokenRegister::Println,
        "addfn" => TokenRegister::Addfn,
        _ => container.token,
    }
}

#[derive(Debug, Clone, )]
pub struct TokenContainer{
    pub token: TokenRegister,
    pub text: String
}

impl TokenRegister {
    pub fn get_token(&self) -> String{
        match self {
            TokenRegister::Error => String::from("Error"),
            TokenRegister::Pending => String::from("Pending"),
            TokenRegister::Integer => String::from("Integer"),
            TokenRegister::Double => String::from("Double"),
            TokenRegister::Uppercase => String::from("Uppercase"),
            TokenRegister::Lowercase => String::from("Lowercase"),
            TokenRegister::Space => String::from("Space"),
            TokenRegister::OpenParentheses => String::from("OpenParentheses"),
            TokenRegister::CloseParentheses => String::from("CloseParentheses"),
            TokenRegister::DoubleCuotes => String::from("DoubleCuotes"),
            TokenRegister::Add => String::from("Add"),
            TokenRegister::Substract => String::from("Substract"),
            TokenRegister::Multiplier => String::from("Multiplier"),
            TokenRegister::Divide => String::from("Divide"),
            TokenRegister::Comma => String::from("Comma"),
            TokenRegister::Semicolon => String::from("Semicolon"),
            TokenRegister::Dot => String::from("Dot"),
            //Reserved functions
            _ => String::from("Reserved function"),
        }
    }
}