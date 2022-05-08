use super::data::{Token, TokenDefinition};

/// Initializes every [`TokenDefinition`]
pub fn initialize() -> Vec<TokenDefinition> {
    let mut token_definitions = Vec::<TokenDefinition>::with_capacity(13);

    token_definitions.push(TokenDefinition::new(Token::Query, r"QUERY", 1));
    token_definitions.push(TokenDefinition::new(Token::Delete, r"DELETE", 1));
    token_definitions.push(TokenDefinition::new(Token::Remove, r"REMOVE", 1));
    token_definitions.push(TokenDefinition::new(Token::Then, r"THEN", 1));
    token_definitions.push(TokenDefinition::new(Token::Value, r#"VALUE"#, 1));
    token_definitions.push(TokenDefinition::new(Token::Create, r"CREATE", 1));
    token_definitions.push(TokenDefinition::new(Token::End, r"END", 1));
    token_definitions.push(TokenDefinition::new(Token::Type, r"TYPE", 1));
    token_definitions.push(TokenDefinition::new(Token::ENDL, r";", 1));
    token_definitions.push(TokenDefinition::new(Token::Name, r"NAME", 1));
    token_definitions.push(TokenDefinition::new(Token::Set, r"SET", 1));
    token_definitions.push(TokenDefinition::new(Token::Get, r"GET", 1));
    token_definitions.push(TokenDefinition::new(Token::StringType, r"STRING", 1));
    token_definitions.push(TokenDefinition::new(Token::IntegerType, r"INTEGER", 1));
    token_definitions.push(TokenDefinition::new(Token::FloatType, r"FLOAT", 1));
    token_definitions.push(TokenDefinition::new(Token::Starting, r"STARTING", 1));
    token_definitions.push(TokenDefinition::new(Token::Integer, r#"\d+"#, 2));
    token_definitions.push(TokenDefinition::new(Token::Float, r#"\d+\.\d+"#, 3));
    token_definitions.push(TokenDefinition::new(Token::Literal, r#""[^"]+""#, 4));

    token_definitions.sort_by(|a, b| a.priority.cmp(&b.priority));
    token_definitions
}