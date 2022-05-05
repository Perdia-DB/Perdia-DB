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
        let data = self.data.unwrap_or_default();
    
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

    fn with(&self, name: String, data: DataUnion, data_type: DataType) -> Self {
        let value = Data { data_type, data };
        let mut data: HashMap<String, Data> = HashMap::new();
        data.insert(name, value);
        Self {
            name: self.name.clone(),
            instance: self.instance.clone(),
            data: Some(data),
        }
    }

    pub fn with_string(&self, name: String, string: Option<String>) -> Self {
        self.with(name, string.into(), DataType::STRING)
    }

    pub fn with_integer(&self, name: String, int: Option<i64>) -> Self {
        self.with(name, int.into(), DataType::INTEGER)
    }

    pub fn with_float(&self, name: String, float: Option<f64>) -> Self {
        self.with(name, float.into(), DataType::FLOAT)
    }
}