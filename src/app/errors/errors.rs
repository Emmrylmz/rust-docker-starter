use std::fmt::{self, write};

#[derive(Debug)]
pub enum CustomError {
    InsufficientQuantity {
        product_id: u32,
        required: u32,
        available: u32,
    },
    ProductNotFound(u32),
    JsonParseError(String),
    AuthenticationError(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::InsufficientQuantity {
                product_id,
                required,
                available,
            } => {
                write!(
                    f,
                    "Error: Product {} has insufficient quantity. Required: {}, Available: {}",
                    product_id, required, available
                )
            }
            CustomError::ProductNotFound(id) => write!(f, "Product with ID {} not found.", id),
            CustomError::JsonParseError(msg) => write!(f, "JSON Parse Error: {}", msg),
            CustomError::AuthenticationError(msg) => write!(f,"Authentication Error {}", msg),
        }
    }
}

impl std::error::Error for CustomError {}
