use std::ops::{Range, Sub};

use regex::Regex;
use serde::{Serialize, Deserialize};

/// The different Keywords used in PANG
/// 
/// It has 19 Tokens in total
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Token {
    Delete,
    Select,
    Value,
    Template,
    Instance,
    End,
    Create,
    Query,
    Set,
    From,
    StringType,
    IntegerType,
    FloatType,
    ENDL,
    Literal,
    Integer,
    Float,
}

/// Defines where a given [`TokenDefinition`] has matched the source
#[derive(Debug, Clone)]
pub struct TokenMatch {
    pub token: Token,
    pub value: String,
    pub start: usize,
    pub end: usize,
    pub priority: u8,
}

/// Adds extra information to [`Token`]
#[derive(Debug)]
pub struct TokenDefinition {
    pub regex: Regex,
    pub token: Token,
    pub priority: u8,
}

impl TokenDefinition {
    /// Creates a new TokenDefinition   
    /// based on a [`Token`], a `Regex` string to match the token against and a priority.
    /// 
    /// Every TokenDefinition gets sorted based on their priority and those with higher priority 
    /// will get processed first in the parsing process.
    pub fn new(token: Token, regex: &str, priority: u8) -> Self {
        Self {
            regex: Regex::new(regex).unwrap(),
            token,
            priority,
        }
    }

    /// Match the source string against the TokenDefinition
    pub fn match_text(&self, source: &String, already_matched: &mut Vec<TokenMatch>) {
        let mut result: Vec<TokenMatch> = Vec::new();
        let captures = self.regex.captures_iter(source);
        for capture in captures {
            if capture.get(0).is_some() {
                let capture = capture.get(0).unwrap();
                let mut exists = false;

                already_matched.iter().for_each(|a_match| {
                    exists = Self::range_overlap(capture.start()..capture.end(), a_match.start..a_match.end);
                });
                
                if !exists {
                    result.push(
                        TokenMatch {
                            token: self.token.clone(),
                            value: capture.as_str().to_owned(),
                            start: capture.start(),
                            end: capture.end(),
                            priority: self.priority,
                        }
                    );
                }
            }
        }
        already_matched.append(&mut result);
    }

    /// Returns true if first [`Range`] is overlapping with the second [`Range`]
    fn range_overlap(first: Range<usize>, second: Range<usize>) -> bool {
        (first.start >= second.start && first.start <= second.end) ||
        (first.end-1 >= second.start && first.end-1 <= second.end)
    }
}