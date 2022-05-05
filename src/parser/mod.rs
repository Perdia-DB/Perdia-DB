use std::string::ParseError;

use lazy_static::lazy_static;
use lexer::{Token, TokenMatch, TokenDefinition};

pub mod lexer;
pub mod tokens;


lazy_static! {
    pub static ref TOKEN_DEFINITIONS: Vec<TokenDefinition> = tokens::initialize();
}

pub fn parse(source: &String) -> Vec<Vec<TokenMatch>> {
    let mut token_matches: Vec<TokenMatch> = Vec::new();
    TOKEN_DEFINITIONS.iter()
        .for_each(|definition| 
            definition.match_text(&source, &mut token_matches)
        );
    token_matches.sort_by(|a, b| a.start.cmp(&b.start));
    
    let mut lines = Vec::new();
    token_matches.split(|m| m.token == Token::ENDL)
    .collect::<Vec<&[TokenMatch]>>().iter()
    .for_each(|tms| {
        let mut line = Vec::new();
        for tm in *tms {
            let tm = match tm.token {
                Token::Literal => {
                    let value = tm.value.to_string()
                        .strip_prefix("\"").unwrap().to_string()
                        .strip_suffix("\"").unwrap().to_string();
                    let mut ctm = tm.clone();
                    ctm.value = value;
                    ctm
                },
                _ => tm.clone(),
            };
            line.push(tm.clone());
        }
        lines.push(line);
    });
    lines.remove(lines.len()-1);
    lines
}