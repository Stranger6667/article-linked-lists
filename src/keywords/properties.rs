use imbl::Vector;
use serde_json::Value;

use crate::{Node, ValidationError};

pub(crate) struct Properties {
    properties: Vec<(String, Box<dyn Node>)>,
}

impl Properties {
    pub(crate) fn new(
        properties: impl Iterator<Item = (impl Into<String>, Box<dyn Node>)>,
    ) -> Self {
        Self {
            properties: properties.map(|(k, v)| (k.into(), v)).collect(),
        }
    }
    pub(crate) fn boxed(self) -> Box<dyn Node> {
        Box::new(self)
    }
}

impl Node for Properties {
    fn validate(&self, instance: &Value, path: Vector<&str>) -> Result<(), ValidationError> {
        if let Value::Object(object) = instance {
            for (key, value) in &self.properties {
                if let Some((key, instance)) = object.get_key_value(key) {
                    let mut path = path.clone();
                    path.push_back(key);
                    value.validate(instance, path)?;
                }
            }
        }
        Ok(())
    }
}
