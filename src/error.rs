#[derive(Debug)]
pub enum ErrorType {
    IOError(u64, String),
    RuntimeError(u64, String),
    ScanError(u64, String),
}

#[derive(Debug)]
pub struct Error(pub u64, pub String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: line {}, message: {}", self.0, self.1)
    }
}

impl std::error::Error for Error {}
