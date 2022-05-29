use std::{iter::Map, collections::HashMap};

use regex::{self, Regex};

use crate::{lexer::data::Token, plog};

#[derive(Clone, Debug)]
pub enum RuleSnippet {
    Expandable(Vec<RuleSnippet>),
    Inner(Vec<RuleSnippet>),
    Statement(Vec<RuleSnippet>),
    Defined(Token),
    Tuple(Vec<Token>),
    //Optional(Vec<RuleSnippet>)
}

pub fn grammar_rule(rule: &str, ex: bool, ie: bool) -> RuleSnippet {
    //let optional_regex = Regex::new(r#"\[(\[*(?:[^\]\[]*|\[[^\]]*\])*\]*)\]"#).unwrap();
    let tuple_regex = Regex::new(r#"<(<*(?:[^><]*|<[^>]*>)*>*)>"#).unwrap();
    let keyword_regex = Regex::new("([a-zA-Z$]+)").unwrap();
    if ex {
        RuleSnippet::Expandable(parse(rule.to_string(), /*&optional_regex,*/ &tuple_regex, &keyword_regex))
    } else if ie {
        RuleSnippet::Inner(parse(rule.to_string(), /*&optional_regex,*/ &tuple_regex, &keyword_regex))
    } else {
        RuleSnippet::Statement(parse(rule.to_string(), /*&optional_regex,*/ &tuple_regex, &keyword_regex))
    }
}

fn parse(mut rule: String, /*optional_regex: &Regex,*/ tuple_regex: &Regex, keyword_regex: &Regex) -> Vec<RuleSnippet> {
    let mut map: HashMap<usize, RuleSnippet> = HashMap::new();
    // take care of optionals
    /*for cap in optional_regex.captures_iter(&rule) {
        let index = cap.get(1).unwrap().start();
        let optional = parse((&cap[1]).to_string(), &optional_regex, &tuple_regex, &keyword_regex);
        map.insert(index, RuleSnippet::Optional(optional));
    }
    rule = (&*optional_regex.replace_all(&rule, "")).to_string();*/

    // tuples
    for cap in tuple_regex.captures_iter(&rule) {
        let index = cap.get(0).unwrap().start();
        let content = &cap[1].split("|").collect::<Vec<&str>>();
        let types = content.iter()
            .map(|s| to_token(s))
            .collect::<Vec<Token>>();
        map.insert(index, RuleSnippet::Tuple(types));
    }
    rule = (&*tuple_regex.replace_all(&rule, "")).to_string();
    
    // keywords
    for cap in keyword_regex.captures_iter(&rule) {
        let index = cap.get(0).unwrap().start();
        let content = &cap[1];
        let token = to_token(content);
        map.insert(index, RuleSnippet::Defined(token));
    };
    let mut keys = map.keys().map(|k| *k).collect::<Vec<usize>>();
    keys.sort();
    let mut out: Vec<RuleSnippet> = Vec::with_capacity(keys.len());
    for key in keys {
        out.push(map.get(&key).unwrap().clone())
    }
    out
}

fn to_token(s: &str) -> Token {
    match s {
        "DELETE" => Token::Delete,
        "SELECT" => Token::Select,
        "VALUE" => Token::Value,
        "TEMPLATE" => Token::Template,
        "INSTANCE" => Token::Instance,
        "END" => Token::End,
        "CREATE" => Token::Create,
        "QUERY" => Token::Query,
        "SET" => Token::Set,
        "FROM" => Token::From,
        "STRING" => Token::StringType,
        "INTEGER" => Token::IntegerType,
        "FLOAT" => Token::FloatType,
        ";" => Token::ENDL,
        "$s" => Token::Literal,
        "$i" => Token::Integer,
        "$f" => Token::Float,
        _ => Token::ENDL,
    }
}