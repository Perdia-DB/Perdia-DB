use crate::{lexer::data::{TokenMatch, Token}, error::PangError, ast::rule::Rule, plog};
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};

mod rule;
mod snippet;
mod util;

lazy_static! {
    pub static ref RULE: Rule = Rule::new();
}

/// Nodes of the AST
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Node {
    Literal(String, usize),
    Int(i64, usize),
    Float(f64, usize),
    Token(Token, usize),
    Statement {
        variant: Box<Node>,
        context: Box<Node>,
        child: Option<Box<Node>>
    },
    Shell {
        outside: Box<Node>,
        inside: Vec<Box<Node>>
    }
}

/// TokenMatch to Node
impl From<TokenMatch> for Node {
    fn from(tm: TokenMatch) -> Self {
        match tm.token {
            Token::Literal => Node::Literal(tm.value, tm.start),
            Token::Integer => Node::Int(tm.value.parse::<i64>().unwrap(), tm.start),
            Token::Float => Node::Float(tm.value.parse::<f64>().unwrap(), tm.start),
            _ => Node::Token(tm.token, tm.start),
        }
    }
}

/// Builds an AST from source
pub fn parse(lines: Vec<Vec<TokenMatch>>) -> Result<Vec<Node>, PangError> {
    let mut ast: Vec<Node> = Vec::new();
    let mut shell: Option<Box<Node>> = None;
    for line in lines {
        let node = parse_node(line)?;
        let shell_or_end = is_shell_or_end(&node);
        if shell.clone().is_some() {
            if shell_or_end {
                ast.push(*shell.unwrap());
                shell = None;
            } else {
                shell = match *shell.clone().unwrap() {
                    Node::Shell { outside, mut inside } => {
                        inside.push(Box::new(node));
                        Some(Box::new(Node::Shell {
                            outside,
                            inside,
                        }))
                    },
                    _ => return Err(PangError::SyntaxError(0))
                };
            }
        } else if shell_or_end {
            shell = Some(Box::new(Node::Shell { outside: Box::new(node), inside: Vec::new() }));
        } else {
            ast.push(node);
        }
    }
    //plog!("\n{}", serde_json::to_string_pretty(&ast).unwrap());
    RULE.check(&ast)?;
    Ok(ast)
}

/// Builds Node structure from single line
pub fn parse_node(tms: Vec<TokenMatch>) -> Result<Node, PangError> {
    let node = match tms.len() {
        2 => {
            Node::Statement { 
                variant: Box::new(tms.get(0).unwrap().clone().into()), 
                context: Box::new(tms.get(1).unwrap().clone().into()), 
                child: None }
        },
        4 => {
            Node::Statement { 
                variant: Box::new(tms.get(0).unwrap().clone().into()), 
                context: Box::new(tms.get(1).unwrap().clone().into()),
                child: Some(Box::new(parse_node(tms.split_at(2).1.to_vec())?)) }
        },
        _ => return Err(PangError::SyntaxError(tms.get(0).unwrap().start))
    };
    Ok(node)
}

/// Check if statement has lines that follow that are related to it.
pub fn is_shell_or_end(node: &Node) -> bool {
    match node {
        Node::Statement { variant, context, child } => match **variant {
                Node::Token(token, loc) => match token {
                    Token::Select => true,
                    Token::Template => true,
                    Token::End => true,
                    _ => false,
                },
                _ => false
        }
        _ => false
    }
}