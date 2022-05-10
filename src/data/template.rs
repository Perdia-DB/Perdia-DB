use std::{collections::HashMap};
use linked_hash_map::LinkedHashMap;
use serde::{Serialize, Deserialize, de::Visitor};

use super::serialization::{Data, DataType, DataUnion};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Template {
    pub template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    pub data: LinkedHashMap<String, Data>
}

impl Template {
    pub fn new(name: String) -> TemplateBuilder {
        TemplateBuilder {
            template: Some(name),
            instance: None,
            data: None,
        }
    }

    pub fn instance(instance: String) -> TemplateBuilder {
        TemplateBuilder {
            template: None,
            instance: Some(instance),
            data: None,
        }
    }
}

pub struct TemplateBuilder {
    template: Option<String>,
    instance: Option<String>,
    data: Option<LinkedHashMap<String, Data>>
}

impl TemplateBuilder {

    pub fn build(self) -> Template {
        let data = self.data.unwrap_or_default();
    
        Template {
            template: self.template,
            instance: None,
            data,
        }
    }

    pub fn with_name(&self, name: String) -> Self {
        Self {
            template: self.template.clone(),
            instance: Some(name),
            data: self.data.clone()
        }
    }

    fn with(&self, name: String, data: DataUnion, data_type: DataType) -> Self {
        let value = Data { data_type, data };
        let mut data = self.data.clone().unwrap_or_default();
        data.insert(name, value);
        Self {
            template: self.template.clone(),
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