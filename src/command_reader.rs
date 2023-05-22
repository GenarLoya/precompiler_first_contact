use std::{env, fs};

pub fn read_command() -> String{
    let args: Vec<String> = env::args().collect();

    colour::green_ln!("\n\u{1F7e2} <<<<Reading file...>>>>");
    let file_text = fs::read_to_string(&args[1]).unwrap();

    colour::yellow_ln!("\n\u{1F4d1} File content: {} \n{}", &args[1], file_text);

    file_text
}