use std::collections::HashMap;

use dioxus::events::FormValue;
use dioxus::prelude::*;

pub trait ValidatorService {
    fn is_field_empty(&self, field: &str) -> bool;
    fn is_slug_valid(&self) -> bool;
    fn is_string_valid(&self, field: &str, min_len: usize) -> bool;
    fn get_string(&self, field: &str) -> String;
    fn get_string_option(&self, field: &str) -> Option<String>;
}

impl ValidatorService for Signal<HashMap<String, FormValue>> {
    fn is_field_empty(&self, field: &str) -> bool {
        match self.read().get(field) {
            Some(FormValue(field)) => field.is_empty() || field[0].len().eq(&0),
            None => true,
        }
    }

    fn is_slug_valid(&self) -> bool {
        match self.read().get("slug") {
            Some(FormValue(field)) => !field.is_empty() && field[0].len().ge(&5),
            None => false,
        }
    }

    fn is_string_valid(&self, field: &str, min_len: usize) -> bool {
        match self.read().get(field) {
            Some(FormValue(field)) => !field.is_empty() && field[0].chars().count().ge(&min_len),
            None => false,
        }
    }

    fn get_string(&self, field: &str) -> String {
        match self.read().get(field) {
            Some(FormValue(field)) => field[0].clone(),
            None => String::new(),
        }
    }

    fn get_string_option(&self, field: &str) -> Option<String> {
        let result = self
            .read()
            .get(field)
            .map(|FormValue(field)| field[0].clone());
        if let Some(value) = &result {
            if value.is_empty() {
                return None;
            }
        }
        result
    }
}
