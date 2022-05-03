use lazy_static::lazy_static;
use lexer::TokenDefinition;

pub mod lexer;
pub mod tokens;

lazy_static! {
    pub static ref TOKEN_DEFINITIONS: Vec<TokenDefinition> = tokens::initialize();
}

pub fn parse() {
    
}