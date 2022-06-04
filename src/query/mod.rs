use std::{sync::MutexGuard, time::Instant};

use crate::{data::{structure::{Template, Instance}, TEMPLATES, INSTANCES, serialization::{Data, DataType}}, plog, ast::{self, Node}, perr, error::PangError};
use linked_hash_map::LinkedHashMap;
use crate::lexer::data::{Token, TokenMatch};

use self::backend::{push_template, remove_instance, push_instance};

pub mod error;
mod backend;

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
        Node::Literal(_, _) => todo!(),
        Node::Int(_, _) => todo!(),
        Node::Float(_, _) => todo!(),
        Node::Token(_, _) => todo!(),
        Node::Statement { variant, context, child } => todo!(),
        Node::Shell { outside, inside } => {
            let (token, name) = match *outside {
                Node::Literal(_, loc) => return Err(PangError::SyntaxError(loc)),
                Node::Int(_, loc) => return Err(PangError::SyntaxError(loc)),
                Node::Float(_, loc) => return Err(PangError::SyntaxError(loc)),
                Node::Token(_, loc) => return Err(PangError::SyntaxError(loc)),
                Node::Shell { outside, inside } => {
                    return exec_branch(*outside)
                },
                Node::Statement { variant, context, child } => {
                    let (token, token_loc) = match *variant {
                        Node::Literal(_, loc) => return Err(PangError::SyntaxError(loc)),
                        Node::Int(_, loc) => return Err(PangError::SyntaxError(loc)),
                        Node::Float(_, loc) => return Err(PangError::SyntaxError(loc)),
                        Node::Token(token, loc) => (token, loc),
                        Node::Statement { variant, context, child } => {
                            return exec_branch(*variant)
                        },
                        Node::Shell { outside, inside } => {
                            return exec_branch(*outside)
                        },
                    };
                },
            };
        },
    }

    todo!()
}

/// Creates a Variable from the Inside branch of a Template Shell
fn create_template_prop(prop: Node) -> Result<(String, Data), PangError> {
    // Check if node is statement
    match prop {
        Node::Literal(_, loc) => Err(PangError::SyntaxError(loc)),
        Node::Int(_, loc) => Err(PangError::SyntaxError(loc)),
        Node::Float(_, loc) => Err(PangError::SyntaxError(loc)),
        Node::Token(_, loc) => Err(PangError::SyntaxError(loc)),
        Node::Shell { outside, inside } => {
            create_template_prop(*outside)
        },
        Node::Statement { variant, context, child } => {
            // What DataType to expect
            let data_type: DataType = match *variant {
                Node::Literal(_, loc) => return Err(PangError::SyntaxError(loc)),
                Node::Int(_, loc) => return Err(PangError::SyntaxError(loc)),
                Node::Float(_, loc) => return Err(PangError::SyntaxError(loc)),
                Node::Shell { outside, inside } => {
                    return create_template_prop(*outside)
                },
                Node::Statement { variant, context, child } => {
                    return create_template_prop(*variant)
                },

                Node::Token(token, loc) => match token {
                    Token::StringType => DataType::STRING,
                    Token::IntegerType => DataType::INTEGER,
                    Token::FloatType => DataType::FLOAT,
                    _ => return Err(PangError::SyntaxError(loc))
                },
            };

            // Name of the field
            let name: String = match *context {
                Node::Int(_, loc) => Err(PangError::SyntaxError(loc)),
                Node::Float(_, loc) => Err(PangError::SyntaxError(loc)),
                Node::Token(_, loc) => Err(PangError::SyntaxError(loc)),
                Node::Shell { outside, inside } => {
                    return create_template_prop(*outside)
                },
                Node::Statement { variant, context, child } => {
                    return create_template_prop(*variant)
                },
                Node::Literal(string, _) => Ok(string),
            }?;

            // Actual data check if starting value is given
            let data: Data = match child {
                Some(child) => {
                    // Verify that child is a statement
                    match *child {
                        Node::Literal(_, loc) => Err(PangError::SyntaxError(loc)),
                        Node::Int(_, loc) => Err(PangError::SyntaxError(loc)),
                        Node::Float(_, loc) => Err(PangError::SyntaxError(loc)),
                        Node::Token(_, loc) => Err(PangError::SyntaxError(loc)),
                        Node::Shell { outside, inside } => {
                            return create_template_prop(*outside)
                        },
                        Node::Statement { variant, context, child } => {
                            // Validate, that variant is VALUE
                            match *variant {
                                Node::Literal(_, loc) => return Err(PangError::SyntaxError(loc)),
                                Node::Int(_, loc) => return Err(PangError::SyntaxError(loc)),
                                Node::Float(_, loc) => return Err(PangError::SyntaxError(loc)),
                                Node::Shell { outside, inside } => {
                                    return create_template_prop(*outside)
                                },
                                Node::Statement { variant, context, child } => {
                                    return create_template_prop(*variant)
                                },
                
                                Node::Token(token, loc) => match token {
                                    Token::Value => {}
                                    _ => return Err(PangError::SyntaxError(loc))
                                },
                            };
                            
                            // If child is something throw it out the window
                            if child.is_some() {
                                return create_template_prop(*child.unwrap())
                            }

                            // Validate value
                            let data: (Data, usize) = match *context {
                                Node::Token(_, loc) => return Err(PangError::SyntaxError(loc)),
                                Node::Shell { outside, inside } => {
                                    return create_template_prop(*outside)
                                },
                                Node::Statement { variant, context, child } => {
                                    return create_template_prop(*variant)
                                },
                                Node::Literal(string, loc) => (string.into(), loc),
                                Node::Int(int, loc) => (int.into(), loc),
                                Node::Float(float, loc) => (float.into(), loc),
                            };

                            // Validate types
                            if data_type != data.0.data_type {
                                return Err(PangError::TypeMismatch(data.1))
                            }

                            Ok(data.0)
                        },
                    }?;

                    todo!()
                },
                None => match data_type {
                    DataType::STRING => "".into(),
                    DataType::INTEGER => 0.into(),
                    DataType::FLOAT => 0.0.into(),
                },
            };

            return Ok((name, data))
        },
    }
}

/// Creates a template from a branch
fn create_template(name: String, properties: Vec<Node>, loc: usize) -> Result<(), PangError> {
    let template = Template::new(name);
    for prop in properties {
        let (name, data) = create_template_prop(prop)?;
        template.add_data(name, data);
    }
    push_template(template.build(), loc)?;
    Ok(())
}

/// Creates a Variable from the Inside branch of a Select Shell
fn create_select_prop(prop: Node) -> Result<(String, (Data, usize)), PangError> {
   // Check if node is statement
   match prop {
    Node::Literal(_, loc) => Err(PangError::SyntaxError(loc)),
    Node::Int(_, loc) => Err(PangError::SyntaxError(loc)),
    Node::Float(_, loc) => Err(PangError::SyntaxError(loc)),
    Node::Token(_, loc) => Err(PangError::SyntaxError(loc)),
    Node::Shell { outside, inside } => {
        create_select_prop(*outside)
    },
    Node::Statement { variant, context, child } => {
        // Check if Set token is present
        match *variant {
            Node::Literal(_, loc) => return Err(PangError::SyntaxError(loc)),
            Node::Int(_, loc) => return Err(PangError::SyntaxError(loc)),
            Node::Float(_, loc) => return Err(PangError::SyntaxError(loc)),
            Node::Shell { outside, inside } => {
                return create_select_prop(*outside)
            },
            Node::Statement { variant, context, child } => {
                return create_select_prop(*variant)
            },

            Node::Token(token, loc) => match token {
                Token::Set => {},
                _ => return Err(PangError::SyntaxError(loc))
            },
        };

        // Name of the field
        let (name, loc) = match *context {
            Node::Int(_, loc) => Err(PangError::SyntaxError(loc)),
            Node::Float(_, loc) => Err(PangError::SyntaxError(loc)),
            Node::Token(_, loc) => Err(PangError::SyntaxError(loc)),
            Node::Shell { outside, inside } => {
                return create_select_prop(*outside)
            },
            Node::Statement { variant, context, child } => {
                return create_select_prop(*variant)
            },
            Node::Literal(string, loc) => Ok((string, loc)),
        }?;

        // Actual data check if starting value is given
        let data = match child {
            Some(child) => {
                // Verify that child is a statement
                match *child {
                    Node::Literal(_, loc) => Err(PangError::SyntaxError(loc)),
                    Node::Int(_, loc) => Err(PangError::SyntaxError(loc)),
                    Node::Float(_, loc) => Err(PangError::SyntaxError(loc)),
                    Node::Token(_, loc) => Err(PangError::SyntaxError(loc)),
                    Node::Shell { outside, inside } => {
                        return create_select_prop(*outside)
                    },
                    Node::Statement { variant, context, child } => {
                        // Validate, that variant is VALUE
                        match *variant {
                            Node::Literal(_, loc) => return Err(PangError::SyntaxError(loc)),
                            Node::Int(_, loc) => return Err(PangError::SyntaxError(loc)),
                            Node::Float(_, loc) => return Err(PangError::SyntaxError(loc)),
                            Node::Shell { outside, inside } => {
                                return create_select_prop(*outside)
                            },
                            Node::Statement { variant, context, child } => {
                                return create_select_prop(*variant)
                            },
            
                            Node::Token(token, loc) => match token {
                                // Validate VALUE token
                                Token::Value => {}
                                _ => return Err(PangError::SyntaxError(loc))
                            },
                        };
                        
                        // If child is something throw it out the window
                        if child.is_some() {
                            return create_select_prop(*child.unwrap())
                        }

                        // Validate value
                        let data: (Data, usize) = match *context {
                            Node::Token(_, loc) => return Err(PangError::SyntaxError(loc)),
                            Node::Shell { outside, inside } => {
                                return create_select_prop(*outside)
                            },
                            Node::Statement { variant, context, child } => {
                                return create_select_prop(*variant)
                            },
                            Node::Literal(string, loc) => (string.into(), loc),
                            Node::Int(int, loc) => (int.into(), loc),
                            Node::Float(float, loc) => (float.into(), loc),
                        };

                        Ok(data)
                    },
                }?;

                todo!()
            },
            None => return Err(PangError::SyntaxError(loc + name.len() + 3)),
        };

        return Ok((name, data))
    },
}
}

/// Makes the instances selection and overwrites the values
fn make_selection(name: String, properties: Vec<Node>, loc: usize) -> Result<(), PangError> {
    let mut instance = remove_instance(name, loc)?;
    for prop in properties {
        let (name, (data, loc)) = create_select_prop(prop)?;
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
    Ok(())
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