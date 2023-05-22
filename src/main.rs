pub mod pre;
pub mod command_reader;

use command_reader::read_command;
use maplit::hashmap;
use std::collections::HashMap;

use pre::{get_new_symbol, Connection, Symbol, SymbolsRegister, Target, TokenRegister, start, CompareType, SyntaxTree};

use crate::pre::TokenContainer;

fn main() {
    //let text:String = String::from("1");

    let symbols: HashMap<SymbolsRegister, Symbol> = hashmap! {
        SymbolsRegister::Void => get_new_symbol(SymbolsRegister::Void),
        SymbolsRegister::Error => get_new_symbol(SymbolsRegister::Error),
        SymbolsRegister::Digits => get_new_symbol(SymbolsRegister::Digits),
        SymbolsRegister::Lowercase => get_new_symbol(SymbolsRegister::Lowercase),
        SymbolsRegister::Uppercase => get_new_symbol(SymbolsRegister::Uppercase),
        SymbolsRegister::Space => get_new_symbol(SymbolsRegister::Space),
        SymbolsRegister::OpenParentheses => get_new_symbol(SymbolsRegister::OpenParentheses),
        SymbolsRegister::CloseParentheses => get_new_symbol(SymbolsRegister::CloseParentheses),
        SymbolsRegister::DoubleCuotes => get_new_symbol(SymbolsRegister::DoubleCuotes),
        SymbolsRegister::Plus => get_new_symbol(SymbolsRegister::Plus),
        SymbolsRegister::Hyphen => get_new_symbol(SymbolsRegister::Hyphen),
        SymbolsRegister::Asterisk => get_new_symbol(SymbolsRegister::Asterisk),
        SymbolsRegister::Slash => get_new_symbol(SymbolsRegister::Slash),
        SymbolsRegister::Comma => get_new_symbol(SymbolsRegister::Comma),
        SymbolsRegister::Semicolon => get_new_symbol(SymbolsRegister::Semicolon),
        SymbolsRegister::Dot => get_new_symbol(SymbolsRegister::Dot),
        //Especials
        SymbolsRegister::Double => get_new_symbol(SymbolsRegister::Double),
    };

    let connections: Vec<Connection> = vec![Connection::new(
        &symbols.get(&SymbolsRegister::Void).unwrap(),
        vec![Target::new(
                &symbols.get(&SymbolsRegister::Digits).unwrap(),
                TokenRegister::Pending,
                None, None
            ),
            // Digits,
            Target::new(
                &symbols.get(&SymbolsRegister::Lowercase).unwrap(),
                TokenRegister::Pending,
                None, None
            ),
            // Lowercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Uppercase).unwrap(),
                TokenRegister::Pending,
                None, None
            ),
            // Uppercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Space).unwrap(),
                TokenRegister::Pending,
                None, None
            ),
            // Space,
            Target::new(
                &symbols.get(&SymbolsRegister::OpenParentheses).unwrap(),
                TokenRegister::Pending,
                None, None
            ),
            // OpenParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::CloseParentheses).unwrap(),
                TokenRegister::Pending,
                None, None
            ),
            // CloseParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::DoubleCuotes).unwrap(),
                TokenRegister::Pending,
                None, None
            ),
            // DoubleCuotes,
            Target::new(
                &symbols.get(&SymbolsRegister::Plus).unwrap(),
                TokenRegister::Pending,
                None, None
            ),
            // Plus,
            Target::new(
                &symbols.get(&SymbolsRegister::Hyphen).unwrap(),
                TokenRegister::Pending,
                None, None
            ),
            // Hyphen,
            Target::new(
                &symbols.get(&SymbolsRegister::Asterisk).unwrap(),
                TokenRegister::Pending,
                None, None
            ),
            // Asterisk,
            Target::new(
                &symbols.get(&SymbolsRegister::Slash).unwrap(),
                TokenRegister::Pending,
                None, None
            ),
            // Slash,
            Target::new(
                &symbols.get(&SymbolsRegister::Comma).unwrap(),
                TokenRegister::Pending,
                None, None
            ),
            // Comma,
            Target::new(
                &symbols.get(&SymbolsRegister::Semicolon).unwrap(),
                TokenRegister::Pending,
                None, None
            ),
            // Semicolon,
            Target::new(
                &symbols.get(&SymbolsRegister::Dot).unwrap(),
                TokenRegister::Pending,
                None, None
            ),
        ],
        Target::new(
            &symbols.get(&SymbolsRegister::Error).unwrap(),
            TokenRegister::Pending,
            None, None
        ),
        // Error,
    ), Connection::new(
        &symbols.get(&SymbolsRegister::Error).unwrap(),
        vec![Target::new(
                &symbols.get(&SymbolsRegister::Digits).unwrap(),
                TokenRegister::Error,
                None, None
            ),
            // Digits,
            Target::new(
                &symbols.get(&SymbolsRegister::Lowercase).unwrap(),
                TokenRegister::Error,
                None, None
            ),
            // Lowercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Uppercase).unwrap(),
                TokenRegister::Error,
                None, None
            ),
            // Uppercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Space).unwrap(),
                TokenRegister::Error,
                None, None
            ),
            // Space,
            Target::new(
                &symbols.get(&SymbolsRegister::OpenParentheses).unwrap(),
                TokenRegister::Error,
                None, None
            ),
            // OpenParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::CloseParentheses).unwrap(),
                TokenRegister::Error,
                None, None
            ),
            // CloseParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::DoubleCuotes).unwrap(),
                TokenRegister::Error,
                None, None
            ),
            // DoubleCuotes,
            Target::new(
                &symbols.get(&SymbolsRegister::Plus).unwrap(),
                TokenRegister::Error,
                None, None
            ),
            // Plus,
            Target::new(
                &symbols.get(&SymbolsRegister::Hyphen).unwrap(),
                TokenRegister::Error,
                None, None
            ),
            // Hyphen,
            Target::new(
                &symbols.get(&SymbolsRegister::Asterisk).unwrap(),
                TokenRegister::Error,
                None, None
            ),
            // Asterisk,
            Target::new(
                &symbols.get(&SymbolsRegister::Slash).unwrap(),
                TokenRegister::Error,
                None, None
            ),
            // Slash,
            Target::new(
                &symbols.get(&SymbolsRegister::Comma).unwrap(),
                TokenRegister::Error,
                None, None
            ),
            // Comma,
            Target::new(
                &symbols.get(&SymbolsRegister::Semicolon).unwrap(),
                TokenRegister::Error,
                None, None
            ),
            // Semicolon,
            Target::new(
                &symbols.get(&SymbolsRegister::Dot).unwrap(),
                TokenRegister::Error,
                None, None
            ),
        ],
        Target::new(
            &symbols.get(&SymbolsRegister::Error).unwrap(),
            TokenRegister::Error,
            None, None
        ),
        // Error,
    ), Connection::new(
        &symbols.get(&SymbolsRegister::Digits).unwrap(),
        vec![Target::new(
                &symbols.get(&SymbolsRegister::Double).unwrap(),
                TokenRegister::Pending,
                Some(String::from(".")), Some(CompareType::Char)
            ),
            Target::new(
                &symbols.get(&SymbolsRegister::Digits).unwrap(),
                TokenRegister::Pending,
                None, None
            ),
            // Digits,
            Target::new(
                &symbols.get(&SymbolsRegister::Lowercase).unwrap(),
                TokenRegister::Integer,
                None, None
            ),
            // Lowercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Uppercase).unwrap(),
                TokenRegister::Integer,
                None, None
            ),
            // Uppercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Space).unwrap(),
                TokenRegister::Integer,
                None, None
            ),
            // Space,
            Target::new(
                &symbols.get(&SymbolsRegister::OpenParentheses).unwrap(),
                TokenRegister::Integer,
                None, None
            ),
            // OpenParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::CloseParentheses).unwrap(),
                TokenRegister::Integer,
                None, None
            ),
            // CloseParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::DoubleCuotes).unwrap(),
                TokenRegister::Integer,
                None, None
            ),
            // DoubleCuotes,
            Target::new(
                &symbols.get(&SymbolsRegister::Plus).unwrap(),
                TokenRegister::Integer,
                None, None
            ),
            // Plus,
            Target::new(
                &symbols.get(&SymbolsRegister::Hyphen).unwrap(),
                TokenRegister::Integer,
                None, None
            ),
            // Hyphen,
            Target::new(
                &symbols.get(&SymbolsRegister::Asterisk).unwrap(),
                TokenRegister::Integer,
                None, None
            ),
            // Asterisk,
            Target::new(
                &symbols.get(&SymbolsRegister::Slash).unwrap(),
                TokenRegister::Integer,
                None, None
            ),
            // Slash,
            Target::new(
                &symbols.get(&SymbolsRegister::Comma).unwrap(),
                TokenRegister::Integer,
                None, None
            ),
            // Comma,
            Target::new(
                &symbols.get(&SymbolsRegister::Semicolon).unwrap(),
                TokenRegister::Integer,
                None, None
            ),
            // Semicolon,
        ],
        Target::new(
            &symbols.get(&SymbolsRegister::Error).unwrap(),
            TokenRegister::Integer,
            None, None
        ),
        // Error,
    ), Connection::new(
        &symbols.get(&SymbolsRegister::Lowercase).unwrap(),
        vec![Target::new(
                &symbols.get(&SymbolsRegister::Digits).unwrap(),
                TokenRegister::Lowercase,
                None, None
            ),
            // Digits,
            Target::new(
                &symbols.get(&SymbolsRegister::Lowercase).unwrap(),
                TokenRegister::Pending,
                None, None
            ),
            // Lowercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Uppercase).unwrap(),
                TokenRegister::Lowercase,
                None, None
            ),
            // Uppercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Space).unwrap(),
                TokenRegister::Lowercase,
                None, None
            ),
            // Space,
            Target::new(
                &symbols.get(&SymbolsRegister::OpenParentheses).unwrap(),
                TokenRegister::Lowercase,
                None, None
            ),
            // OpenParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::CloseParentheses).unwrap(),
                TokenRegister::Lowercase,
                None, None
            ),
            // CloseParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::DoubleCuotes).unwrap(),
                TokenRegister::Lowercase,
                None, None
            ),
            // DoubleCuotes,
            Target::new(
                &symbols.get(&SymbolsRegister::Plus).unwrap(),
                TokenRegister::Lowercase,
                None, None
            ),
            // Plus,
            Target::new(
                &symbols.get(&SymbolsRegister::Hyphen).unwrap(),
                TokenRegister::Lowercase,
                None, None
            ),
            // Hyphen,
            Target::new(
                &symbols.get(&SymbolsRegister::Asterisk).unwrap(),
                TokenRegister::Lowercase,
                None, None
            ),
            // Asterisk,
            Target::new(
                &symbols.get(&SymbolsRegister::Slash).unwrap(),
                TokenRegister::Lowercase,
                None, None
            ),
            // Slash,
            Target::new(
                &symbols.get(&SymbolsRegister::Comma).unwrap(),
                TokenRegister::Lowercase,
                None, None
            ),
            // Comma,
            Target::new(
                &symbols.get(&SymbolsRegister::Semicolon).unwrap(),
                TokenRegister::Lowercase,
                None, None
            ),
            // Semicolon,
            Target::new(
                &symbols.get(&SymbolsRegister::Dot).unwrap(),
                TokenRegister::Lowercase,
                None, None
            ),
        ],
        Target::new(
            &symbols.get(&SymbolsRegister::Error).unwrap(),
            TokenRegister::Lowercase,
            None, None
        ),
        // Error,
    ), Connection::new(
        &symbols.get(&SymbolsRegister::Uppercase).unwrap(),
        vec![Target::new(
                &symbols.get(&SymbolsRegister::Digits).unwrap(),
                TokenRegister::Uppercase,
                None, None
            ),
            // Digits,
            Target::new(
                &symbols.get(&SymbolsRegister::Lowercase).unwrap(),
                TokenRegister::Uppercase,
                None, None
            ),
            // Lowercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Uppercase).unwrap(),
                TokenRegister::Pending,
                None, None
            ),
            // Uppercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Space).unwrap(),
                TokenRegister::Uppercase,
                None, None
            ),
            // Space,
            Target::new(
                &symbols.get(&SymbolsRegister::OpenParentheses).unwrap(),
                TokenRegister::Uppercase,
                None, None
            ),
            // OpenParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::CloseParentheses).unwrap(),
                TokenRegister::Uppercase,
                None, None
            ),
            // CloseParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::DoubleCuotes).unwrap(),
                TokenRegister::Uppercase,
                None, None
            ),
            // DoubleCuotes,
            Target::new(
                &symbols.get(&SymbolsRegister::Plus).unwrap(),
                TokenRegister::Uppercase,
                None, None
            ),
            // Plus,
            Target::new(
                &symbols.get(&SymbolsRegister::Hyphen).unwrap(),
                TokenRegister::Uppercase,
                None, None
            ),
            // Hyphen,
            Target::new(
                &symbols.get(&SymbolsRegister::Asterisk).unwrap(),
                TokenRegister::Uppercase,
                None, None
            ),
            // Asterisk,
            Target::new(
                &symbols.get(&SymbolsRegister::Slash).unwrap(),
                TokenRegister::Uppercase,
                None, None
            ),
            // Slash,
            Target::new(
                &symbols.get(&SymbolsRegister::Comma).unwrap(),
                TokenRegister::Uppercase,
                None, None
            ),
            // Comma,
            Target::new(
                &symbols.get(&SymbolsRegister::Semicolon).unwrap(),
                TokenRegister::Uppercase,
                None, None
            ),
            // Semicolon,
            Target::new(
                &symbols.get(&SymbolsRegister::Dot).unwrap(),
                TokenRegister::Uppercase,
                None, None
            ),
        ],
        Target::new(
            &symbols.get(&SymbolsRegister::Error).unwrap(),
            TokenRegister::Uppercase,
            None, None
        ),
        // Error,
    ), Connection::new(
        &symbols.get(&SymbolsRegister::Space).unwrap(),
        vec![Target::new(
                &symbols.get(&SymbolsRegister::Digits).unwrap(),
                TokenRegister::Space,
                None, None
            ),
            // Digits,
            Target::new(
                &symbols.get(&SymbolsRegister::Lowercase).unwrap(),
                TokenRegister::Space,
                None, None
            ),
            // Lowercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Uppercase).unwrap(),
                TokenRegister::Space,
                None, None
            ),
            // Uppercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Space).unwrap(),
                TokenRegister::Space,
                None, None
            ),
            // Space,
            Target::new(
                &symbols.get(&SymbolsRegister::OpenParentheses).unwrap(),
                TokenRegister::Space,
                None, None
            ),
            // OpenParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::CloseParentheses).unwrap(),
                TokenRegister::Space,
                None, None
            ),
            // CloseParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::DoubleCuotes).unwrap(),
                TokenRegister::Space,
                None, None
            ),
            // DoubleCuotes,
            Target::new(
                &symbols.get(&SymbolsRegister::Plus).unwrap(),
                TokenRegister::Space,
                None, None
            ),
            // Plus,
            Target::new(
                &symbols.get(&SymbolsRegister::Hyphen).unwrap(),
                TokenRegister::Space,
                None, None
            ),
            // Hyphen,
            Target::new(
                &symbols.get(&SymbolsRegister::Asterisk).unwrap(),
                TokenRegister::Space,
                None, None
            ),
            // Asterisk,
            Target::new(
                &symbols.get(&SymbolsRegister::Slash).unwrap(),
                TokenRegister::Space,
                None, None
            ),
            // Slash,
            Target::new(
                &symbols.get(&SymbolsRegister::Comma).unwrap(),
                TokenRegister::Space,
                None, None
            ),
            // Comma,
            Target::new(
                &symbols.get(&SymbolsRegister::Semicolon).unwrap(),
                TokenRegister::Space,
                None, None
            ),
            // Semicolon,
            Target::new(
                &symbols.get(&SymbolsRegister::Dot).unwrap(),
                TokenRegister::Space,
                None, None
            ),
        ],
        Target::new(
            &symbols.get(&SymbolsRegister::Error).unwrap(),
            TokenRegister::Space,
            None, None
        ),
        // Error,
    ), Connection::new(
        &symbols.get(&SymbolsRegister::OpenParentheses).unwrap(),
        vec![Target::new(
                &symbols.get(&SymbolsRegister::Digits).unwrap(),
                TokenRegister::OpenParentheses,
                None, None
            ),
            // Digits,
            Target::new(
                &symbols.get(&SymbolsRegister::Lowercase).unwrap(),
                TokenRegister::OpenParentheses,
                None, None
            ),
            // Lowercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Uppercase).unwrap(),
                TokenRegister::OpenParentheses,
                None, None
            ),
            // Uppercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Space).unwrap(),
                TokenRegister::OpenParentheses,
                None, None
            ),
            // Space,
            Target::new(
                &symbols.get(&SymbolsRegister::OpenParentheses).unwrap(),
                TokenRegister::OpenParentheses,
                None, None
            ),
            // OpenParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::CloseParentheses).unwrap(),
                TokenRegister::OpenParentheses,
                None, None
            ),
            // CloseParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::DoubleCuotes).unwrap(),
                TokenRegister::OpenParentheses,
                None, None
            ),
            // DoubleCuotes,
            Target::new(
                &symbols.get(&SymbolsRegister::Plus).unwrap(),
                TokenRegister::OpenParentheses,
                None, None
            ),
            // Plus,
            Target::new(
                &symbols.get(&SymbolsRegister::Hyphen).unwrap(),
                TokenRegister::OpenParentheses,
                None, None
            ),
            // Hyphen,
            Target::new(
                &symbols.get(&SymbolsRegister::Asterisk).unwrap(),
                TokenRegister::OpenParentheses,
                None, None
            ),
            // Asterisk,
            Target::new(
                &symbols.get(&SymbolsRegister::Slash).unwrap(),
                TokenRegister::OpenParentheses,
                None, None
            ),
            // Slash,
            Target::new(
                &symbols.get(&SymbolsRegister::Comma).unwrap(),
                TokenRegister::OpenParentheses,
                None, None
            ),
            // Comma,
            Target::new(
                &symbols.get(&SymbolsRegister::Semicolon).unwrap(),
                TokenRegister::OpenParentheses,
                None, None
            ),
            // Semicolon,
            Target::new(
                &symbols.get(&SymbolsRegister::Dot).unwrap(),
                TokenRegister::OpenParentheses,
                None, None
            ),
        ],
        Target::new(
            &symbols.get(&SymbolsRegister::Error).unwrap(),
            TokenRegister::OpenParentheses,
            None, None
        ),
        // Error,
    ), Connection::new(
        &symbols.get(&SymbolsRegister::CloseParentheses).unwrap(),
        vec![Target::new(
                &symbols.get(&SymbolsRegister::Digits).unwrap(),
                TokenRegister::CloseParentheses,
                None, None
            ),
            // Digits,
            Target::new(
                &symbols.get(&SymbolsRegister::Lowercase).unwrap(),
                TokenRegister::CloseParentheses,
                None, None
            ),
            // Lowercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Uppercase).unwrap(),
                TokenRegister::CloseParentheses,
                None, None
            ),
            // Uppercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Space).unwrap(),
                TokenRegister::CloseParentheses,
                None, None
            ),
            // Space,
            Target::new(
                &symbols.get(&SymbolsRegister::OpenParentheses).unwrap(),
                TokenRegister::CloseParentheses,
                None, None
            ),
            // OpenParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::CloseParentheses).unwrap(),
                TokenRegister::CloseParentheses,
                None, None
            ),
            // CloseParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::DoubleCuotes).unwrap(),
                TokenRegister::CloseParentheses,
                None, None
            ),
            // DoubleCuotes,
            Target::new(
                &symbols.get(&SymbolsRegister::Plus).unwrap(),
                TokenRegister::CloseParentheses,
                None, None
            ),
            // Plus,
            Target::new(
                &symbols.get(&SymbolsRegister::Hyphen).unwrap(),
                TokenRegister::CloseParentheses,
                None, None
            ),
            // Hyphen,
            Target::new(
                &symbols.get(&SymbolsRegister::Asterisk).unwrap(),
                TokenRegister::CloseParentheses,
                None, None
            ),
            // Asterisk,
            Target::new(
                &symbols.get(&SymbolsRegister::Slash).unwrap(),
                TokenRegister::CloseParentheses,
                None, None
            ),
            // Slash,
            Target::new(
                &symbols.get(&SymbolsRegister::Comma).unwrap(),
                TokenRegister::CloseParentheses,
                None, None
            ),
            // Comma,
            Target::new(
                &symbols.get(&SymbolsRegister::Semicolon).unwrap(),
                TokenRegister::CloseParentheses,
                None, None
            ),
            // Semicolon,
            Target::new(
                &symbols.get(&SymbolsRegister::Dot).unwrap(),
                TokenRegister::CloseParentheses,
                None, None
            ),
        ],
        Target::new(
            &symbols.get(&SymbolsRegister::Error).unwrap(),
            TokenRegister::CloseParentheses,
            None, None
        ),
        // Error,
    ), Connection::new(
        &symbols.get(&SymbolsRegister::DoubleCuotes).unwrap(),
        vec![Target::new(
                &symbols.get(&SymbolsRegister::Digits).unwrap(),
                TokenRegister::DoubleCuotes,
                None, None
            ),
            // Digits,
            Target::new(
                &symbols.get(&SymbolsRegister::Lowercase).unwrap(),
                TokenRegister::DoubleCuotes,
                None, None
            ),
            // Lowercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Uppercase).unwrap(),
                TokenRegister::DoubleCuotes,
                None, None
            ),
            // Uppercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Space).unwrap(),
                TokenRegister::DoubleCuotes,
                None, None
            ),
            // Space,
            Target::new(
                &symbols.get(&SymbolsRegister::OpenParentheses).unwrap(),
                TokenRegister::DoubleCuotes,
                None, None
            ),
            // OpenParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::CloseParentheses).unwrap(),
                TokenRegister::DoubleCuotes,
                None, None
            ),
            // CloseParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::DoubleCuotes).unwrap(),
                TokenRegister::DoubleCuotes,
                None, None
            ),
            // DoubleCuotes,
            Target::new(
                &symbols.get(&SymbolsRegister::Plus).unwrap(),
                TokenRegister::DoubleCuotes,
                None, None
            ),
            // Plus,
            Target::new(
                &symbols.get(&SymbolsRegister::Hyphen).unwrap(),
                TokenRegister::DoubleCuotes,
                None, None
            ),
            // Hyphen,
            Target::new(
                &symbols.get(&SymbolsRegister::Asterisk).unwrap(),
                TokenRegister::DoubleCuotes,
                None, None
            ),
            // Asterisk,
            Target::new(
                &symbols.get(&SymbolsRegister::Slash).unwrap(),
                TokenRegister::DoubleCuotes,
                None, None
            ),
            // Slash,
            Target::new(
                &symbols.get(&SymbolsRegister::Comma).unwrap(),
                TokenRegister::DoubleCuotes,
                None, None
            ),
            // Comma,
            Target::new(
                &symbols.get(&SymbolsRegister::Semicolon).unwrap(),
                TokenRegister::DoubleCuotes,
                None, None
            ),
            // Semicolon,
            Target::new(
                &symbols.get(&SymbolsRegister::Dot).unwrap(),
                TokenRegister::DoubleCuotes,
                None, None
            ),
        ],
        Target::new(
            &symbols.get(&SymbolsRegister::Error).unwrap(),
            TokenRegister::DoubleCuotes,
            None, None
        ),
        // Error,
    ), Connection::new(
        &symbols.get(&SymbolsRegister::Plus).unwrap(),
        vec![Target::new(
                &symbols.get(&SymbolsRegister::Digits).unwrap(),
                TokenRegister::Add,
                None, None
            ),
            // Digits,
            Target::new(
                &symbols.get(&SymbolsRegister::Lowercase).unwrap(),
                TokenRegister::Add,
                None, None
            ),
            // Lowercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Uppercase).unwrap(),
                TokenRegister::Add,
                None, None
            ),
            // Uppercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Space).unwrap(),
                TokenRegister::Add,
                None, None
            ),
            // Space,
            Target::new(
                &symbols.get(&SymbolsRegister::OpenParentheses).unwrap(),
                TokenRegister::Add,
                None, None
            ),
            // OpenParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::CloseParentheses).unwrap(),
                TokenRegister::Add,
                None, None
            ),
            // CloseParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::DoubleCuotes).unwrap(),
                TokenRegister::Add,
                None, None
            ),
            // DoubleCuotes,
            Target::new(
                &symbols.get(&SymbolsRegister::Plus).unwrap(),
                TokenRegister::Add,
                None, None
            ),
            // Plus,
            Target::new(
                &symbols.get(&SymbolsRegister::Hyphen).unwrap(),
                TokenRegister::Add,
                None, None
            ),
            // Hyphen,
            Target::new(
                &symbols.get(&SymbolsRegister::Asterisk).unwrap(),
                TokenRegister::Add,
                None, None
            ),
            // Asterisk,
            Target::new(
                &symbols.get(&SymbolsRegister::Slash).unwrap(),
                TokenRegister::Add,
                None, None
            ),
            // Slash,
            Target::new(
                &symbols.get(&SymbolsRegister::Comma).unwrap(),
                TokenRegister::Add,
                None, None
            ),
            // Comma,
            Target::new(
                &symbols.get(&SymbolsRegister::Semicolon).unwrap(),
                TokenRegister::Add,
                None, None
            ),
            // Semicolon,
            Target::new(
                &symbols.get(&SymbolsRegister::Dot).unwrap(),
                TokenRegister::Add,
                None, None
            ),
        ],
        Target::new(
            &symbols.get(&SymbolsRegister::Error).unwrap(),
            TokenRegister::Add,
            None, None
        ),
        // Error,
    ), Connection::new(
        &symbols.get(&SymbolsRegister::Hyphen).unwrap(),
        vec![Target::new(
                &symbols.get(&SymbolsRegister::Digits).unwrap(),
                TokenRegister::Substract,
                None, None
            ),
            // Digits,
            Target::new(
                &symbols.get(&SymbolsRegister::Lowercase).unwrap(),
                TokenRegister::Substract,
                None, None
            ),
            // Lowercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Uppercase).unwrap(),
                TokenRegister::Substract,
                None, None
            ),
            // Uppercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Space).unwrap(),
                TokenRegister::Substract,
                None, None
            ),
            // Space,
            Target::new(
                &symbols.get(&SymbolsRegister::OpenParentheses).unwrap(),
                TokenRegister::Substract,
                None, None
            ),
            // OpenParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::CloseParentheses).unwrap(),
                TokenRegister::Substract,
                None, None
            ),
            // CloseParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::DoubleCuotes).unwrap(),
                TokenRegister::Substract,
                None, None
            ),
            // DoubleCuotes,
            Target::new(
                &symbols.get(&SymbolsRegister::Plus).unwrap(),
                TokenRegister::Substract,
                None, None
            ),
            // Plus,
            Target::new(
                &symbols.get(&SymbolsRegister::Hyphen).unwrap(),
                TokenRegister::Substract,
                None, None
            ),
            // Hyphen,
            Target::new(
                &symbols.get(&SymbolsRegister::Asterisk).unwrap(),
                TokenRegister::Substract,
                None, None
            ),
            // Asterisk,
            Target::new(
                &symbols.get(&SymbolsRegister::Slash).unwrap(),
                TokenRegister::Substract,
                None, None
            ),
            // Slash,
            Target::new(
                &symbols.get(&SymbolsRegister::Comma).unwrap(),
                TokenRegister::Substract,
                None, None
            ),
            // Comma,
            Target::new(
                &symbols.get(&SymbolsRegister::Semicolon).unwrap(),
                TokenRegister::Substract,
                None, None
            ),
            // Semicolon,
            Target::new(
                &symbols.get(&SymbolsRegister::Dot).unwrap(),
                TokenRegister::Substract,
                None, None
            ),
        ],
        Target::new(
            &symbols.get(&SymbolsRegister::Error).unwrap(),
            TokenRegister::Substract,
            None, None
        ),
        // Error,
    ), Connection::new(
        &symbols.get(&SymbolsRegister::Asterisk).unwrap(),
        vec![Target::new(
                &symbols.get(&SymbolsRegister::Digits).unwrap(),
                TokenRegister::Multiplier,
                None, None
            ),
            // Digits,
            Target::new(
                &symbols.get(&SymbolsRegister::Lowercase).unwrap(),
                TokenRegister::Multiplier,
                None, None
            ),
            // Lowercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Uppercase).unwrap(),
                TokenRegister::Multiplier,
                None, None
            ),
            // Uppercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Space).unwrap(),
                TokenRegister::Multiplier,
                None, None
            ),
            // Space,
            Target::new(
                &symbols.get(&SymbolsRegister::OpenParentheses).unwrap(),
                TokenRegister::Multiplier,
                None, None
            ),
            // OpenParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::CloseParentheses).unwrap(),
                TokenRegister::Multiplier,
                None, None
            ),
            // CloseParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::DoubleCuotes).unwrap(),
                TokenRegister::Multiplier,
                None, None
            ),
            // DoubleCuotes,
            Target::new(
                &symbols.get(&SymbolsRegister::Plus).unwrap(),
                TokenRegister::Multiplier,
                None, None
            ),
            // Plus,
            Target::new(
                &symbols.get(&SymbolsRegister::Hyphen).unwrap(),
                TokenRegister::Multiplier,
                None, None
            ),
            // Hyphen,
            Target::new(
                &symbols.get(&SymbolsRegister::Asterisk).unwrap(),
                TokenRegister::Multiplier,
                None, None
            ),
            // Asterisk,
            Target::new(
                &symbols.get(&SymbolsRegister::Slash).unwrap(),
                TokenRegister::Multiplier,
                None, None
            ),
            // Slash,
            Target::new(
                &symbols.get(&SymbolsRegister::Comma).unwrap(),
                TokenRegister::Multiplier,
                None, None
            ),
            // Comma,
            Target::new(
                &symbols.get(&SymbolsRegister::Semicolon).unwrap(),
                TokenRegister::Multiplier,
                None, None
            ),
            // Semicolon,
            Target::new(
                &symbols.get(&SymbolsRegister::Dot).unwrap(),
                TokenRegister::Multiplier,
                None, None
            ),
        ],
        Target::new(
            &symbols.get(&SymbolsRegister::Error).unwrap(),
            TokenRegister::Multiplier,
            None, None
        ),
        // Error,
    ), Connection::new(
        &symbols.get(&SymbolsRegister::Slash).unwrap(),
        vec![Target::new(
                &symbols.get(&SymbolsRegister::Digits).unwrap(),
                TokenRegister::Divide,
                None, None
            ),
            // Digits,
            Target::new(
                &symbols.get(&SymbolsRegister::Lowercase).unwrap(),
                TokenRegister::Divide,
                None, None
            ),
            // Lowercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Uppercase).unwrap(),
                TokenRegister::Divide,
                None, None
            ),
            // Uppercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Space).unwrap(),
                TokenRegister::Divide,
                None, None
            ),
            // Space,
            Target::new(
                &symbols.get(&SymbolsRegister::OpenParentheses).unwrap(),
                TokenRegister::Divide,
                None, None
            ),
            // OpenParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::CloseParentheses).unwrap(),
                TokenRegister::Divide,
                None, None
            ),
            // CloseParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::DoubleCuotes).unwrap(),
                TokenRegister::Divide,
                None, None
            ),
            // DoubleCuotes,
            Target::new(
                &symbols.get(&SymbolsRegister::Plus).unwrap(),
                TokenRegister::Divide,
                None, None
            ),
            // Plus,
            Target::new(
                &symbols.get(&SymbolsRegister::Hyphen).unwrap(),
                TokenRegister::Divide,
                None, None
            ),
            // Hyphen,
            Target::new(
                &symbols.get(&SymbolsRegister::Asterisk).unwrap(),
                TokenRegister::Divide,
                None, None
            ),
            // Asterisk,
            Target::new(
                &symbols.get(&SymbolsRegister::Slash).unwrap(),
                TokenRegister::Divide,
                None, None
            ),
            // Slash,
            Target::new(
                &symbols.get(&SymbolsRegister::Comma).unwrap(),
                TokenRegister::Divide,
                None, None
            ),
            // Comma,
            Target::new(
                &symbols.get(&SymbolsRegister::Semicolon).unwrap(),
                TokenRegister::Divide,
                None, None
            ),
            // Semicolon,
            Target::new(
                &symbols.get(&SymbolsRegister::Dot).unwrap(),
                TokenRegister::Divide,
                None, None
            ),
        ],
        Target::new(
            &symbols.get(&SymbolsRegister::Error).unwrap(),
            TokenRegister::Divide,
            None, None
        ),
        // Error,
    ), Connection::new(
        &symbols.get(&SymbolsRegister::Comma).unwrap(),
        vec![Target::new(
                &symbols.get(&SymbolsRegister::Digits).unwrap(),
                TokenRegister::Comma,
                None, None
            ),
            // Digits,
            Target::new(
                &symbols.get(&SymbolsRegister::Lowercase).unwrap(),
                TokenRegister::Comma,
                None, None
            ),
            // Lowercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Uppercase).unwrap(),
                TokenRegister::Comma,
                None, None
            ),
            // Uppercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Space).unwrap(),
                TokenRegister::Comma,
                None, None
            ),
            // Space,
            Target::new(
                &symbols.get(&SymbolsRegister::OpenParentheses).unwrap(),
                TokenRegister::Comma,
                None, None
            ),
            // OpenParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::CloseParentheses).unwrap(),
                TokenRegister::Comma,
                None, None
            ),
            // CloseParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::DoubleCuotes).unwrap(),
                TokenRegister::Comma,
                None, None
            ),
            // DoubleCuotes,
            Target::new(
                &symbols.get(&SymbolsRegister::Plus).unwrap(),
                TokenRegister::Comma,
                None, None
            ),
            // Plus,
            Target::new(
                &symbols.get(&SymbolsRegister::Hyphen).unwrap(),
                TokenRegister::Comma,
                None, None
            ),
            // Hyphen,
            Target::new(
                &symbols.get(&SymbolsRegister::Asterisk).unwrap(),
                TokenRegister::Comma,
                None, None
            ),
            // Asterisk,
            Target::new(
                &symbols.get(&SymbolsRegister::Slash).unwrap(),
                TokenRegister::Comma,
                None, None
            ),
            // Slash,
            Target::new(
                &symbols.get(&SymbolsRegister::Comma).unwrap(),
                TokenRegister::Comma,
                None, None
            ),
            // Comma,
            Target::new(
                &symbols.get(&SymbolsRegister::Semicolon).unwrap(),
                TokenRegister::Comma,
                None, None
            ),
            // Semicolon,
            Target::new(
                &symbols.get(&SymbolsRegister::Dot).unwrap(),
                TokenRegister::Comma,
                None, None
            ),
        ],
        Target::new(
            &symbols.get(&SymbolsRegister::Error).unwrap(),
            TokenRegister::Comma,
            None, None
        ),
        // Error,
    ), Connection::new(
        &symbols.get(&SymbolsRegister::Semicolon).unwrap(),
        vec![Target::new(
                &symbols.get(&SymbolsRegister::Digits).unwrap(),
                TokenRegister::Semicolon,
                None, None
            ),
            // Digits,
            Target::new(
                &symbols.get(&SymbolsRegister::Lowercase).unwrap(),
                TokenRegister::Semicolon,
                None, None
            ),
            // Lowercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Uppercase).unwrap(),
                TokenRegister::Semicolon,
                None, None
            ),
            // Uppercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Space).unwrap(),
                TokenRegister::Semicolon,
                None, None
            ),
            // Space,
            Target::new(
                &symbols.get(&SymbolsRegister::OpenParentheses).unwrap(),
                TokenRegister::Semicolon,
                None, None
            ),
            // OpenParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::CloseParentheses).unwrap(),
                TokenRegister::Semicolon,
                None, None
            ),
            // CloseParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::DoubleCuotes).unwrap(),
                TokenRegister::Semicolon,
                None, None
            ),
            // DoubleCuotes,
            Target::new(
                &symbols.get(&SymbolsRegister::Plus).unwrap(),
                TokenRegister::Semicolon,
                None, None
            ),
            // Plus,
            Target::new(
                &symbols.get(&SymbolsRegister::Hyphen).unwrap(),
                TokenRegister::Semicolon,
                None, None
            ),
            // Hyphen,
            Target::new(
                &symbols.get(&SymbolsRegister::Asterisk).unwrap(),
                TokenRegister::Semicolon,
                None, None
            ),
            // Asterisk,
            Target::new(
                &symbols.get(&SymbolsRegister::Slash).unwrap(),
                TokenRegister::Semicolon,
                None, None
            ),
            // Slash,
            Target::new(
                &symbols.get(&SymbolsRegister::Comma).unwrap(),
                TokenRegister::Semicolon,
                None, None
            ),
            // Comma,
            Target::new(
                &symbols.get(&SymbolsRegister::Semicolon).unwrap(),
                TokenRegister::Semicolon,
                None, None
            ),
            // Semicolon,
            Target::new(
                &symbols.get(&SymbolsRegister::Dot).unwrap(),
                TokenRegister::Semicolon,
                None, None
            ),
        ],
        Target::new(
            &symbols.get(&SymbolsRegister::Error).unwrap(),
            TokenRegister::Semicolon,
            None, None
        ),
        // Error,
    ), Connection::new(
        &symbols.get(&SymbolsRegister::Dot).unwrap(),
        vec![Target::new(
                &symbols.get(&SymbolsRegister::Double).unwrap(),
                TokenRegister::Pending,
                Some(String::from(r"\d")), Some(CompareType::Regex)
            ),
            // Digits,
            Target::new(
                &symbols.get(&SymbolsRegister::Lowercase).unwrap(),
                TokenRegister::Dot,
                None, None
            ),
            // Lowercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Uppercase).unwrap(),
                TokenRegister::Dot,
                None, None
            ),
            // Uppercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Space).unwrap(),
                TokenRegister::Dot,
                None, None
            ),
            // Space,
            Target::new(
                &symbols.get(&SymbolsRegister::OpenParentheses).unwrap(),
                TokenRegister::Dot,
                None, None
            ),
            // OpenParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::CloseParentheses).unwrap(),
                TokenRegister::Dot,
                None, None
            ),
            // CloseParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::DoubleCuotes).unwrap(),
                TokenRegister::Dot,
                None, None
            ),
            // DoubleCuotes,
            Target::new(
                &symbols.get(&SymbolsRegister::Plus).unwrap(),
                TokenRegister::Dot,
                None, None
            ),
            // Plus,
            Target::new(
                &symbols.get(&SymbolsRegister::Hyphen).unwrap(),
                TokenRegister::Dot,
                None, None
            ),
            // Hyphen,
            Target::new(
                &symbols.get(&SymbolsRegister::Asterisk).unwrap(),
                TokenRegister::Dot,
                None, None
            ),
            // Asterisk,
            Target::new(
                &symbols.get(&SymbolsRegister::Slash).unwrap(),
                TokenRegister::Dot,
                None, None
            ),
            // Slash,
            Target::new(
                &symbols.get(&SymbolsRegister::Comma).unwrap(),
                TokenRegister::Dot,
                None, None
            ),
            // Comma,
            Target::new(
                &symbols.get(&SymbolsRegister::Semicolon).unwrap(),
                TokenRegister::Dot,
                None, None
            ),
            // Semicolon,
            Target::new(
                &symbols.get(&SymbolsRegister::Dot).unwrap(),
                TokenRegister::Dot,
                None, None
            ),
        ],
        Target::new(
            &symbols.get(&SymbolsRegister::Error).unwrap(),
            TokenRegister::Dot,
            None, None
        ),
        // Error,
    ), Connection::new(
        &symbols.get(&SymbolsRegister::Double).unwrap(),
        vec![Target::new(
                &symbols.get(&SymbolsRegister::Double).unwrap(),
                TokenRegister::Pending,
                Some(String::from(r"\d")), Some(CompareType::Regex)
            ),
            // Digits,
            Target::new(
                &symbols.get(&SymbolsRegister::Lowercase).unwrap(),
                TokenRegister::Double,
                None, None
            ),
            // Lowercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Uppercase).unwrap(),
                TokenRegister::Double,
                None, None
            ),
            // Uppercase,
            Target::new(
                &symbols.get(&SymbolsRegister::Space).unwrap(),
                TokenRegister::Double,
                None, None
            ),
            // Space,
            Target::new(
                &symbols.get(&SymbolsRegister::OpenParentheses).unwrap(),
                TokenRegister::Double,
                None, None
            ),
            // OpenParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::CloseParentheses).unwrap(),
                TokenRegister::Double,
                None, None
            ),
            // CloseParentheses,
            Target::new(
                &symbols.get(&SymbolsRegister::DoubleCuotes).unwrap(),
                TokenRegister::Double,
                None, None
            ),
            // DoubleCuotes,
            Target::new(
                &symbols.get(&SymbolsRegister::Plus).unwrap(),
                TokenRegister::Double,
                None, None
            ),
            // Plus,
            Target::new(
                &symbols.get(&SymbolsRegister::Hyphen).unwrap(),
                TokenRegister::Double,
                None, None
            ),
            // Hyphen,
            Target::new(
                &symbols.get(&SymbolsRegister::Asterisk).unwrap(),
                TokenRegister::Double,
                None, None
            ),
            // Asterisk,
            Target::new(
                &symbols.get(&SymbolsRegister::Slash).unwrap(),
                TokenRegister::Double,
                None, None
            ),
            // Slash,
            Target::new(
                &symbols.get(&SymbolsRegister::Comma).unwrap(),
                TokenRegister::Double,
                None, None
            ),
            // Comma,
            Target::new(
                &symbols.get(&SymbolsRegister::Semicolon).unwrap(),
                TokenRegister::Double,
                None, None
            ),
            // Semicolon,
            Target::new(
                &symbols.get(&SymbolsRegister::Dot).unwrap(),
                TokenRegister::Double,
                None, None
            ),
        ],
        Target::new(
            &symbols.get(&SymbolsRegister::Error).unwrap(),
            TokenRegister::Double,
            None, None
        ),
        // Error,
    ),];

    let syntax_endline: Vec<SyntaxTree> = vec![SyntaxTree{
        token: TokenRegister::Semicolon,
        next_tokens: None
    }];

    let syntax_endfunc: Vec<SyntaxTree> = vec![SyntaxTree{
        token: TokenRegister::CloseParentheses,
        next_tokens: Some(syntax_endline.clone())
    }];

    let syntax_word: Vec<SyntaxTree> = vec![SyntaxTree{
        token: TokenRegister::DoubleCuotes,
        next_tokens: Some(syntax_endfunc.clone())
    }];

    let syntax_trees: Vec<SyntaxTree> = vec![SyntaxTree{
        token: TokenRegister::Print,
        next_tokens: Some(vec![SyntaxTree{
            token: TokenRegister::OpenParentheses,
            next_tokens:Some(vec![SyntaxTree{
                token: TokenRegister::DoubleCuotes,
                next_tokens: Some(vec![SyntaxTree{
                    token: TokenRegister::Uppercase,
                    next_tokens: Some(syntax_word.clone())
                }, SyntaxTree{
                    token: TokenRegister::Lowercase,
                    next_tokens: Some(syntax_word.clone())
                }])
            }, SyntaxTree{
                token: TokenRegister::Integer,
                next_tokens: Some(syntax_endfunc.clone())
            }])
        }])
    }, SyntaxTree{
        token: TokenRegister::Println,
        next_tokens: Some(vec![SyntaxTree{
            token: TokenRegister::OpenParentheses,
            next_tokens:Some(vec![SyntaxTree{
                token: TokenRegister::DoubleCuotes,
                next_tokens: Some(vec![SyntaxTree{
                    token: TokenRegister::Uppercase,
                    next_tokens: Some(syntax_word.clone())
                }, SyntaxTree{
                    token: TokenRegister::Lowercase,
                    next_tokens: Some(syntax_word.clone())
                }])
            }])
        }])
    }, SyntaxTree{
        token: TokenRegister::Addfn,
        next_tokens: Some(vec![SyntaxTree{
            token: TokenRegister::OpenParentheses,
            next_tokens:Some(vec![SyntaxTree{
                token: TokenRegister::Integer,
                next_tokens: Some(syntax_endfunc.clone())
            }, SyntaxTree{
                token: TokenRegister::Double,
                next_tokens: Some(syntax_endfunc.clone())
            }])
        }])
    }, SyntaxTree{
        token: TokenRegister::Integer,
        next_tokens: Some(vec![SyntaxTree{
            token: TokenRegister::Add,
            next_tokens:Some(vec![SyntaxTree{
                token: TokenRegister::Integer,
                next_tokens: Some(syntax_endline.clone())
            }])
        }])
    }];

    // println!("{:#?}", syntax_trees);

    let _lex: Vec<TokenContainer> = start(&read_command(), &connections, &syntax_trees);
}
