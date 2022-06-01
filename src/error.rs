use serde::{Serialize, ser::SerializeStruct};

#[derive(Debug, Copy, Clone)]
pub enum PangError {
    SyntaxError(usize),
    InstanceAlreadyExists(usize),
    InstanceNonExistent(usize),
    TemplateAlreadyExists(usize),
    TemplateNonExistent(usize),
}

impl PangError {

    pub fn name<'a>(&'a self) -> &'a str {
        match self {
            PangError::SyntaxError(_) => "SyntaxError",
            PangError::InstanceAlreadyExists(_) => "InstanceAlreadyExists",
            PangError::InstanceNonExistent(_) => "InstanceNonExistent",
            PangError::TemplateAlreadyExists(_) => "TemplateAlreadyExists",
            PangError::TemplateNonExistent(_) => "TemplateNonExistent",
        }
    }

    pub fn location(&self) -> usize {
        match self {
            PangError::SyntaxError(loc) => *loc,
            PangError::InstanceAlreadyExists(loc) => *loc,
            PangError::InstanceNonExistent(loc) => *loc,
            PangError::TemplateAlreadyExists(loc) => *loc,
            PangError::TemplateNonExistent(loc) => *loc,
        }
    }

}

impl Serialize for PangError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut error = serializer.serialize_struct("Error", 2)?;
        error.serialize_field("Definition", self.name())?;
        error.serialize_field("Location", &self.location())?;
        error.end()
    }
}