use crate::{ast::Node, error::PangError, lexer::data::Token, data::serialization::{Data, DataType}};

/// Creates a Variable from the Inside branch of a Template Shell
pub fn create_template_prop(prop: Node) -> Result<(String, Data), PangError> {
    // Check if node is statement
    match prop {
        Node::Statement { variant, context, child } => {
            // What DataType to expect
            let data_type: DataType = match *variant {
                Node::Token(token, loc) => match token {
                    Token::StringType => DataType::STRING,
                    Token::IntegerType => DataType::INTEGER,
                    Token::FloatType => DataType::FLOAT,
                    _ => return Err(PangError::SyntaxError(loc))
                },
                _ => return Err(PangError::ExecutionError),
            };

            // Name of the field
            let name: String = match *context {
                Node::Literal(string, _) => Ok(string),
                _ => return Err(PangError::ExecutionError),
            }?;

            // Actual data check if starting value is given
            let data: Data = match child {
                Some(child) => {
                    // Verify that child is a statement
                    match *child {
                        Node::Statement { variant, context, child } => {
                            // Validate, that variant is VALUE
                            match *variant {
                                Node::Token(token, loc) => match token {
                                    Token::Value => {}
                                    _ => return Err(PangError::SyntaxError(loc))
                                },
                                _ => return Err(PangError::ExecutionError),
                            };
                            
                            // If child is something throw it out the window
                            if child.is_some() {
                                return create_template_prop(*child.unwrap())
                            }

                            // Validate value
                            let data: (Data, usize) = match *context {
                                Node::Literal(string, loc) => (string.into(), loc),
                                Node::Int(int, loc) => (int.into(), loc),
                                Node::Float(float, loc) => (float.into(), loc),
                                _ => return Err(PangError::ExecutionError),
                            };

                            // Validate types
                            if data_type != data.0.data_type {
                                return Err(PangError::TypeMismatch(data.1))
                            }

                            Ok(data.0)
                        },
                        _ => return Err(PangError::ExecutionError),
                    }?
                },
                None => match data_type {
                    DataType::STRING => "".into(),
                    DataType::INTEGER => 0.into(),
                    DataType::FLOAT => 0.0.into(),
                },
            };

            return Ok((name, data))
        },
        _ => return Err(PangError::ExecutionError),
    }
}

/// Creates a Variable from the Inside branch of a Select Shell
pub fn create_select_prop(prop: Node) -> Result<(String, (Data, usize)), PangError> {
    // Check if node is statement
    match prop {
        Node::Statement { variant, context, child } => {
            // Check if Set token is present
            match *variant {
                Node::Token(token, loc) => match token {
                    Token::Set => {},
                    _ => return Err(PangError::SyntaxError(loc))
                },
                _ => return Err(PangError::ExecutionError),
            };
 
            // Name of the field
            let (name, loc) = match *context {
                Node::Literal(string, loc) => Ok((string, loc)),
                _ => return Err(PangError::ExecutionError),
            }?;
 
            // Actual data check if starting value is given
            let data = match child {
                Some(child) => {
                    // Verify that child is a statement
                    match *child {
                        Node::Statement { variant, context, child } => {
                            // Validate, that variant is VALUE
                            match *variant {
                                Node::Token(token, loc) => match token {
                                    // Validate VALUE token
                                    Token::Value => {}
                                    _ => return Err(PangError::SyntaxError(loc))
                                },
                                _ => return Err(PangError::ExecutionError),
                            };
                         
                            // If child is something throw it out the window
                            if child.is_some() {
                                return create_select_prop(*child.unwrap())
                            }
 
                            // Validate value
                            let data: (Data, usize) = match *context {
                                Node::Literal(string, loc) => (string.into(), loc),
                                Node::Int(int, loc) => (int.into(), loc),
                                Node::Float(float, loc) => (float.into(), loc),
                                _ => return Err(PangError::ExecutionError),
                            };
 
                            Ok(data)
                        },
                        _ => return Err(PangError::ExecutionError),
                    }?
                },
                None => return Err(PangError::SyntaxError(loc + name.len() + 3)),
            };
            return Ok((name, data))
        },
        _ => return Err(PangError::ExecutionError),
    }
}
 