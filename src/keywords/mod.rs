use serde_json::Value;

use crate::{JsonPointerNode, ValidationError};

pub(crate) mod properties;
pub(crate) mod type_;

pub(crate) use properties::Properties;
pub(crate) use type_::Type;

pub(crate) trait Node {
    fn validate<'a>(
        &self,
        instance: &'a Value,
        path: JsonPointerNode<'a, '_>,
    ) -> Result<(), ValidationError>;
}
