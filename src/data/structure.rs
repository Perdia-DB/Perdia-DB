use linked_hash_map::LinkedHashMap;
use serde::{Serialize, Deserialize, de::Visitor};

use crate::error::PangError;

use super::serialization::{Data, DataType, DataUnion};

/// The core structure of the in-memory values.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Template {
    pub name: String,
    pub data: LinkedHashMap<String, Data>
}

impl Template {
    pub fn new(name: String) -> TemplateBuilder {
        TemplateBuilder {
            name: Some(name),
            data: None,
        }
    }
}

/// The core structure of the in-memory values.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Instance {
    pub name: String,
    #[serde(skip_serializing)]
    pub template: Template,
    pub data: LinkedHashMap<String, Data>
}

impl Instance {
    pub fn new(name: String, template: Template) -> Self {
        Self {
            data: template.data.clone(),
            name,
            template,
        }
    }

    pub fn overwrite(&mut self, name: String, data: Data, loc: usize) -> Result<(), PangError> {
        match self.data.get_mut(&name) {
            Some(d) => {
                if d.data_type != data.data_type {
                    return Err(PangError::TypeMismatch(loc))
                }
                d.data = data.data;
                Ok(())
            },
            None => Err(PangError::DataNonExistent(loc)),
        }   
        
    }
}

pub struct TemplateBuilder {
    name: Option<String>,
    data: Option<LinkedHashMap<String, Data>>
}

impl TemplateBuilder {

    pub fn build(self) -> Template {
        let data = self.data.unwrap_or_default();
    
        Template {
            name: self.name.unwrap(),
            data,
        }
    }

    pub fn add_data(&self, name: String, data: Data) -> Self {
        let mut map = self.data.clone().unwrap_or_default();
        map.insert(name, data);
        Self {
            name: self.name.clone(),
            data: Some(map),
        }
    }
}