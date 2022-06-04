use serde::{Serialize, ser::SerializeStruct};

#[derive(Debug, Clone)]
pub enum PangError {
    SyntaxError(usize),
    InstanceAlreadyExists(String, usize),
    InstanceNonExistent(String, usize),
    TemplateAlreadyExists(String, usize),
    TemplateNonExistent(String, usize),
    TypeMismatch(usize),
    DataNonExistent(usize),
    ExecutionError,
}

impl PangError {

    /// Detailed description of the error.
    pub fn desc<'a>(&'a self) -> String {
        match self {
            PangError::ExecutionError => "Something unexpected went wrong during execution :/".to_string(),
            PangError::SyntaxError(_) => "Syntax error occurred".to_string(),
            PangError::InstanceAlreadyExists(name, _) => 
                format!("Instance {} already exists.", name).to_string(),
            PangError::InstanceNonExistent(name, _) => 
                format!("Instance {} doesn't exist.", name).to_string(),
            PangError::TemplateAlreadyExists(name, _) => 
                format!("Template {} already exists.", name).to_string(),
            PangError::TemplateNonExistent(name, _) => 
                format!("Template {} doesn't exist.", name).to_string(),
            PangError::TypeMismatch(_) => "Unexpected data type.".to_string(),
            PangError::DataNonExistent(_) => "Data point doesn't exist in Template/Instance.".to_string(),
        }
    }

    /// Location where the error occurred.
    pub fn loc(&self) -> usize {
        match self {
            PangError::ExecutionError => usize::MAX,
            PangError::SyntaxError(loc) => *loc,
            PangError::InstanceAlreadyExists(_, loc) => *loc,
            PangError::InstanceNonExistent(_, loc) => *loc,
            PangError::TemplateAlreadyExists(_, loc) => *loc,
            PangError::TemplateNonExistent(_, loc) => *loc,
            PangError::TypeMismatch(loc) => *loc,
            PangError::DataNonExistent(loc) => *loc,
        }
    }

    pub fn code(&self) -> usize {
        match self {
            PangError::ExecutionError => 0b0000,
            PangError::SyntaxError(_) => 0b1111,
            PangError::InstanceAlreadyExists(_, _) => 0b0101,
            PangError::InstanceNonExistent(_, _) => 0b0110,
            PangError::TemplateAlreadyExists(_, _) => 0b1001,
            PangError::TemplateNonExistent(_, _) => 0b1010,
            PangError::TypeMismatch(_) => 0b0001,
            PangError::DataNonExistent(_) => 0b0010,
        }
    }

}

impl Serialize for PangError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut error = serializer.serialize_struct("Error", 3)?;
        error.serialize_field("Description", &self.desc())?;
        error.serialize_field("Location", &self.loc())?;
        error.serialize_field("Code", &self.code())?;
        error.end()
    }
}