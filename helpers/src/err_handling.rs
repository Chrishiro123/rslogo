use std::fmt;

#[derive(Debug)]
pub struct LogoError;

impl fmt::Display for LogoError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error occurered during parsing logo!")
    }
}

impl std::error::Error for LogoError {}