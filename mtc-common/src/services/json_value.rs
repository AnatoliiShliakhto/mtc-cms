use super::*;

pub trait JsonValueService {
    fn contains_key(&self, key: &str) -> bool;
    fn insert_value(&mut self, key: &str, value: Value);

    fn self_obj<T: DeserializeOwned>(&self) -> Option<T>;
    fn key_obj<T: DeserializeOwned>(&self, key: &str) -> Option<T>;

    fn self_str(&self) -> Option<Cow<'static, str>>;
    fn key_str(&self, key: &str) -> Option<Cow<'static, str>>;

    fn self_string(&self) -> Option<String>;
    fn key_string(&self, key: &str) -> Option<String>;

    fn self_datetime(&self) -> Option<DateTime<Local>>;
    fn key_datetime(&self, key: &str) -> Option<DateTime<Local>>;

    fn self_bool(&self) -> Option<bool>;
    fn key_bool(&self, key: &str) -> Option<bool>;

    fn self_i64(&self) -> Option<i64>;
    fn key_i64(&self, key: &str) -> Option<i64>;
}

impl JsonValueService for Value {
    fn contains_key(&self, key: &str) -> bool {
        self
            .as_object()
            .map_or(false, |obj| obj.contains_key(key))
    }

    fn insert_value(&mut self, key: &str, value: Value) {
        if let Some(obj) = self.as_object_mut() {
            obj.insert(key.to_owned(), value);
        }
    }

    fn self_obj<T: DeserializeOwned>(&self) -> Option<T> {
        serde_json::from_value::<T>(self.to_owned()).ok()
    }

    fn key_obj<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        self
            .as_object()
            .and_then(|obj| obj.get(key))
            .and_then(|val| serde_json::from_value::<T>(val.to_owned()).ok())
    }

    fn self_str(&self) -> Option<Cow<'static, str>> {
        self.as_str().and_then(|val| Some(val.to_owned().into()))
    }

    fn key_str(&self, key: &str) -> Option<Cow<'static, str>> {
        self
            .as_object()
            .and_then(|obj| obj.get(key).and_then(|val| val.as_str()))
            .and_then(|val| Some(val.to_owned().into()))
    }

    fn self_string(&self) -> Option<String> {
        self.as_str().and_then(|val| Some(val.to_string()))
    }

    fn key_string(&self, key: &str) -> Option<String> {
        self
            .as_object()
            .and_then(|obj| obj.get(key).and_then(|val| val.as_str()))
            .and_then(|val| Some(val.to_owned()))
    }

    fn self_datetime(&self) -> Option<DateTime<Local>> {
        self.as_str()
            .and_then(|val| val.parse::<DateTime<Local>>().ok())
    }

    fn key_datetime(&self, key: &str) -> Option<DateTime<Local>> {
        self
            .as_object()
            .and_then(|obj| obj.get(key).and_then(|val| val.as_str()))
            .and_then(|val| val.parse::<DateTime<Local>>().ok())
    }

    fn self_bool(&self) -> Option<bool> {
        self.as_bool()
    }

    fn key_bool(&self, key: &str) -> Option<bool> {
        self
            .as_object()
            .and_then(|obj| obj.get(key).and_then(|val| val.as_bool()))
    }

    fn self_i64(&self) -> Option<i64> {
        self.as_i64()
    }

    fn key_i64(&self, key: &str) -> Option<i64> {
        self
            .as_object()
            .and_then(|obj| obj.get(key).and_then(|val| val.as_i64()))
    }
}

impl JsonValueService for Option<Value> {
    fn contains_key(&self, key: &str) -> bool
    {
        let Some(value) = self else { return false };
        value
            .as_object()
            .map_or(false, |obj| obj.contains_key(key))

    }

    fn insert_value(&mut self, key: &str, value: Value) {
        let Some(val)= self else { return };
        if let Some(obj) = val.as_object_mut() {
            obj.insert(key.to_owned(), value);
        }
    }

    fn self_obj<T: DeserializeOwned>(&self) -> Option<T> {
        let Some(value)= self else { return None };
        serde_json::from_value::<T>(value.to_owned()).ok()
    }

    fn key_obj<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        let Some(value)= self else { return None };
        value
            .as_object()
            .and_then(|obj| obj.get(key))
            .and_then(|val| serde_json::from_value::<T>(val.to_owned()).ok())
    }

    fn self_str(&self) -> Option<Cow<'static, str>> {
        let Some(value)= self else { return None };
        value.as_str().and_then(|val| Some(val.to_owned().into()))
    }

    fn key_str(&self, key: &str) -> Option<Cow<'static, str>> {
        let Some(value)= self else { return None };
        value
            .as_object()
            .and_then(|obj| obj.get(key).and_then(|val| val.as_str()))
            .and_then(|val| Some(val.to_owned().into()))
    }

    fn self_string(&self) -> Option<String> {
        let Some(value)= self else { return None };
        value.as_str().and_then(|val| Some(val.to_owned()))
    }

    fn key_string(&self, key: &str) -> Option<String> {
        let Some(value)= self else { return None };
        value
            .as_object()
            .and_then(|obj| obj.get(key).and_then(|val| val.as_str()))
            .and_then(|val| Some(val.to_owned()))
    }

    fn self_datetime(&self) -> Option<DateTime<Local>> {
        let Some(value)= self else { return None };
        value.as_str()
            .and_then(|val| val.parse::<DateTime<Local>>().ok())
    }

    fn key_datetime(&self, key: &str) -> Option<DateTime<Local>> {
        let Some(value)= self else { return None };
        value
            .as_object()
            .and_then(|obj| obj.get(key).and_then(|val| val.as_str()))
            .and_then(|val| val.parse::<DateTime<Local>>().ok())
    }

    fn self_bool(&self) -> Option<bool> {
        let Some(value)= self else { return None };
        value.as_bool()
    }

    fn key_bool(&self, key: &str) -> Option<bool> {
        let Some(value)= self else { return None };
        value
            .as_object()
            .and_then(|obj| obj.get(key).and_then(|val| val.as_bool()))
    }

    fn self_i64(&self) -> Option<i64> {
        let Some(value)= self else { return None };
        value.as_i64()
    }

    fn key_i64(&self, key: &str) -> Option<i64> {
        let Some(value)= self else { return None };
        value
            .as_object()
            .and_then(|obj| obj.get(key).and_then(|val| val.as_i64()))
    }
}