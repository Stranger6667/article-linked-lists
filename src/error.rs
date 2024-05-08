use core::fmt;
use std::error;

/// Indicates invalid schema.
#[derive(Debug)]
pub struct SchemaError {
    message: String,
}

impl fmt::Display for SchemaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl error::Error for SchemaError {}

/// Error that may happen during input validation.
#[derive(Debug)]
pub struct ValidationError {
    message: String,
    location: Vec<String>,
}

impl ValidationError {
    /// Create new validation error.
    pub fn new(
        message: impl Into<String>,
        location: impl Iterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            message: message.into(),
            location: location.map(Into::into).collect(),
        }
    }
    /// JSON Pointer to the location of the error.
    pub fn location_pointer(&self) -> String {
        let mut pointer = String::new();
        for segment in &self.location {
            pointer.push('/');
            pointer.push_str(segment);
        }
        pointer
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)?;
        f.write_str(" at ")?;
        for segment in self.location.iter() {
            f.write_str("/")?;
            f.write_str(segment)?;
        }
        Ok(())
    }
}

impl error::Error for ValidationError {}
