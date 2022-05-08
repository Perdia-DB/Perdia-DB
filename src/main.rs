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
    QUERY TYPE;
    
    CREATE "Monday" TYPE "DAY";
    CREATE "Tuesday" TYPE "DAY";
    CREATE "Wednesday" TYPE "DAY";

    QUERY "Tuesday" SET "Day" VALUE 2;
    QUERY "Wednesday" SET "Day" VALUE 3;

    QUERY "Monday";
    "#.to_string();

    let parsed_data = lexer::parse(&source);
    let start = std::time::Instant::now();
    let data = query::data(parsed_data).unwrap();
    println!("Took: {:?}", start.elapsed());
    println!("{}", data);
    let temp = TEMPLATES.lock().unwrap();
    let inst = INSTANCES.lock().unwrap();
    std::fs::write("./template.json", serde_json::to_string_pretty(&*temp).unwrap()).unwrap();
    std::fs::write("./instance.json", serde_json::to_string_pretty(&*inst).unwrap()).unwrap();
}
