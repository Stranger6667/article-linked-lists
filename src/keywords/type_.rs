use core::fmt;

use imbl::Vector;
use serde_json::Value;

use crate::ValidationError;

use super::Node;

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub(crate) enum Type {
    Array,
    Boolean,
    Integer,
    Null,
    Number,
    Object,
    String,
}

impl Type {
    pub(crate) fn boxed(self) -> Box<dyn Node> {
        Box::new(self)
    }
}

impl Node for Type {
    fn validate(&self, instance: &Value, path: Vector<&str>) -> Result<(), ValidationError> {
        match (self, instance) {
            (Type::Array, Value::Array(_))
            | (Type::Null, Value::Null)
            | (Type::Boolean, Value::Bool(_))
            | (Type::Number, Value::Number(_))
            | (Type::Object, Value::Object(_))
            | (Type::String, Value::String(_)) => Ok(()),
            (Type::Integer, Value::Number(n)) if n.is_i64() || n.is_u64() => Ok(()),
            _ => Err(ValidationError::new(
                format!("{instance} is not of type '{self}'"),
                path.into_iter(),
            )),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Array => f.write_str("array"),
            Type::Boolean => f.write_str("boolean"),
            Type::Integer => f.write_str("integer"),
            Type::Null => f.write_str("null"),
            Type::Number => f.write_str("number"),
            Type::Object => f.write_str("object"),
            Type::String => f.write_str("string"),
        }
    }
}
