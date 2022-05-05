use std::ops::Range;

use regex::Regex;

/// The diffrent Keywords used in PANG
/// 
/// It has 13 Tokens
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Type,
    Name,
    End,
    Create,
    Query,
    Put,
    Get,
    StringType,
    IntegerType,
    FloatType,
    Starting,
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
    /// Create a new TokenDefinition,   
    /// based on a [`Token`], a `Regex` string to match the token against and a priority.
    /// 
    /// Every TokenDefinition gets sorted based on this priority and those with higher priority will go first in the parsing process.
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

    /// Returns true if first [`Range`] is overlapping second [`Range`]
    fn range_overlap<T>(first: Range<T>, second: Range<T>) -> bool 
        where T: PartialEq + PartialOrd 
    {
        (first.start >= second.start && first.start <= second.end) ||
        (first.end >= second.start && first.end <= second.end)
    }
}