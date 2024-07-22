use dioxus::prelude::*;

pub trait ValidatorService {
    fn is_slug_valid(&self) -> bool;
    fn is_title_valid(&self) -> bool;
    fn is_login_valid(&self) -> bool;
    fn is_string_valid(&self, field: &str, min_len: usize) -> bool;
    fn get_string(&self, field: &str) -> String;
    fn get_string_option(&self, field: &str) -> Option<String>;
}

impl ValidatorService for Event<FormData> {
    fn is_slug_valid(&self) -> bool {
        match self.values().get("slug") {
            Some(value) => value.0[0].len().ge(&4),
            _ => false,
        }
    }

    fn is_title_valid(&self) -> bool {
        match self.values().get("title") {
            Some(value) => value.0[0].len().ge(&4),
            _ => false,
        }
    }

    fn is_login_valid(&self) -> bool {
        match self.values().get("login") {
            Some(value) => value.0[0].len().ge(&4),
            _ => false,
        }
    }

    fn is_string_valid(&self, field: &str, min_len: usize) -> bool {
        match self.values().get(field) {
            Some(value) => value.0[0].len().ge(&min_len),
            _ => false,
        }
    }

    fn get_string(&self, field: &str) -> String {
        match self.values().get(field) {
            Some(value) => value.0[0].clone(),
            _ => String::new(),
        }
    }

    fn get_string_option(&self, field: &str) -> Option<String> {
        self.values().get(field).map(|value| value.0[0].clone())
    }
}
