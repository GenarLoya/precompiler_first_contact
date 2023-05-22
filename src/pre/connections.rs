use colour::*;
use regex::Regex;
use uuid::Uuid;

use super::{get_resfunc, CompareType, Symbol, SyntaxTree, TokenContainer, TokenRegister};

#[derive(Debug)]
pub struct Target {
    pub node: Symbol,
    pub token: TokenRegister,
    pub regex: String,
    pub compare_type: CompareType,
}

impl Target {
    pub fn new(
        node: &Symbol,
        token: TokenRegister,
        regex: Option<String>,
        compare_type: Option<CompareType>,
    ) -> Self {
        let regex_real = match regex {
            Some(value) => value,
            None => node.regex.clone(),
        };

        let compare_real = match compare_type {
            Some(value) => value,
            None => node.compare_type.clone(),
        };

        Target {
            node: node.clone(),
            token,
            regex: regex_real,
            compare_type: compare_real,
        }
    }

    pub fn get_compare(&self, text: &str) -> bool {
        match self.compare_type {
            CompareType::Char => text == self.regex,
            CompareType::Regex => {
                let re = Regex::new(&self.regex).unwrap();
                re.is_match(text)
            }
        }
    }
}

#[derive(Debug)]
pub struct Connection {
    pub id_conection: uuid::Uuid,
    pub start: Symbol,
    pub targets: Vec<Target>,
    pub error: Target,
}

impl Connection {
    pub fn new(start: &Symbol, targets: Vec<Target>, error: Target) -> Self {
        Connection {
            id_conection: start.id,
            start: start.clone(),
            targets,
            error,
        }
    }

    pub fn match_target(&self, text: &str, lex: &mut Vec<TokenContainer>) -> Uuid {
        for target in &self.targets {
            let target_match: bool = target.get_compare(text);

            if target_match {
                prnt!("End Match with... {:?}\n", target.token);
                green_ln!("{} <=!MATCH!=> {}", target.node.regex, text);
                lex.push(TokenContainer {
                    token: target.token,
                    text: String::from(text),
                });
                return target.node.id;
            }
        }

        prnt!("End Match with... {:?}\n", self.error.token);
        red_ln!("=>ERROR=> {}", text);
        lex.push(TokenContainer {
            token: self.error.token,
            text: String::from(text),
        });
        self.error.node.id
    }
}

pub fn start(
    text: &String,
    conns: &Vec<Connection>,
    syntax: &Vec<SyntaxTree>,
) -> Vec<TokenContainer> {
    green_ln!("\n\u{1F7e2} <<<<Initializing analize...>>>>");

    let mut lex: Vec<TokenContainer> = Vec::new();
    let mut real_lex: Vec<TokenContainer> = Vec::new();

    if !conns.is_empty() {
        let mut real_text = text.clone();
        real_text.push_str("#");
        let mut current_char: &str = &real_text[0..1];
        let mut current_uuid: Uuid = conns[0].match_target(current_char, &mut lex);
        println!("Active char: {}", current_char);
        // println!("Current UUID: {:?} \n", current_uuid);

        for i in 1..real_text.len() {
            current_char = &real_text[i..i + 1];

            for conn in conns {
                if conn.id_conection == current_uuid {
                    current_uuid = conn.match_target(current_char, &mut lex);
                    break;
                }
            }

            println!("Active char: {}", current_char);
        }

        for i in 1..lex.len() {
            lex[i - 1].token = lex[i].token.clone();
        }

        lex.pop().unwrap();

        let mut pending_text = String::new();

        for c in &lex {
            pending_text.push_str(&c.text);

            if c.token != TokenRegister::Pending {
                real_lex.push(TokenContainer {
                    token: c.token,
                    text: pending_text,
                });
                pending_text = String::new();
            }
        }

        for container in &mut real_lex {
            get_resfunc(container)
        }

        green_ln!("\n\u{1F7e2} <<<<Token sequence>>>>");

        for i in &real_lex {
            if i.token == TokenRegister::Error {
                e_red_ln!("\u{274c} {:?} ===> {}", i.token, i.text);
            } else {
                e_blue_ln!("\u{2714} {:?} ===> {}", i.token, i.text);
            }
        }

        let mut nxt = true;
        let mut idx = 1;

        green_ln!("\n\u{1F7e2} <<<<Syntax analize>>>>");

        for container in &real_lex {
            if nxt {
                nxt = false;
                for syn in syntax {
                    if syn.token == container.token {
                        dark_yellow_ln!("\n\u{1F680} function ###<<{:?}>>###", syn.token);
                        blue_ln!(">>>>{:?}", syn.token);
                        syn.check_syntax(idx, &real_lex);
                    }
                }
            }

            if container.token == TokenRegister::Semicolon {
                nxt = true;
            }

            idx += 1;
        }
    }
    green_ln!("Precompilation ended...");
    println!("\u{1F44D} Right...");
    real_lex
}
