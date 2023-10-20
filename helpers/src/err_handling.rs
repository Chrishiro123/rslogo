use std::fmt;

#[derive(Debug)]
pub struct LogoError {
    message: String,
}

impl LogoError {
    pub fn new(mess: String) -> Self {
        Self { message: mess }
    }
}

impl fmt::Display for LogoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LogoError {}
