use super::data::{Token, TokenDefinition};

/// Initializes every [`TokenDefinition`] and defines the regex for each.
pub fn initialize() -> Vec<TokenDefinition> {
    let mut token_definitions = Vec::<TokenDefinition>::with_capacity(23);
    
    token_definitions.push(TokenDefinition::new(Token::ENDL, r";", 2));
    token_definitions.push(TokenDefinition::new(Token::Query, r"QUERY", 2));
    token_definitions.push(TokenDefinition::new(Token::Delete, r"DELETE", 2));
    token_definitions.push(TokenDefinition::new(Token::Select, r"SELECT", 2));
    token_definitions.push(TokenDefinition::new(Token::Value, r#"VALUE"#, 2));
    token_definitions.push(TokenDefinition::new(Token::From, r#"FROM"#, 2));
    token_definitions.push(TokenDefinition::new(Token::Create, r"CREATE", 2));
    token_definitions.push(TokenDefinition::new(Token::Template, r"TEMPLATE", 2));
    token_definitions.push(TokenDefinition::new(Token::Instance, r#"INSTANCE"#, 2));
    token_definitions.push(TokenDefinition::new(Token::Set, r"SET", 2));
    token_definitions.push(TokenDefinition::new(Token::End, r"END", 2));
    token_definitions.push(TokenDefinition::new(Token::StringType, r"STRING", 2));
    token_definitions.push(TokenDefinition::new(Token::IntegerType, r"INTEGER", 2));
    token_definitions.push(TokenDefinition::new(Token::FloatType, r"FLOAT", 2));
    token_definitions.push(TokenDefinition::new(Token::Literal, r#""(?:[^"\\]|\\.)*""#, 1));
    token_definitions.push(TokenDefinition::new(Token::Float, r#"\d+\.\d+"#, 3));
    token_definitions.push(TokenDefinition::new(Token::Integer, r#"\d+"#, 4));

    token_definitions.sort_by(|a, b| a.priority.cmp(&b.priority));
    token_definitions
}