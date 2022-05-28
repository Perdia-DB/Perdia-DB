use crate::{lexer::data::{Token, TokenMatch}, error::PangError};

use super::Node;

pub struct Rule {
    start: Token,
    context: Vec<Token>,
    ext: Option<Vec<Option<Rule>>>,
    expanded: bool,
}

impl Rule {
    pub fn check(&self, line: Vec<TokenMatch>) -> Result<(Node, bool), PangError> {
        todo!()
    }
}

pub fn init_rules() -> Vec<Rule> {
    let mut rules: Vec<Rule> = Vec::new();

    // Rule for QUERY
    rules.push(Rule {
        start: Token::Query,
        context: vec![Token::Template, Token::Literal],
        ext: Some(vec![Some(Rule { 
                start: Token::From, 
                context: vec![Token::Template, Token::Instance], 
                ext: None, expanded: false, })]),
        expanded: false,
    });
    // Rule for DELETE
    rules.push(Rule {
        start: Token::Delete,
        context: vec![Token::Literal],
        ext: Some(vec![Some(Rule { 
                start: Token::From, 
                context: vec![Token::Template, Token::Instance], 
                ext: None, expanded: false, })]),
        expanded: false,
    });
    // Rule for CREATE
    rules.push(Rule {
        start: Token::Create,
        context: vec![Token::Literal],
        ext: Some(vec![
            Some(Rule { 
                start: Token::Instance, 
                context: vec![Token::Literal], 
                ext: None, expanded: false, }), 
            Some(Rule { 
                start: Token::Template, 
                context: vec![Token::Literal], 
                ext: None, expanded: false, })]),
        expanded: false,
    });
    // Rule for TEMPLATE
    rules.push(Rule {
        start: Token::Template,
        context: vec![Token::Literal],
        ext: None,
        expanded: true,
    });
    // Rule for SELECT
    rules.push(Rule {
        start: Token::Select,
        context: vec![Token::Literal],
        ext: None,
        expanded: true,
    });
    // Rule for END
    rules.push(Rule {
        start: Token::End,
        context: vec![Token::Literal],
        ext: None,
        expanded: false,
    });
    // Rule for SET
    rules.push(Rule { 
        start: Token::Set,
        context: vec![Token::Literal], 
        ext: Some(vec![Some(Rule { 
            start: Token::Value,
            context: vec![Token::Literal, Token::Integer, Token::Float], 
            ext: None, 
            expanded: true })]), 
        expanded: true 
    });
    // Rule for STRING
    rules.push(Rule { 
        start: Token::StringType,
        context: vec![Token::Literal], 
        ext: Some(vec![Some(Rule { 
            start: Token::Value,
            context: vec![Token::Literal], 
            ext: None, 
            expanded: true }), None]), 
        expanded: true 
    });
    // Rule for INTEGER
    rules.push(Rule { 
        start: Token::IntegerType,
        context: vec![Token::Literal], 
        ext: Some(vec![Some(Rule { 
            start: Token::Value,
            context: vec![Token::Integer], 
            ext: None, 
            expanded: true }), None]), 
        expanded: true 
    });
    // Rule for FLOAT
    rules.push(Rule { 
        start: Token::FloatType,
        context: vec![Token::Literal], 
        ext: Some(vec![Some(Rule { 
            start: Token::Value,
            context: vec![Token::Float], 
            ext: None, 
            expanded: true }), None]), 
        expanded: true 
    });
    rules
}