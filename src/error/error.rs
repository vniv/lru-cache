use std::fmt::Display;

#[derive(Debug)]
pub enum MyError {
    IoError(std::io::Error),
    Custom(String),
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::IoError(error) => write!(f, "IoError: {error}"),
            MyError::Custom(custom) => write!(f, "{custom}"),
        }
    }
}

impl From<std::io::Error> for MyError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}