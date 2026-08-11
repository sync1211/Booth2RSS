#[derive(Debug)]
pub enum BoothRequestError {
    InvalidUrl(String),
    NotBoothUrl(String),
    HttpError(u16, String),
    ParseError(String),
}