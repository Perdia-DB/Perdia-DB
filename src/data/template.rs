use std::{collections::HashMap};
use serde::{Serialize, Deserialize, de::Visitor};

use super::serialization::{Data, DataType, DataUnion};

#[derive(Serialize, Deserialize)]
pub struct Template {
    pub name: Option<String>,
    pub instance: Option<String>,
    pub data: HashMap<String, Data>
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

pub struct TemplateBuilder {
    name: Option<String>,
    instance: Option<String>,
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
            let value: Data = Data { data_type: DataType::STRING, data: DataUnion { string: Box::leak(string.unwrap_or("".to_string()).into_boxed_str()) } };
            let mut data: HashMap<String, Data> = HashMap::new();
            data.insert(name, value);
            Self {
                name: self.name.clone(),
                instance: self.instance.clone(),
                data: Some(data),
            }
        } else {
            let mut data = self.data.clone().unwrap();
            let value: Data = Data { data_type: DataType::STRING, data: DataUnion { string: Box::leak(string.unwrap().into_boxed_str()) } };
            data.insert(name, value);
            Self {
                name: self.name.clone(),
                instance: self.instance.clone(),
                data: Some(data),
            }
        }
    }

    pub fn with_integer(&self, name: String, int: Option<i64>) -> Self {
        if self.data.is_none() {
            let value: Data = Data { data_type: DataType::INTEGER, data: DataUnion { integer: int.unwrap_or(0) } };
            let mut data: HashMap<String, Data> = HashMap::new();
            data.insert(name, value);
            Self {
                name: self.name.clone(),
                instance: self.instance.clone(),
                data: Some(data),
            }
        } else {
            let mut data = self.data.clone().unwrap();
            let value: Data = Data { data_type: DataType::INTEGER, data: DataUnion { integer: int.unwrap() } };
            data.insert(name, value);
            Self {
                name: self.name.clone(),
                instance: self.instance.clone(),
                data: Some(data),
            }
        }
    }

    pub fn with_float(&self, name: String, float: Option<f64>) -> Self {
        if self.data.is_none() {
            let value: Data = Data { data_type: DataType::FLOAT, data: DataUnion { float: float.unwrap_or(0.0) } };
            let mut data: HashMap<String, Data> = HashMap::new();
            data.insert(name, value);
            Self {
                name: self.name.clone(),
                instance: self.instance.clone(),
                data: Some(data),
            }
        } else {
            let mut data = self.data.clone().unwrap();
            let value: Data = Data { data_type: DataType::FLOAT, data: DataUnion { float: float.unwrap() } };
            data.insert(name, value);
            Self {
                name: self.name.clone(),
                instance: self.instance.clone(),
                data: Some(data),
            }
        }
    }
}