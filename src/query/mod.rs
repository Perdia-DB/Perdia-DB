use std::{sync::MutexGuard, time::Instant};

use crate::{data::{template::Template, TEMPLATES, INSTANCES, serialization::{Data, DataType}}, plog, ast::{self, Node}, perr, error::PangError};
use error::RequestError;
use linked_hash_map::LinkedHashMap;
use crate::lexer::data::{Token, TokenMatch};

pub mod error;

/*
/// Creates a new [`Template`] from a template declaration block.
pub fn create_template(mut lines: Vec<Vec<TokenMatch>>) -> Result<Template, RequestError> {
    let first = lines.remove(0);
    lines.remove(lines.len()-1);
    // Validate statement begin
    if first.len() != 2 { return Err(RequestError::SyntaxError); }
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
                _ => { return Err(RequestError::SyntaxError); }
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
                _ => { return Err(RequestError::SyntaxError); }
            }
        // throw an error at any other size
        } else {
            return Err(RequestError::SyntaxError)
        }
    }
    Ok(template.build())
}

/// executes multiline queries.
pub fn multiline_query(instance: Template, lines: Vec<Vec<TokenMatch>>, mutex: &mut MutexGuard<Vec<Template>>) -> Result<Vec<Template>, RequestError> {
    let mut output: Vec<Template> = Vec::new();
    let mut instance = Box::new(instance);
    for line in lines {
        let mut iter = line.iter();
        match iter.next() {
            Some(next) => match next.token {
                Token::Get => match iter.next() {
                    Some(next) => {
                        let mut instance_clone = instance.clone(); 
                        let instance_data = instance_clone.data.clone();
                        instance_clone.data = LinkedHashMap::new();
                        let field = next.value.clone();
                        let data = instance_data.get(&field).unwrap().clone();
                        let mut map: LinkedHashMap<String, Data> = LinkedHashMap::new();
                        map.insert(field, data);
                        instance_clone.data.extend(map.clone());
                        'inner: loop {
                            match iter.next() {
                                Some(next) => match next.token {
                                    Token::Literal => {
                                        let field = next.value.clone();
                                        let data = instance_data.get(&field).unwrap().clone();
                                        map.insert(field, data);
                                        instance_clone.data.extend(map.clone());
                                    }
                                    _ => return Err(RequestError::SyntaxError)
                                },
                                None => break 'inner,
                            }
                        }
                        output.push(*instance_clone);
                    },
                    None => return Err(RequestError::SyntaxError),
                }
                Token::Set => {
                    match iter.next() {
                        Some(next) => match next.token {
                            Token::Literal => {
                                let key = next.value.clone();
                                match iter.next() {
                                    Some(next) => match next.token {
                                        Token::Value => match iter.next() {
                                            Some(next) => match next.token {
                                                Token::Literal => {
                                                    let value = next.value.clone();
                                                    instance.data.insert(key, Data::from(value));
                                                },
                                                Token::Integer => {
                                                    let value = next.value.clone().parse::<i64>().unwrap();
                                                    instance.data.insert(key, Data::from(value));
                                                },
                                                Token::Float => {
                                                    let value = next.value.clone().parse::<f64>().unwrap();
                                                    instance.data.insert(key, Data::from(value));
                                                },
                                                _ => return Err(RequestError::SyntaxError)
                                            },
                                            None => return Err(RequestError::SyntaxError),
                                        }
                                        _ => return Err(RequestError::SyntaxError)
                                    },
                                    None => return Err(RequestError::SyntaxError),
                                }
                            }
                            _ => return Err(RequestError::SyntaxError)
                        },
                        None => return Err(RequestError::SyntaxError),
                    }

                }
                _ => return Err(RequestError::SyntaxError)
            },
            None => return Err(RequestError::SyntaxError),
        }
    }
    mutex.push(*instance);
    Ok(output)
}

// TODO: Should be reworked to feature an ast with dynamic execution. For now this very rigid model works `fine`.
/// Executes the statements from the query.
pub fn execute_statements(mut lines: Vec<Vec<TokenMatch>>) -> Result<Vec<Template>, RequestError> {
    plog!("{:?}", lines);
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
                                                    template.template.as_ref().unwrap().clone() == template_name
                                                ).collect::<Vec<&Template>>().get(0).unwrap().clone().clone();
                                            instance.instance = Some(name);
                                            let mut mutex = INSTANCES.lock().unwrap();
                                            mutex.push(instance);
                                            lines.remove(0);
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
                Token::Query => {
                    match iter.next() {
                        Some(next) => {
                            match next.token {
                                Token::Type => {
                                    let mutex = TEMPLATES.lock().unwrap();
                                    output.extend(mutex.clone())
                                },
                                Token::Literal => {
                                    let mut mutex = INSTANCES.lock().unwrap();
                                    let index = *mutex.iter().enumerate()
                                        .filter(|(_, template)| template.instance == Some(next.value.clone()))
                                        .map(|(index, _)| index).collect::<Vec<usize>>().get(0).unwrap(); // TODO: Throw error
                                    let mut instance = mutex.remove(index);
                                    match iter.next() {
                                        Some(next) => match next.token {
                                            
                                            Token::Get => {
                                                let data = instance.data.clone();
                                                instance.data = LinkedHashMap::new();
                                                loop {
                                                    match iter.next() {
                                                        Some(next) => match next.token {
                                                            Token::Literal => {
                                                                let field = next.value.clone();
                                                                let data = data.get(&field).unwrap().clone();
                                                                let mut map: LinkedHashMap<String, Data> = LinkedHashMap::new();
                                                                map.insert(field, data);
                                                                instance.data.extend(map);
                                                                //lines.remove(0);
                                                            }
                                                            _ => return Err(RequestError::SyntaxError)
                                                        },
                                                        None => break,
                                                    }
                                                }
                                                output.push(instance.clone());
                                                mutex.push(instance);
                                            }
                                            Token::Set => {
                                                match iter.next() {
                                                    Some(next) => match next.token {
                                                        Token::Literal => {
                                                            let key = next.value.clone();
                                                            match iter.next() {
                                                                Some(next) => match next.token {
                                                                    Token::Value => match iter.next() {
                                                                        Some(next) => match next.token {
                                                                            Token::Literal => {
                                                                                let value = next.value.clone();
                                                                                instance.data.insert(key, Data::from(value));
                                                                                mutex.push(instance)
                                                                            },
                                                                            Token::Integer => {
                                                                                let value = next.value.clone().parse::<i64>().unwrap();
                                                                                instance.data.insert(key, Data::from(value));
                                                                                mutex.push(instance)
                                                                            },
                                                                            Token::Float => {
                                                                                let value = next.value.clone().parse::<f64>().unwrap();
                                                                                instance.data.insert(key, Data::from(value));
                                                                                mutex.push(instance)
                                                                            },
                                                                            _ => return Err(RequestError::SyntaxError)
                                                                        },
                                                                        None => return Err(RequestError::SyntaxError),
                                                                    }
                                                                    _ => return Err(RequestError::SyntaxError)
                                                                },
                                                                None => return Err(RequestError::SyntaxError),
                                                            }
                                                        }
                                                        _ => return Err(RequestError::SyntaxError)
                                                    },
                                                    None => return Err(RequestError::SyntaxError),
                                                }
                                            }
                                            Token::Then => {
                                                let start_index = index+1;
                                                let end_index = *lines.iter().enumerate()
                                                    .filter(|(_, line)| line.get(0).unwrap().token == Token::End)
                                                    .map(|(index, _)| index).collect::<Vec<usize>>().get(0).unwrap()-1;
                                                let set_lines: Vec<Vec<TokenMatch>> = lines.drain(start_index..=end_index).collect();
                                                lines.drain(0..end_index);
                                                let result = multiline_query(instance.clone(), set_lines, &mut mutex)?;
                                                output.extend(result.clone());
                                            }
                                            _ => return Err(RequestError::SyntaxError)
                                        },
                                        None => {
                                            output.push(instance.clone());
                                            mutex.push(instance);
                                        },
                                    }
                                },
                                _ => return Err(RequestError::SyntaxError)
                            }
                        },
                        None => return Err(RequestError::SyntaxError),
                    }
                },
                // Pull out the whole template
                Token::Type => {
                    //let start_index = index;
                    let start_index = 0;
                    let end_index = *lines.iter().enumerate()
                        .filter(|(_, line)| line.get(0).unwrap().token == Token::End)
                        .map(|(index, _)| index).collect::<Vec<usize>>().get(0).unwrap();
                    let template: Vec<Vec<TokenMatch>> = lines.drain(start_index..=end_index).collect();
                    let template = create_template(template)?;
                    let mut mutex = TEMPLATES.lock().unwrap();
                    if mutex.contains(&template) {
                        return Err(RequestError::InstanceAlreadyExists);
                    }
                    mutex.push(template.clone());
                },
                // Deletion
                Token::Delete => match iter.next() {
                    Some(next) => match next.token {
                        Token::Literal => {
                            let mut mutex = INSTANCES.lock().unwrap();
                            let index = *mutex.iter().enumerate()
                                .filter(|(_, template)| template.instance == Some(next.value.clone()))
                                .map(|(index, _)| index).collect::<Vec<usize>>().get(0).unwrap(); // TODO: Throw error
                            mutex.remove(index);
                        },
                        Token::Type => match iter.next() {
                            Some(next) => match next.token {
                                Token::Literal => {
                                    let mut mutex = TEMPLATES.lock().unwrap();
                                    let index = *mutex.iter().enumerate()
                                        .filter(|(_, template)| template.template == Some(next.value.clone()))
                                        .map(|(index, _)| index).collect::<Vec<usize>>().get(0).unwrap(); // TODO: Throw error
                                    mutex.remove(index);
                                    // Remove Instances
                                    let mut mutex = INSTANCES.lock().unwrap();
                                    for (index, instance) in mutex.clone().iter().enumerate() {
                                        if instance.template.as_ref().unwrap_or(&"".to_owned()) == &next.value {
                                            mutex.remove(index);
                                        }
                                    }
                                },
                                _ => return Err(RequestError::SyntaxError),
                            },
                            None => return Err(RequestError::SyntaxError),
                        }
                        _ => return Err(RequestError::SyntaxError), 
                    },
                    None => return Err(RequestError::SyntaxError),
                },
                // Ignore declaration Tokens
                Token::Name => {},
                Token::End => {},
                // Ignore multiline query set
                Token::Set => {}
                Token::Get => {}
                _ => return Err(RequestError::SyntaxError)
            },
            None => return Err(RequestError::SyntaxError),
        }
    }

    Ok(output)
}
*/

fn exec(ast: Vec<Node>) -> Result<String, PangError> {

    for branch in ast {
        exec_branch(branch)
    }

    Ok("".to_string())
}

fn exec_branch(branch: Node) {
    match branch {
        Node::Literal(_, _) => todo!(),
        Node::Int(_, _) => todo!(),
        Node::Float(_, _) => todo!(),
        Node::Token(_, _) => todo!(),
        Node::Statement { variant, context, child } => todo!(),
        Node::Shell { outside, inside } => todo!(),
    }
}

/// Creates a Variable from the Inside branch of a Template Shell
fn create_prop(prop: Node) -> Result<(String, Data), PangError> {
    // Check if node is statement
    match prop {
        Node::Literal(_, loc) => Err(PangError::SyntaxError(loc)),
        Node::Int(_, loc) => Err(PangError::SyntaxError(loc)),
        Node::Float(_, loc) => Err(PangError::SyntaxError(loc)),
        Node::Token(_, loc) => Err(PangError::SyntaxError(loc)),
        Node::Shell { outside, inside } => {
            create_prop(*outside)
        },
        Node::Statement { variant, context, child } => {
            // What DataType to expect
            let data_type: DataType = match *variant {
                Node::Literal(_, loc) => return Err(PangError::SyntaxError(loc)),
                Node::Int(_, loc) => return Err(PangError::SyntaxError(loc)),
                Node::Float(_, loc) => return Err(PangError::SyntaxError(loc)),
                Node::Shell { outside, inside } => {
                    return create_prop(*outside)
                },
                Node::Statement { variant, context, child } => {
                    return create_prop(*variant)
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
                    return create_prop(*outside)
                },
                Node::Statement { variant, context, child } => {
                    return create_prop(*variant)
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
                            return create_prop(*outside)
                        },
                        Node::Statement { variant, context, child } => {
                            // Validate, that variant is VALUE
                            match *variant {
                                Node::Literal(_, loc) => return Err(PangError::SyntaxError(loc)),
                                Node::Int(_, loc) => return Err(PangError::SyntaxError(loc)),
                                Node::Float(_, loc) => return Err(PangError::SyntaxError(loc)),
                                Node::Shell { outside, inside } => {
                                    return create_prop(*outside)
                                },
                                Node::Statement { variant, context, child } => {
                                    return create_prop(*variant)
                                },
                
                                Node::Token(token, loc) => match token {
                                    Token::Value => {}
                                    _ => return Err(PangError::SyntaxError(loc))
                                },
                            };
                            
                            // If child is something throw it out the window
                            if child.is_some() {
                                return create_prop(*child.unwrap())
                            }

                            // Validate value
                            let data: (Data, usize) = match *context {
                                Node::Token(_, loc) => return Err(PangError::SyntaxError(loc)),
                                Node::Shell { outside, inside } => {
                                    return create_prop(*outside)
                                },
                                Node::Statement { variant, context, child } => {
                                    return create_prop(*variant)
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
fn create_template(name: String, properties: Vec<Node>) -> Result<(), PangError> {
    let template = Template::new(name);
    for prop in properties {
        let (name, data) = create_prop(prop)?;
        template.with_data(name, data);
    }
    Ok(())
}

/// Query the parsed data from memory
pub fn data(lines: Vec<Vec<TokenMatch>>) -> String {
    /*match serde_json::to_string_pretty(&execute_statements(lines)?) {
        Ok(value) => Ok(value),
        Err(_) => Err(RequestError::SerializationError),
    }*/
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