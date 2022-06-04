use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use crate::data::structure::Instance;

use super::data::structure::Template;

pub mod structure;
pub mod serialization;

lazy_static! {
    pub static ref TEMPLATES:  Arc<Mutex<Vec<Template>>> = Arc::new(Mutex::new(Vec::new()));
    pub static ref INSTANCES:  Arc<Mutex<Vec<Instance>>> = Arc::new(Mutex::new(Vec::new()));
}