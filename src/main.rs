mod parser;
mod template;

fn main() {

    let source = r#"
    TYPE "DAY";
    NAME "First" TYPE STRING STARTING "Nothing";
    NAME "Second" TYPE STRING STARTING "Nothing";
    NAME "Third" TYPE STRING STARTING "Nothing";
    NAME "Day" TYPE INTEGER STARTING 1;
    NAME "Seconds" TYPE FLOAT;
    END;
    "#.to_string();

    let out = parser::parse(&source);
    out.iter().for_each(|m| println!("{:?}", m));
}
