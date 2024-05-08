use serde_json::Value;

use crate::ValidationError;

pub(crate) mod properties;
pub(crate) mod type_;

pub(crate) use properties::Properties;
pub(crate) use type_::Type;

pub(crate) trait Node {
    fn validate<'a>(
        &self,
        instance: &'a Value,
        path: &mut Vec<&'a str>,
    ) -> Result<(), ValidationError>;
}
