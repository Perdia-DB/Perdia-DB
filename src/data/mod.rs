use lazy_static::lazy_static;

use super::data::template::Template;

pub mod template;

lazy_static! {
    pub static ref TEMPLATES: Vec<Template> = Vec::new();
}