use super::Error;

#[derive(Clone, Debug, Default)]
pub struct SquareApiError {
    pub message: String,
    pub errors: Vec<Error>,
}

impl SquareApiError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
            ..Default::default()
        }
    }

    pub fn with_response_errors(message: &str, errors: &[Error]) -> Self {
        Self {
            message: message.to_owned(),
            errors: errors.to_vec(),
        }
    }
}

impl std::fmt::Display for SquareApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "API Error: {:?}", self)
    }
}

impl std::error::Error for SquareApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
