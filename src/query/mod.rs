use crate::data::{template::Template, TEMPLATES, NEW_TEMPLATE};

use super::parser::lexer::{Token, TokenMatch};

mod queries;

/// Gets thrown if the source is invalid or the parser has trouble doing it's job.
pub struct PangQueryError;

impl std::fmt::Display for PangQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error occurred while parseing query!")
    }
}

impl std::fmt::Debug for PangQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}



trait Executable {
    fn execute(&self, input: Option<Vec<Template>>, arguments: Vec<Argument>) -> Result<Vec<Template>, PangQueryError>;
}

impl Executable for Token {
    fn execute(&self, input: Option<Vec<Template>>, mut arguments: Vec<Argument>) -> Result<Vec<Template>, PangQueryError> {
        match self {
            Token::Create => {
                let name = match arguments.remove(0) {
                    Argument::Literal(value) => value,
                    _ => return Err(PangQueryError),
                };
                let template = Template::instance(name.to_string()).build();
                Ok(vec![template])
            },
            Token::Query => todo!(),
            Token::Put => todo!(),
            Token::Get => todo!(),
            Token::Type => {
                let template: Template = match input {
                    Some(mut input) => {
                        let mut input = input.remove(0);
                        input.name = match arguments.remove(0) {
                            Argument::Literal(value) => Some(value),
                            _ => return Err(PangQueryError)
                        };
                        let mut base = TEMPLATES.iter().filter(|t| t.name == input.name).collect::<Vec<&Template>>();
                        input.data = base.remove(0).data.clone();
                        input
                    },
                    None => {
                        let name = match arguments.get(0).unwrap() {
                            Argument::Literal(value) => value,
                            _ => return Err(PangQueryError)
                        };
                        let builder = Template::new(name.to_string());
                        let mut mutex = NEW_TEMPLATE.lock().unwrap();
                        *mutex = Some(builder.clone());
                        builder.build()
                    },
                };
                Ok(vec![template])
            },
            Token::Name => todo!(),
            Token::Starting => todo!(),
            Token::End => todo!(),
            _ => { return Err(PangQueryError) }
        }
    }
}

enum DataType {
    String,
    Integer,
    Float,
}

enum Argument {
    Literal(String),
    Literals(Vec<String>),
    DataType(DataType),
    Type
}

#[derive(Copy, Clone, PartialEq)]
enum Block {
    Declaration,
    Statement
}

pub fn declarations(lines: Vec<Vec<TokenMatch>>) {
    let endings = lines.iter().enumerate()
        .filter(|(_, line)| line.get(0).unwrap().token == Token::End)
        .map(|(index, _)| index)
        .collect::<Vec<usize>>();
    let blocks: Vec<Vec<Vec<TokenMatch>>> = Vec::with_capacity(endings)
}

/// Query the parsed data from memory
pub fn data(lines: Vec<Vec<TokenMatch>>) -> Result<String, PangQueryError> {
    
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

    let declarations = lines.iter()
        .enumerate().filter(|(index, _)| *blocks.get(*index).unwrap() == Block::Declaration)
        .map(|(_, e)| e.clone())
        .collect::<Vec<Vec<TokenMatch>>>();
    let statements = lines.iter()
        .enumerate().filter(|(index, _)| *blocks.get(*index).unwrap() == Block::Statement)
        .map(|(_, e)| e.clone())
        .collect::<Vec<Vec<TokenMatch>>>();



    Ok("".to_owned())
}