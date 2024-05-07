use serde_json::Value;

use crate::ValidationError;

pub(crate) mod properties;
pub(crate) mod type_;

pub(crate) use properties::Properties;
pub(crate) use type_::Type;

pub(crate) trait Node {
    fn validate(&self, instance: &Value) -> Result<(), ValidationError>;
}
