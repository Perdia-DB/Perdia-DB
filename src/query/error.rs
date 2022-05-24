
/// Possible errors in requests.
#[derive(Debug)]
pub enum RequestError {
    TemplateNonExistent,
    TemplateAlreadyExists,
    InstanceNonExistent,
    InstanceAlreadyExists,
    SyntaxError,
    SerializationError
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for RequestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}