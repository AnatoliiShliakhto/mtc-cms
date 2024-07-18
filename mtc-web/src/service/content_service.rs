use dioxus::prelude::*;
use serde_json::Value;

use mtc_model::api_model::ApiModel;

pub trait ContentService {
    fn extract_string(&self, field: &str) -> String;
}

impl ContentService for Signal<ApiModel> {
    fn extract_string(&self, field: &str) -> String {
        let mut result = String::new();
        if let Some(value) = self.peek().fields.clone() {
            if let Value::Object(fields) = value {
                if let Some(Value::String(val)) = fields.get(field) {
                    result = val.clone()
                } 
            }
        }

        result
    }
}
