use std::fmt;

#[derive(Debug)]
pub enum BoothRequestError {
    InvalidUrl(String),
    NotBoothUrl(),
    HttpError(u16, String),
    NetworkError(String),
    ParseError(String),
}

impl fmt::Display for BoothRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            BoothRequestError::InvalidUrl(s) => write!(f, "Invalid URL: {}", s),
            BoothRequestError::NotBoothUrl() => write!(f, "Not a booth.pm URL!"),
            BoothRequestError::HttpError(status_code, reason) => write!(f, "Request failed: {} - {}", status_code, reason),
            BoothRequestError::NetworkError(s) => write!(f ,"Network error: {}", s),
            BoothRequestError::ParseError(s) => write!(f, "Parsing error: {}", s) 
        };
    }
}
