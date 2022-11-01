#[derive(Debug)]
pub(crate) struct AnimalError {
    details: String,
}

impl AnimalError {
    pub fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

impl std::error::Error for AnimalError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl std::fmt::Display for AnimalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}
