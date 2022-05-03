use std::collections::HashMap;

type Data = (Option<String>, Option<f64>, u8);

pub struct Template {
    name: String,
    data: HashMap<String, Data>
}

impl Template {
    pub fn new(name: String, data: HashMap<String, Data>) -> Self {
        Self {
            name,
            data,
        }
    }
}