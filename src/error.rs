#[derive(Debug, Copy, Clone)]
pub enum PangError {
    SyntaxError(usize),
    InstanceAlreadyExists(usize),
    InstanceNonExistent(usize),
    TemplateAlreadyExists(usize),
    TemplateNonExistent(usize),
}