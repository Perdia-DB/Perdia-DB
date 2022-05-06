use std::fmt::Display;

use crate::data::{template::Template, TEMPLATES, INSTANCES};
use error::RequestError;
use crate::lexer::data::{Token, TokenMatch};

mod error;

#[derive(Copy, Clone, PartialEq)]
enum Block {
    Declaration,
    Statement
}

pub fn create_template(lines: Vec<Vec<TokenMatch>>) -> Result<Template, RequestError> {
    let first = lines.remove(0);
    // Validate statement begin
    if first.len() != 2 { return Err(RequestError::DeclarationError); }
    // Get name of template
    let name = first.get(1).unwrap();
    let mut template = Template::new(name.value.clone());
    // Loop over field declaration lines
    for line in lines {
        
        // if the line only has 4 tokens then it has no starting value
        if line.len() == 4 {
            let field = line.get(1).unwrap();
            let data_type = line.get(3).unwrap();
            template = match data_type.token {
                Token::StringType => {
                    template.with_string(field.value.clone(), Some("".to_owned()))
                },
                Token::IntegerType => {
                    template.with_integer(field.value.clone(), Some(0))
                },
                Token::FloatType => {
                    template.with_float(field.value.clone(), Some(0.0))
                },
                _ => { return Err(RequestError::DeclarationError); }
            }
        // if it has 6 tokens it has a starting value
        } else if line.len() == 6 {
            let field = line.get(1).unwrap();
            let data_type = line.get(3).unwrap();
            let starting = line.get(5).unwrap();
            template = match data_type.token {
                Token::StringType => {
                    template.with_string(field.value.clone(), Some(starting.value.clone()))
                },
                Token::IntegerType => {
                    template.with_integer(field.value.clone(), Some(starting.value.clone().parse::<i64>().unwrap()))
                },
                Token::FloatType => {
                    template.with_float(field.value.clone(), Some(starting.value.clone().parse::<f64>().unwrap()))
                },
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
    mutex.push(template.clone());

    Ok(template)
}

pub fn parse_statements(mut lines: Vec<Vec<TokenMatch>>) -> Result<Vec<Template>, RequestError> {
    let mut output: Vec<Template> = Vec::new();
    for (index, line) in lines.clone().iter().enumerate() {
        let mut iter = line.iter();
        match iter.next() {
            Some(next) => match next.token {
                Token::Create => match iter.next() {
                    Some(next) => match next.token {
                        Token::Literal => {
                            // name of the instance
                            let name = next.value.clone();
                            match iter.next() {
                                Some(next) => match next.token {
                                    Token::Type => match iter.next() {
                                        Some(next) => {
                                            // name of the template 
                                            let template_name = next.value.clone();
                                            // grab template and make instance
                                            let mutex = TEMPLATES.lock().unwrap();
                                            let mut instance = mutex.iter()
                                                .filter(
                                                    |template| 
                                                    template.name.as_ref().unwrap().clone() == template_name
                                                ).collect::<Vec<&Template>>().get(0).unwrap().clone().clone();
                                            instance.instance = Some(name);
                                            let mut mutex = INSTANCES.lock().unwrap();
                                            mutex.push(instance);
                                        },
                                        None => return Err(RequestError::SyntaxError),
                                    },
                                    _ => return Err(RequestError::SyntaxError),
                                },
                                None => return Err(RequestError::SyntaxError),
                            }
                        },
                        _ => return Err(RequestError::SyntaxError)
                    },
                    None => return Err(RequestError::SyntaxError),
                },
                Token::Query => todo!(),
                // Pull out whole template
                Token::Type => {
                    let start_index = index;
                    let end_index = *lines.iter().enumerate()
                        .filter(|(_, line)| line.get(0).unwrap().token == Token::End)
                        .map(|(index, _)| index).collect::<Vec<usize>>().get(0).unwrap();
                    let template: Vec<Vec<TokenMatch>> = lines.drain(start_index..=end_index).collect();
                    let template = create_template(template)?;
                    output.push(template);
                },
                // Ignore declaration Tokens
                Token::Name => {},
                Token::End => {}
                _ => return Err(RequestError::SyntaxError)
            },
            None => return Err(RequestError::SyntaxError),
        }
    }

    Ok(output)
}

/// Query the parsed data from memory
pub fn data(lines: Vec<Vec<TokenMatch>>) -> Result<String, RequestError> {
    
    parse_statements(lines).unwrap();


    Ok("".to_owned())
}