use dioxus::prelude::*;
use serde_json::Value;

use mtc_model::api_model::ApiModel;

pub trait ContentService {
    fn extract_field(&self, field: &str) -> Value;
}

impl ContentService for Signal<ApiModel> {
    fn extract_field(&self, field: &str) -> Value {
        let mut result = Value::default();
        if let Some(value) = &self.read().fields {
            if let Value::Object(fields) = value {
                if let Some(val) = fields.get(field) {
                    result = val.clone()
                } 
            }
        }

        result
    }
}
