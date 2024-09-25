use super::*;

pub trait FormDataUtils {
    fn get_str(&self, field: &str) -> Option<Cow<'static, str>>;
    fn get_bool(&self, field: &str) -> bool;
    fn get_str_array(&self, field: &str) -> Option<Vec<Cow<'static, str>>>;
    fn get_i64(&self, field: &str) -> Option<i64>;
}

impl FormDataUtils for Event<FormData> {
    fn get_str(&self, field: &str) -> Option<Cow<'static, str>> {
        if let Some(value) = self.values().get(field) {
            if !value.0.is_empty() {
                return Some(value.0[0].to_owned().into())
            }
        }
        None
    }

    fn get_bool(&self, field: &str) -> bool {
        self.values().contains_key(field)
    }

    fn get_str_array(&self, field: &str) -> Option<Vec<Cow<'static, str>>> {
        if let Some(FormValue(value)) = self.values().get(field) {
            return Some(value
                .iter()
                .map(|val| val.to_owned().into())
                .collect::<Vec<Cow<'static, str>>>());
        }
        None
    }

    fn get_i64(&self, field: &str) -> Option<i64> {
        if let Some(FormValue(value)) = self.values().get(field) {
            if let Ok(value) = value[0].parse::<i64>() {
                return Some(value);
            }
        }
        None
    }
}