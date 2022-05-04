use std::{collections::HashMap};
use serde::{Serialize, Deserialize};

pub type Data = (Option<String>, Option<f64>, Option<i64>);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DataType {
    STRING,
    INTEGER,
    FLOAT,
}

#[derive(Serialize, Deserialize)]
pub struct Template {
    pub name: Option<String>,
    pub instance: Option<String>,
    pub data: HashMap<String, (DataType, Data)>
}

impl Template {
    pub fn new(name: String) -> TemplateBuilder {
        TemplateBuilder {
            name: Some(name),
            instance: None,
            data: None,
        }
    }

    pub fn instance(instance: String) -> TemplateBuilder {
        TemplateBuilder {
            name: None,
            instance: Some(instance),
            data: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TemplateBuilder {
    name: Option<String>,
    instance: Option<String>,
    data: Option<HashMap<String, (DataType, Data)>>
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
            instance: None,
            data,
        }
    }

    pub fn with_name(&self, name: String) -> Self {
        Self {
            name: self.name.clone(),
            instance: Some(name),
            data: self.data.clone()
        }
    }

    pub fn with_string(&self, name: String, string: Option<String>) -> Self {
        if self.data.is_none() {
            let value: Data = (string, None, None);
            let mut data: HashMap<String, (DataType, Data)> = HashMap::new();
            data.insert(name, (DataType::STRING, value));
            Self {
                name: self.name.clone(),
                instance: self.instance.clone(),
                data: Some(data),
            }
        } else {
            let mut data = self.data.clone().unwrap();
            let value: Data = (string, None, None);
            data.insert(name, (DataType::STRING, value));
            Self {
                name: self.name.clone(),
                instance: self.instance.clone(),
                data: Some(data),
            }
        }
    }

    pub fn with_integer(&self, name: String, int: Option<i64>) -> Self {
        if self.data.is_none() {
            let value: Data = (None, None, int);
            let mut data: HashMap<String, (DataType, Data)> = HashMap::new();
            data.insert(name, (DataType::INTEGER, value));
            Self {
                name: self.name.clone(),
                instance: self.instance.clone(),
                data: Some(data),
            }
        } else {
            let mut data = self.data.clone().unwrap();
            let value: Data = (None, None, int);
            data.insert(name, (DataType::STRING, value));
            Self {
                name: self.name.clone(),
                instance: self.instance.clone(),
                data: Some(data),
            }
        }
    }

    pub fn with_float(&self, name: String, float: Option<f64>) -> Self {
        if self.data.is_none() {
            let value: Data = (None, float, None);
            let mut data: HashMap<String, (DataType, Data)> = HashMap::new();
            data.insert(name, (DataType::INTEGER, value));
            Self {
                name: self.name.clone(),
                instance: self.instance.clone(),
                data: Some(data),
            }
        } else {
            let mut data = self.data.clone().unwrap();
            let value: Data = (None, float, None);
            data.insert(name, (DataType::STRING, value));
            Self {
                name: self.name.clone(),
                instance: self.instance.clone(),
                data: Some(data),
            }
        }
    }
}