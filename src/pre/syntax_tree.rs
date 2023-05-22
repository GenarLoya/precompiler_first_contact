use super::{TokenRegister, TokenContainer};
use colour::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SyntaxTree {
    pub token: TokenRegister,
    pub next_tokens: Option<Vec<SyntaxTree>>,
}

impl SyntaxTree {
    pub fn check_syntax(&self, idx: usize,lex: &Vec<TokenContainer>) -> bool{
        let mut i = idx;
        let mut err = false;
        let mut err_here = false;
        let mut err_token = TokenRegister::Pending;

        if lex.len() == idx && lex[i-1].token != TokenRegister::Semicolon {
            red_ln!("\u{1F608} SYNTAX ERROR AFTER >>>>> {:?}, expected token {:?}", lex[i-1].token, TokenRegister::Semicolon);
            return true;
        }

        if let Some(tokens) = &self.next_tokens {
            for token in tokens {
                
                if token.token == lex[i].token {
                    // println!("{:?}=={:?}", token.token, lex[i].token);
                    blue_ln!(">>>>{:?}", token.token);
                    i += 1;
                    err = token.check_syntax(i, lex);
                    err_here = false;
                    break;
                }else{
                    err_here = true;
                    err_token = token.token.clone();
                }
            }

            if err_here{
                red_ln!("\u{1F608} SYNTAX ERROR AFTER >>>>> {:?}, expected token {:?}", lex[i-1].token, err_token);
            }
        }

        err
    }
}

