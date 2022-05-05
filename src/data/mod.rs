use std::sync::Mutex;

use lazy_static::lazy_static;

use super::data::template::{TemplateBuilder, Template};

pub mod template;
pub mod serialization;

lazy_static! {
    pub static ref TEMPLATES:  Mutex<Vec<Template>> = Mutex::new(Vec::new());
    pub static ref INSTANCES:  Mutex<Vec<Template>> = Mutex::new(Vec::new());
}