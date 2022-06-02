use crate::{lexer::data::{Token}, error::PangError, gr};

use super::{Node, snippet::{grammar_rule, RuleSnippet}};

#[derive(Clone)]
pub struct Rule {
    snippets: Vec<RuleSnippet>
}

impl Rule {
    pub fn new() -> Self {
        let snippets = Self::init_rules();
        Self {
            snippets
        }
    }

    fn init_rules() -> Vec<RuleSnippet> {
        let mut rules: Vec<RuleSnippet> = Vec::new();
        rules.push(gr!("QUERY <$s|TEMPLATE> FROM <TEMPLATE|INSTANCE>"));
        rules.push(gr!("CREATE $s <TEMPLATE|INSTANCE> &s"));
        rules.push(gr!("TEMPLATE $s", true));
        rules.push(gr!("<STRING|INTEGER|FLOAT> $s VALUE <$s|$i|$f>", false, true));
        rules.push(gr!("<STRING|INTEGER|FLOAT> $s", false, true));
        rules.push(gr!("SELECT $s", true));
        rules.push(gr!("SET $s VALUE <$s|$i|$f>", false, true));
        rules.push(gr!("END $s", false, true));
        rules.push(gr!("DELETE $s FROM <TEMPLATE|INSTANCE>"));
        rules
    }

    pub fn check(&self, ast: &Vec<Node>) -> Result<(), PangError> {
        for branch in ast {
            self.check_branch(branch, &self.snippets)?;
        }
        Ok(())
    }

    fn check_branch(&self, branch: &Node, pos: &Vec<RuleSnippet>) -> Result<(), PangError> {
        match branch {
            Node::Literal(_, loc) => return Err(PangError::SyntaxError(*loc)),
            Node::Int(_, loc) => return Err(PangError::SyntaxError(*loc)),
            Node::Float(_, loc) => return Err(PangError::SyntaxError(*loc)),
            Node::Token(_, loc) => return Err(PangError::SyntaxError(*loc)),
            Node::Statement { 
                variant, 
                context, 
                child } => {
                    // drain valid rules based on variant
                    let last_pos = Self::get_node_position(variant);
                    let variant = Self::node_to_token(variant)?;
                    let pos: Vec<RuleSnippet> = pos.clone()
                        .drain_filter(|p| {
                            let p = match p {
                                RuleSnippet::Statement(s) => s,
                                RuleSnippet::Expandable(s) => s,
                                RuleSnippet::Inner(s) => s,
                                _ => return false
                            };
                            return match &p[0] {
                                RuleSnippet::Defined(s) => variant == s.clone(),
                                RuleSnippet::Tuple(s) => s.contains(&variant),
                                _ => false
                            }
                        }).collect();
                    if pos.len() == 0 {
                        return Err(PangError::SyntaxError(last_pos))
                    }
                    // drain valid rules based on context
                    let last_pos = Self::get_node_position(context);
                    let context = Self::node_to_token(context)?;
                    let mut pos: Vec<RuleSnippet> = pos.clone()
                        .drain_filter(|p| {
                            let p = match p {
                                RuleSnippet::Statement(s) => s,
                                RuleSnippet::Expandable(s) => s,
                                RuleSnippet::Inner(s) => s,
                                _ => return false
                            };
                            return match p.get(1).unwrap() {
                                RuleSnippet::Defined(s) => context == s.clone(),
                                RuleSnippet::Tuple(s) => s.contains(&context),
                                _ => false
                            }
                        }).collect();
                    if pos.len() == 0 {
                        return Err(PangError::SyntaxError(last_pos))
                    }
                    let mut to_remove: Vec<usize> = Vec::new();
                    for (index, p) in pos.iter_mut().enumerate() {
                        let remove =  match p {
                            RuleSnippet::Statement(s) => {
                                s.remove(0);
                                s.remove(0);
                                s.len() == 0
                            },
                            RuleSnippet::Expandable(s) => {
                                s.remove(0);
                                s.remove(0);
                                s.len() == 0
                            },
                            RuleSnippet::Inner(s) => {
                                s.remove(0);
                                s.remove(0);
                                s.len() == 0
                            },
                            _ => true
                        };
                        if remove {
                            to_remove.push(index);
                        }
                    }
                    for index in to_remove {
                        pos.remove(index);
                    }
                    // Check child
                    match child {
                        Some(child) => {
                            self.check_branch(child, &pos)?
                        },
                        None => {
                            if pos.len() != 0 {
                                return Err(PangError::SyntaxError(last_pos))
                            }
                        },
                    };
                },
            Node::Shell { 
                outside, 
                inside } => {
                    // Drain everything that cannot be expandet
                    let pos: Vec<RuleSnippet> = pos.clone()
                        .drain_filter(|p| match p {
                            RuleSnippet::Expandable(_) => true,
                            RuleSnippet::Inner(_) => true,
                            _ => false
                        }).collect();
                    let pos_inner: Vec<RuleSnippet> = pos.clone()
                        .drain_filter(|p| match p {
                            RuleSnippet::Inner(_) => true,
                            _ => false
                        }).collect();
                    let pos_outside: Vec<RuleSnippet> = pos.clone()
                        .drain_filter(|p| match p {
                            RuleSnippet::Expandable(_) => true,
                            _ => false
                        }).collect();
                    self.check_branch(outside, &pos_outside)?;
                    for statement in inside {
                        self.check_branch(statement, &pos_inner)?;
                    }
                },
        };
        Ok(())
    }

    fn node_to_token(node: &Box<Node>) -> Result<Token, PangError> {
        match &**node {
            Node::Literal(_, _) => Ok(Token::Literal),
            Node::Int(_, _) => Ok(Token::Integer),
            Node::Float(_, _) => Ok(Token::Float),
            Node::Token(t, _) => Ok(*t),
            Node::Statement { variant, context: _, child: _ } => Self::node_to_token(&variant),
            Node::Shell { outside, inside: _ } => Self::node_to_token(&outside),
        }
    }

    fn get_node_position(node: &Box<Node>) -> usize {
        match &**node {
            Node::Literal(_, loc) => *loc,
            Node::Int(_, loc) => *loc,
            Node::Float(_, loc) => *loc,
            Node::Token(_, loc) => *loc,
            Node::Statement { variant, context: _, child: _ } => Self::get_node_position(variant),
            Node::Shell { outside, inside: _ } => Self::get_node_position(outside),
        }
    }
}