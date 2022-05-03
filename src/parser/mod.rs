use lazy_static::lazy_static;
use lexer::{TokenMatch, TokenDefinition};

pub mod lexer;
pub mod tokens;

lazy_static! {
    pub static ref TOKEN_DEFINITIONS: Vec<TokenDefinition> = tokens::initialize();
}

pub fn parse(source: &String) -> Vec<TokenMatch> {
    let mut token_matches: Vec<TokenMatch> = Vec::new();
    TOKEN_DEFINITIONS.iter()
        .for_each(|definition| 
            definition.match_text(&source, &mut token_matches)
        );
    token_matches.sort_by(|a, b| a.start.cmp(&b.start));
    token_matches
}