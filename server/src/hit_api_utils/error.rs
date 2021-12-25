use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum YelpAPIAccessError {
    InternalError,
    InternalErrorWithMessage(String),
}

impl StdError for YelpAPIAccessError {}

impl fmt::Display for YelpAPIAccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YelpAPIAccessError::InternalError => {
                write!(f, "Hotpepper API access Error in use case!")
            }
            YelpAPIAccessError::InternalErrorWithMessage(message) => write!(f, "{}", message),
        }
    }
}

pub trait UseCase {
    fn parse_data_access_result<T>(
        &self,
        result: Result<T, Box<dyn StdError>>,
    ) -> Result<T, YelpAPIAccessError> {
        match result {
            Ok(data) => Ok(data),
            Err(_) => Err(YelpAPIAccessError::InternalError),
        }
    }
}
