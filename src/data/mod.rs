use std::sync::Mutex;

use lazy_static::lazy_static;

use super::data::template::{TemplateBuilder, Template};

pub mod template;

lazy_static! {
    pub static ref TEMPLATES: Vec<Template> = Vec::new();
    pub static ref INSTANCES: Vec<Template> = Vec::new();
    pub static ref NEW_TEMPLATE: Mutex<Option<TemplateBuilder>> = Mutex::new(None);
}