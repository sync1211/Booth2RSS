use std::fmt;

#[derive(Debug)]
pub enum RequestError {
    InvalidUrl(String),
    NotBoothUrl(),
    HttpError(u16, String),
    NetworkError(String),
    ParseError(String),
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            RequestError::InvalidUrl(s) => write!(f, "Invalid URL: {}", s),
            RequestError::NotBoothUrl() => write!(f, "Not a booth.pm URL!"),
            RequestError::HttpError(status_code, reason) => write!(f, "Request failed: {} - {}", status_code, reason),
            RequestError::NetworkError(s) => write!(f ,"Network error: {}", s),
            RequestError::ParseError(s) => write!(f, "Parsing error: {}", s) 
        };
    }
}