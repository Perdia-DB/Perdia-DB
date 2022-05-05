use data::{TEMPLATES, template::Template};
use serde_json::*;

use crate::data::INSTANCES;

mod lexer;
mod data;
mod query;

fn main() {

    let source = r#"
    TYPE "DAY";
    NAME "First" TYPE STRING STARTING "Nothing";
    NAME "Second" TYPE STRING STARTING "Nothing";
    NAME "Third" TYPE STRING STARTING "Nothing";
    NAME "Day" TYPE INTEGER STARTING 1;
    NAME "Seconds" TYPE FLOAT;
    END;
    CREATE "Monday" TYPE "DAY";
    "#.to_string();

    let start = std::time::Instant::now();
    let parsed_data = lexer::parse(&source);
    println!("Took: {:?}", start.elapsed());
    query::data(parsed_data).unwrap();
    let temp = TEMPLATES.lock().unwrap();
    let inst = INSTANCES.lock().unwrap();
    std::fs::write("./template.json", serde_json::to_string_pretty(&*temp).unwrap()).unwrap();
    std::fs::write("./instance.json", serde_json::to_string_pretty(&*inst).unwrap()).unwrap();
}
