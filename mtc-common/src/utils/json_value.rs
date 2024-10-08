use super::*;

pub trait JsonValueUtils {
    fn has_key(&self, key: &str) -> bool;
    fn insert_string(&mut self, key: &str, string: Cow<'static, str>);
    fn insert_value(&mut self, key: &str, value: Value);

    fn get_str(&self, key: &str) -> Option<Cow<'static, str>>;
    fn get_string(&self, key: &str) -> Option<String>;
    fn get_object<T: DeserializeOwned>(&self, key: &str) -> Option<T>;
    fn get_value(&self, key: &str) -> Option<Value>;
    fn get_datetime(&self, key: &str) -> Option<DateTime<Local>>;
    fn get_i64(&self, key: &str) -> Option<i64>;
    fn get_bool(&self, key: &str) -> Option<bool>;
    fn get_str_array(&self, key: &str) -> Option<Vec<Cow<'static, str>>>;
    fn get_entries(&self, key: &str) -> Option<Vec<Entry>>;
    fn get_schema_kind(&self) -> SchemaKind;
    fn get_schema_fields(&self) -> Option<Vec<Field>>;
}

impl JsonValueUtils for Value {
    fn has_key(&self, key: &str) -> bool {
        self
            .as_object()
            .map(|obj| obj.contains_key(key)).unwrap_or_else(|| false)
    }

    fn insert_string(&mut self, key: &str, string: Cow<'static, str>) {
        if let Some(obj) = self.as_object_mut() {
            obj.insert(key.to_owned(), Value::String(string.into()));
        }
    }

    fn insert_value(&mut self, key: &str, value: Value) {
        if let Some(obj) = self.as_object_mut() {
            obj.insert(key.to_owned(), value);
        }
    }

    fn get_str(&self, key: &str) -> Option<Cow<'static, str>> {
        match self
            .as_object()
            .and_then(|obj| obj.get(key).and_then(|val| val.as_str()))
        {
            Some(value) => Some(Cow::Owned(value.into())),
            _ => None,
        }
    }

    fn get_string(&self, key: &str) -> Option<String> {
        match self
            .as_object()
            .and_then(|obj| obj.get(key).and_then(|val| val.as_str()))
        {
            Some(value) => Some(value.into()),
            _ => None,
        }
    }

    fn get_object<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        match self
            .as_object()
            .and_then(|obj| obj.get(key))
            .map(|val| serde_json::from_value::<T>(val.to_owned()))
        {
            Some(Ok(value)) => Some(value),
            _ => None,
        }
    }

    fn get_value(&self, key: &str) -> Option<Value> {
        self
            .as_object()
            .and_then(|obj| obj.get(key)
                .and_then(|val| Some(val.to_owned())))
    }

    fn get_datetime(&self, key: &str) -> Option<DateTime<Local>> {
        if let Some(value) = self
            .as_object()
            .and_then(|obj| obj.get(key).and_then(|val| val.as_str())) {
            if let Ok(value) = value.parse::<DateTime<Local>>() {
                return Some(value);
            }
        }
        None
    }

    fn get_i64(&self, key: &str) -> Option<i64> {
        self
            .as_object()
            .and_then(|obj| obj.get(key)
                .and_then(|val| val.as_i64()))
    }

    fn get_bool(&self, key: &str) -> Option<bool> {
        self
            .as_object()
            .and_then(|obj| obj.get(key)
                .and_then(|val| val.as_bool()))
    }

    fn get_str_array(&self, key: &str) -> Option<Vec<Cow<'static, str>>> {
        if let Some(array) = self
            .as_object()
            .and_then(|obj| obj.get(key)
                .and_then(|val| val.as_array())) {
            return Some(array
                .iter()
                .map(|val| {
                    if let Some(string) = val.as_str() {
                        Cow::Owned(string.to_owned())
                    } else {
                        Cow::Borrowed("")
                    }
                })
                .collect::<Vec<Cow<'static, str>>>());
        }
        None
    }

    fn get_entries(&self, key: &str) -> Option<Vec<Entry>> {
        if let Some(array) = self
            .as_object()
            .and_then(|obj| obj.get(key)
                .and_then(|val| val.as_array())) {
            let mut entries = vec![];
            for item in array {
                if let Ok(val) = serde_json::from_value::<Entry>(item.to_owned()) {
                    entries.push(val)
                }
            }

            return Some(entries);
        }
        None
    }

    fn get_schema_kind(&self) -> SchemaKind {
        if let Some(value) = self.as_object()
            .and_then(|obj| obj.get("kind")
                .and_then(|val| val.as_i64())) {
            return SchemaKind::from(value);
        }
        SchemaKind::Page
    }

    fn get_schema_fields(&self) -> Option<Vec<Field>> {
        if let Some(array) = self
            .as_object()
            .and_then(|obj| obj.get("fields")
                .and_then(|val| val.as_array())) {
            let mut fields = vec![];
            for item in array {
                if let Ok(val) = serde_json::from_value::<Field>(item.to_owned()) {
                    fields.push(val)
                }
            }

            return Some(fields);
        }
        None
    }
}