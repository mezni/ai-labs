#[derive(Debug)]
pub enum Error {
    NotFound(String),
    Validation(String),
    Database(String),
    Auth(String),
}
