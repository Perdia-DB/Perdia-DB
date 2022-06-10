use crate::{data::{structure::{Template, Instance}}, ast::{self, Node}, error::PangError};
use serde::{Serialize, Deserialize};
use crate::lexer::data::{Token, TokenMatch};

use self::{backend::{push_template, remove_instance, push_instance}, prop::{create_template_prop, create_select_prop}};

pub mod backend;
mod prop;

#[derive(Serialize, Deserialize)]
enum QueryResult {
    Template(Template),
    Instance(Instance),
}

impl From<Instance> for QueryResult {
    fn from(v: Instance) -> Self {
        QueryResult::Instance(v)
    }
}

impl From<Template> for QueryResult {
    fn from(v: Template) -> Self {
        QueryResult::Template(v)
    }
}

fn exec(ast: Vec<Node>) -> Result<String, PangError> {
    let mut res = Vec::new();
    for branch in ast {
        match exec_branch(branch) {
            Ok(value) => match value {
                Some(value) => res.push(value),
                None => {},
            },
            Err(e) => return Err(e),
        }
    }

    match serde_json::to_string_pretty(&res) {
        Ok(out) => Ok(out),
        Err(_) => Err(PangError::ExecutionError),
    }
}

fn exec_branch(branch: Node) -> Result<Option<Vec<QueryResult>>, PangError> {
    match branch {
        Node::Statement { variant, context, child } => {
            match *variant {
                Node::Token(token, loc) => match token {
                    Token::Query => return Ok(Some(query_statement(*context, child, loc)?)),
                    Token::Create => { 
                        create_statement(*context, child, loc)?; 
                        Ok(None)
                    },
                    Token::Delete => return Ok(Some(delete_statement(*context, child, loc)?)),
                    _ => return Err(PangError::ExecutionError),
                },
                _ => return Err(PangError::ExecutionError),
            }
        },
        Node::Shell { outside, inside } => {
            let (token, name, loc) = match *outside {
                Node::Statement { variant, context, child: _} => {
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
                Token::Select => {
                    make_selection(name, inside, loc)?;
                    Ok(None)
                },
                Token::Template => { 
                    create_template(name, inside, loc)?;
                    Ok(None)
                },
                _ => return Err(PangError::ExecutionError),
            }
        },
        _ => return Err(PangError::ExecutionError),
    }
}

/// Creates a template from a branch
fn create_template(name: String, properties: Vec<Box<Node>>, loc: usize) -> Result<(), PangError> {
    let mut template = Template::new(name);
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
fn query_statement(context: Node, child: Option<Box<Node>>, _loc: usize) -> Result<Vec<QueryResult>, PangError> {
    let name = match context {
        Node::Literal(name, _) => Ok(name),
        Node::Token(token, _) => match token {
            Token::Instance => {
                return Ok(backend::copy_instances().iter().map(|e| e.clone().into()).collect())
            },
            Token::Template => {
                return Ok(backend::copy_templates().iter().map(|e| e.clone().into()).collect())
            },
            _ => Err(PangError::ExecutionError)
        }
        _ => Err(PangError::ExecutionError),
    }?;
    match child {
        Some(child) => match *child {
            Node::Statement { variant: _, context, child: _ } => {
                match *context {
                    Node::Token(token, loc) => match token {
                        Token::Template => {
                            Ok(vec![backend::copy_template(name, loc)?.into()])
                        }
                        Token::Instance => {
                            Ok(vec![backend::copy_instance(name, loc)?.into()])
                        }
                        _ => Err(PangError::ExecutionError),
                    }
                    _ => Err(PangError::ExecutionError),
                }
            },
            _ => Err(PangError::ExecutionError),
        },
        None => Err(PangError::ExecutionError),
    }
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
fn delete_statement(context: Node, child: Option<Box<Node>>, loc: usize) -> Result<Vec<QueryResult>, PangError> {
    let name = match context {
        Node::Literal(name, _) => Ok(name),
        _ => Err(PangError::ExecutionError),
    }?;
    Ok(vec![match child {
        Some(child) => match *child {
            Node::Statement { variant: _, context, child: _ } => {
                match *context {
                    Node::Token(token, _) => match token {
                        Token::Template => {
                            Ok(backend::remove_template(name, loc)?.into())
                        },
                        Token::Instance => {
                            Ok(backend::remove_instance(name, loc)?.into())
                        },
                        _ => Err(PangError::ExecutionError),
                    },
                    _ => Err(PangError::ExecutionError),
                }
            },
            _ => Err(PangError::ExecutionError),
        },
        None => Err(PangError::ExecutionError),
    }?])
}

/// Query the parsed data from memory
pub fn data(lines: Vec<Vec<TokenMatch>>) -> String {
    let ast = ast::parse(lines);
    match ast {
        Ok(ast) => match exec(ast) {
            Ok(res) => res,
            Err(err) => serde_json::to_string_pretty(&err).unwrap(),
        },
        Err(err) => serde_json::to_string_pretty(&err).unwrap(),
    }
}