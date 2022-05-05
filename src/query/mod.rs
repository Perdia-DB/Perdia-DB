use std::fmt::Display;

use crate::data::{template::Template, TEMPLATES};
use request::RequestError;
use super::parser::lexer::{Token, TokenMatch};

mod request;

#[derive(Copy, Clone, PartialEq)]
enum Block {
    Declaration,
    Statement
}

pub fn declare(lines: Vec<Vec<TokenMatch>>) -> Result<(), RequestError>{
    let mut endings = lines.iter().enumerate()
        .filter(|(_, line)| line.get(0).unwrap().token == Token::End)
        .map(|(index, _)| index)
        .collect::<Vec<usize>>();
    endings.insert(0, 0);

    let mut blocks: Vec<Vec<Vec<TokenMatch>>> = Vec::with_capacity(endings.len());
    for (index, ending) in endings.iter().enumerate() {
        let next = endings.get(index+1);
        if next.is_none() { break; }
        let next = next.unwrap();

        let mut clone = lines.clone();
        blocks.push(clone.drain(*ending..*next).collect())
    }
    // parse a single declaration block.
    for mut block in blocks {
        let first = block.remove(0);
        // Validate statement begin
        if first.len() != 2 { return Err(RequestError::DeclarationError); }
        // Get name of template
        let name = first.get(1).unwrap();
        let mut template = Template::new(name.value.clone());
        // Loop over field declaration lines
        for line in block {
            
            // if the line only has 4 tokens then it has no starting value
            if line.len() == 4 {
                let field = line.get(1).unwrap();
                let data_type = line.get(3).unwrap();
                template = match data_type.token {
                    Token::StringType => template.with_string(field.value.clone(), Some("".to_owned())),
                    Token::IntegerType => template.with_integer(field.value.clone(), Some(0)),
                    Token::FloatType => template.with_float(field.value.clone(), Some(0.0)),
                    _ => { return Err(RequestError::DeclarationError); }
                }
            // if it has 6 tokens it has a starting value
            } else if line.len() == 6 {
                let field = line.get(1).unwrap();
                let data_type = line.get(3).unwrap();
                let starting = line.get(5).unwrap();
                template = match data_type.token {
                    Token::StringType => template.with_string(field.value.clone(), Some(starting.value.clone())),
                    Token::IntegerType => template.with_integer(field.value.clone(), Some(starting.value.clone().parse::<i64>().unwrap())),
                    Token::FloatType => template.with_float(field.value.clone(), Some(starting.value.clone().parse::<f64>().unwrap())),
                    _ => { return Err(RequestError::DeclarationError); }
                }
            // throw an error at any other size
            } else {
                return Err(RequestError::DeclarationError)
            }
        }
        // Push the template onto the static mutex
        let template = template.build();
        let mut mutex = TEMPLATES.lock().unwrap();
        mutex.push(template);
    }

    Ok(())
}

/// Query the parsed data from memory
pub fn data(lines: Vec<Vec<TokenMatch>>) -> Result<String, RequestError> {
    
    // Mark diffrent blocks
    let mut blocks: Vec<Block> = Vec::new();
    for line in &lines {
        blocks.push(match line.get(0).unwrap().token {
            Token::Type => Block::Declaration,
            Token::Name => Block::Declaration,
            Token::End => Block::Declaration,
            _ => Block::Statement,
        });
    }

    let objects = lines.iter()
        .enumerate().filter(|(index, _)| *blocks.get(*index).unwrap() == Block::Declaration)
        .map(|(_, e)| e.clone())
        .collect::<Vec<Vec<TokenMatch>>>();
    let statements = lines.iter()
        .enumerate().filter(|(index, _)| *blocks.get(*index).unwrap() == Block::Statement)
        .map(|(_, e)| e.clone())
        .collect::<Vec<Vec<TokenMatch>>>();
    declare(objects).unwrap();


    Ok("".to_owned())
}