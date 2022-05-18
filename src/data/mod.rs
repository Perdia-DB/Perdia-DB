use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use super::data::template::Template;

pub mod template;
pub mod serialization;

lazy_static! {
    pub static ref TEMPLATES:  Arc<Mutex<Vec<Template>>> = Arc::new(Mutex::new(Vec::new()));
    pub static ref INSTANCES:  Arc<Mutex<Vec<Template>>> = Arc::new(Mutex::new(Vec::new()));
}