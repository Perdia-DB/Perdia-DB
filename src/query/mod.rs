use std::{sync::MutexGuard, time::Instant};

use crate::{data::{structure::{Template, Instance}, TEMPLATES, INSTANCES, serialization::{Data, DataType}}, plog, ast::{self, Node}, perr, error::PangError};
use linked_hash_map::LinkedHashMap;
use crate::lexer::data::{Token, TokenMatch};

use self::{backend::{push_template, remove_instance, push_instance}, prop::{create_template_prop, create_select_prop}};

pub mod error;
mod backend;
mod prop;

enum QueryResult {
    Template(Template),
    Instance(Instance),
}

fn exec(ast: Vec<Node>) -> Result<String, PangError> {

    for branch in ast {
        exec_branch(branch)?;
    }

    Ok("".to_string())
}

fn exec_branch(branch: Node) -> Result<Option<QueryResult>, PangError> {
    match branch {
        Node::Statement { variant, context, child } => {

        },
        Node::Shell { outside, inside } => {
            let (token, name, loc) = match *outside {
                Node::Statement { variant, context, child } => {
                    // Token
                    let (token, loc) = match *variant {
                        Node::Token(token, loc) => (token, loc),
                        _ => return Err(PangError::ExecutionError),
                    };
                    // Name
                    let name = match *context {
                        Node::Literal(name, _) => name,
                        _ => return Err(PangError::ExecutionError),
                    };
                    (token, name, loc)
                },
                _ => return Err(PangError::ExecutionError),
            };
            
            match token {
                Token::Select => make_selection(name, inside, loc)?,
                Token::Template => create_template(name, inside, loc)?,
                _ => return Err(PangError::ExecutionError),
            }
        },
        _ => return Err(PangError::ExecutionError),
    }

    todo!()
}

/// Creates a template from a branch
fn create_template(name: String, properties: Vec<Box<Node>>, loc: usize) -> Result<(), PangError> {
    let template = Template::new(name);
    for prop in properties {
        let (name, data) = create_template_prop(*prop)?;
        template.add_data(name, data);
    }
    push_template(template.build(), loc)?;
    Ok(())
}

/// Makes the instances selection and overwrites the values
fn make_selection(name: String, properties: Vec<Box<Node>>, loc: usize) -> Result<(), PangError> {
    let mut instance = remove_instance(name, loc)?;
    for prop in properties {
        let (name, (data, loc)) = create_select_prop(*prop)?;
        instance.overwrite(name, data, loc)?;
    }
    push_instance(instance, loc)?;
    Ok(())
}

/// Queries the data from the backend
fn query_statement(context: Node, child: Option<Box<Node>>, loc: usize) -> Result<QueryResult, PangError> {
    
    todo!()
}

/// Makes a Instance or Template entry in the backend
fn create_statement(context: Node, child: Option<Box<Node>>, loc: usize) -> Result<(), PangError> {
    let name = match context {
        Node::Literal(name, _) => Ok(name),
        _ => Err(PangError::ExecutionError),
    }?;
    match child {
        Some(child) => match *child {
            Node::Statement { variant, context, child: _ } => {
                let context = match *context {
                    Node::Literal(context, _) => Ok(context),
                    _ => Err(PangError::ExecutionError),
                }?;
                match *variant {
                    Node::Token(token, _) => match token {
                        Token::Template => {
                            let template = backend::copy_template(context, loc)?;
                            let instance = Instance::new(name, template);
                            backend::push_instance(instance, loc)?;
                            Ok(())
                        },
                        Token::Instance => {
                            let origin = backend::copy_instance(context, loc)?;
                            let mut instance = Instance::new(name, origin.template);
                            instance.data = origin.data;
                            backend::push_instance(instance, loc)?;
                            Ok(())
                        },
                        _ => Err(PangError::ExecutionError),
                    },
                    _ => Err(PangError::ExecutionError),
                }?;
                Ok(())
            },
            _ => Err(PangError::ExecutionError),
        },
        None => Err(PangError::ExecutionError),
    }?;
    Ok(())
}

/// Deletes data from the backend
fn delete_statement(context: Node, child: Option<Box<Node>>, loc: usize) -> Result<QueryResult, PangError> {
    
    todo!()
}

/// Query the parsed data from memory
pub fn data(lines: Vec<Vec<TokenMatch>>) -> String {
    let now = Instant::now();
    let ast = ast::parse(lines);
    plog!("AST done in: {:?}", now.elapsed());
    match ast {
        Ok(ast) => match exec(ast) {
            Ok(res) => res,
            Err(err) => serde_json::to_string_pretty(&err).unwrap(),
        },
        Err(err) => serde_json::to_string_pretty(&err).unwrap(),
    }
}