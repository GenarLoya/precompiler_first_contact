pub mod symbols;
pub use symbols::{SymbolsRegister, CompareType, Symbol, get_new_symbol};
pub mod tokens;
pub use tokens::{TokenContainer, TokenRegister, get_resfunc};
pub mod connections;
pub use connections::{Target, Connection, start};
pub mod syntax_tree;
pub use syntax_tree::SyntaxTree;