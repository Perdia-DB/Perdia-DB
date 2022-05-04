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

/// Query the parsed data from memory
// TODO: Rework the Query system to use a more modular approach.
pub fn data(parsed_data: Vec<Vec<TokenMatch>>) -> Result<String, PangQueryError> {
    for line in parsed_data {
        if line.len() < 2 { continue }
        let token_match = line.get(0).unwrap();
        match token_match.token {
            Token::Type => todo!(),
            Token::Name => todo!(),
            Token::Create => todo!(),
            Token::Query => {
                let token_match = line.get(1).unwrap();
                match token_match.token {
                    Token::Literal => {

                    }
                    Token::Type => {
                        if line.len() == 2 {
                            return Ok(serde_json::to_string(&queries::query_registered_types()).unwrap_err().to_string());
                        }
                        let token_match = line.get(2).unwrap();
                        match token_match.token {
                            Token::Literal => {
                                let instances = queries::query_types(&token_match.value);
                                if line.len() == 3 {
                                    return Ok(serde_json::to_string(&instances).unwrap_err().to_string());
                                }
                                let token_match = line.get(3).unwrap();
                                match token_match.token {
                                    Token::Get => {
                                        let (_, right) = line.split_at(4);
                                        let 
                                    },
                                    _ => { return Err(PangQueryError) }
                                }
                            }
                            _ => { return Err(PangQueryError) }
                        }
                    }
                    _ => { return Err(PangQueryError) }
                }
            },
            _ => { return Err(PangQueryError); }
        }
    }
    Ok("".to_owned())
}