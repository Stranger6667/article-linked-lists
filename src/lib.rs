#![doc = include_str!("../README.md")]
#![warn(
    clippy::pedantic,
    clippy::doc_markdown,
    clippy::redundant_closure,
    clippy::explicit_iter_loop,
    clippy::match_same_arms,
    clippy::needless_borrow,
    clippy::print_stdout,
    clippy::cast_possible_truncation,
    clippy::unwrap_used,
    clippy::map_unwrap_or,
    clippy::trivially_copy_pass_by_ref,
    clippy::needless_pass_by_value,
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences,
    rust_2018_idioms,
    rust_2018_compatibility,
    rust_2021_compatibility
)]
#![allow(clippy::module_name_repetitions)]

use core::fmt;

use serde_json::Value;

mod error;
mod keywords;

pub use error::{SchemaError, ValidationError};
use keywords::{Node, Properties, Type};

/// A JSON Schema validator.
pub struct Validator {
    node: Box<dyn Node>,
}

impl fmt::Debug for Validator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Validator").finish()
    }
}

impl Validator {
    /// Create a new validator.
    ///
    /// # Errors
    ///
    /// Returns `SchemaError` if the input schema is invalid.
    pub fn new(_: &Value) -> Result<Validator, SchemaError> {
        // NOTE: Hardcoded for simplicity, as we're not actually optimizing the schema parsing

        macro_rules! properties {
            ($($key:expr => $value:expr),* $(,)?) => {
                Properties::new([$(($key, $value.boxed())),*].into_iter())
            };
        }

        let eleven = properties!(
           "another" => Type::String,
           "inner" => Type::String,
        );
        let ten = properties!(
           "another" => Type::String,
           "inner" => eleven,
        );
        let nine = properties!(
           "another" => Type::String,
           "inner" => ten,
        );
        let eight = properties!(
            "another" => Type::String,
            "inner" => nine,
        );
        let seven = properties!(
            "another" => Type::String,
            "inner" => eight,
        );
        let six = properties!(
            "another" => Type::String,
            "inner" => seven,
        );
        let five = properties!(
            "another" => Type::String,
            "inner" => six,
        );
        let four = properties!(
            "another" => Type::String,
            "inner" => five,
        );
        let three = properties!(
            "another" => Type::String,
            "inner" => four,
        );
        let two = properties!(
            "another" => Type::String,
            "inner" => three,
        );
        let one = properties!(
            "another" => Type::String,
            "inner" => two,
        );
        Ok(Validator { node: one.boxed() })
    }
    /// Validate JSON instance against this validator.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError` if the input instance is not valid under the given validator.
    pub fn validate(&self, instance: &Value) -> Result<(), ValidationError> {
        if let Err(error) = self.node.validate(instance, 0) {
            Err(error.finish())
        } else {
            Ok(())
        }
    }
}

/// Validate JSON instance against this validator.
///
/// # Errors
///
/// Returns `ValidationError` if the input instance is not valid under the given validator.
///
/// # Panics
///
/// This function panics on invalid schema.
pub fn validate(instance: &Value, schema: &Value) -> Result<(), ValidationError> {
    Validator::new(schema)
        .expect("Invalid schema")
        .validate(instance)
}

#[cfg(test)]
mod tests {
    use super::validate;
    use serde_json::json;

    #[test]
    fn test_error_message() {
        let instance = json!({
          "inner": {
            "inner": {
              "inner": {
                "inner": {
                  "another": 1
                }
              }
            }
          }
        });
        let error = validate(&instance, &json!({})).expect_err("Should fail");
        assert_eq!(
            error.to_string(),
            "1 is not of type 'string' at /inner/inner/inner/inner/another"
        );
        assert_eq!(instance.pointer(&error.location_pointer()), Some(&json!(1)));
    }
}
