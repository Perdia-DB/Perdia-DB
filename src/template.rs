use std::{collections::HashMap, hash::Hash};

pub type Data = (Option<String>, Option<f64>, Option<i64>, u8);

pub struct Template {
    pub name: String,
    pub data: HashMap<String, Data>
}

impl Template {
    pub fn new(name: String) -> TemplateBuilder {
        TemplateBuilder {
            name,
            data: None,
        }
    }
}

pub struct TemplateBuilder {
    name: String,
    data: Option<HashMap<String, Data>>
}

impl TemplateBuilder {

    pub fn build(self) -> Template {
        let data;
        if self.data.is_none() {
            data = HashMap::new();
        }
        else {
            data = self.data.unwrap();
        }
        Template {
            name: self.name,
            data,
        }
    }

    pub fn with_string(&self, name: String, string: String) -> Self {
        if self.data.is_none() {
            let value: Data = (Some(string), None, None, 0);
            let mut data: HashMap<String, Data> = HashMap::new();
            data.insert(name, value);
            Self {
                name: self.name.clone(),
                data: Some(data),
            }
        } else {
            let mut data = self.data.clone().unwrap();
            let value: Data = (Some(string), None, None, 0);
            data.insert(name, value);
            Self {
                name: self.name.clone(),
                data: Some(data),
            }
        }
    }

    pub fn with_integer(&self, name: String, int: i64) -> Self {
        if self.data.is_none() {
            let value: Data = (None, None, Some(int), 2);
            let mut data: HashMap<String, Data> = HashMap::new();
            data.insert(name, value);
            Self {
                name: self.name.clone(),
                data: Some(data),
            }
        } else {
            let mut data = self.data.clone().unwrap();
            let value: Data = (None, None, Some(int), 2);
            data.insert(name, value);
            Self {
                name: self.name.clone(),
                data: Some(data),
            }
        }
    }

    pub fn with_float(&self, name: String, float: f64) -> Self {
        if self.data.is_none() {
            let value: Data = (None, Some(float), None, 1);
            let mut data: HashMap<String, Data> = HashMap::new();
            data.insert(name, value);
            Self {
                name: self.name.clone(),
                data: Some(data),
            }
        } else {
            let mut data = self.data.clone().unwrap();
            let value: Data = (None, Some(float), None, 1);
            data.insert(name, value);
            Self {
                name: self.name.clone(),
                data: Some(data),
            }
        }
    }
}