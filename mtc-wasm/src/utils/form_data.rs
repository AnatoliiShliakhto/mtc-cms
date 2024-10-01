use super::*;

pub trait FormDataUtils {
    fn get_str(&self, field: &str) -> Option<Cow<'static, str>>;
    fn get_bool(&self, field: &str) -> bool;
    fn get_str_array(&self, field: &str) -> Option<Vec<Cow<'static, str>>>;
    fn get_i64(&self, field: &str) -> Option<i64>;
    fn get_fields_array(&self) -> Option<Vec<Field>>;
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

    fn get_fields_array(&self) -> Option<Vec<Field>> {
        let mut fields: Vec<Field> = vec![];

        let kind_set = self.get_str_array("fields-kind").unwrap_or_default();
        let slug_set = self.get_str_array("fields-slug").unwrap_or_default();
        let title_set = self.get_str_array("fields-title").unwrap_or_default();

        for (kind, (slug, title))
        in zip(kind_set, zip(slug_set, title_set)) {
            fields.push(Field {
                kind: FieldKind::from_str(&*kind).unwrap_or_default(),
                slug,
                title,
            })
        }

        if fields.is_empty() { return None }
        Some(fields)
    }
}