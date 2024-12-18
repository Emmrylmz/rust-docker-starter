use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    InvalidInput(String),
    ProductNotFound(u32),
    InsufficientStock { product_id: u32, available: u32 },
    JsonParseError(String),
    IOError(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            CustomError::ProductNotFound(id) => write!(f, "Product with ID {} not found", id),
            CustomError::InsufficientStock { product_id, available } => write!(
                f,
                "Insufficient stock for product ID {}. Available: {}",
                product_id, available
            ),
            CustomError::JsonParseError(msg) => write!(f, "JSON parsing error: {}", msg),
            CustomError::IOError(msg) => write!(f, "I/O error: {}", msg),
        }
    }
}

impl std::error::Error for CustomError {}
