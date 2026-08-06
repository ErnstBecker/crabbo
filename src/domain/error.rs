use std::fmt;

#[derive(Debug)]
pub enum AccountError {
    NotFound,
    RequestFailed(String),
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountError::NotFound => write!(f, "Address not found or invalid"),
            AccountError::RequestFailed(msg) => write!(f, "{}", msg),
        }
    }
}
