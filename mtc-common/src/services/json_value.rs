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

#[cfg(test)]
mod tests {
    use serde_json::json;
    use super::*;

    static JSON_TEST_VALUE: &'static str = r#"
    {
        "bool_key": true,
        "i64_key": 12345,
        "string_key": "test",
        "datetime_key": "1970-01-01T00:00:00Z",
        "object_key": { "id": 0, "val": false }
    }
    "#;

    #[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
    struct TestService {
        pub id: i32,
        pub val: bool,
    }

    #[test]
    fn json_value_service() {
        let mut value = serde_json::from_str::<Value>(JSON_TEST_VALUE).unwrap();
        let some_value = Some(value.clone());

        //contains_key
        assert_eq!(value.contains_key("string_key"), true);
        assert_ne!(value.contains_key("dummy"), true);

        //insert_value
        value.insert_value("inserted", json!(TestService::default()));
        assert_eq!(value.contains_key("inserted"), true);

        //self_obj
        assert_eq!(
            json!(TestService::default()).self_obj::<TestService>(),
            Some(TestService::default())
        );
        assert_eq!(
            Some(json!(TestService::default())).self_obj::<TestService>(),
            Some(TestService::default())
        );
        assert_eq!(
            value.clone().self_obj::<TestService>(), None);

        //key_obj
        assert_eq!(
            value.clone().key_obj::<TestService>("inserted"),
            Some(TestService::default())
        );
        assert_eq!(
            Some(value.clone()).key_obj::<TestService>("inserted"),
            Some(TestService::default())
        );
        assert_eq!(value.clone().key_obj::<TestService>("dummy"), None);

        let str_value = Some(Cow::Borrowed("test"));
        //self_str
        assert_eq!(json!(Value::from("test")).self_str(), str_value);
        assert_eq!(Some(json!(Value::from("test"))).self_str(), str_value);
        assert_eq!(json!(Value::Null).self_str(), None);

        //key_str
        assert_eq!(value.key_str("string_key"), str_value);
        assert_eq!(some_value.key_str("string_key"), str_value);
        assert_eq!(value.key_str("dummy"), None);

        let string_value = Some("test".to_string());
        //self_string
        assert_eq!(json!(Value::from("test")).self_string(), string_value);
        assert_eq!(Some(json!(Value::from("test"))).self_string(), string_value);
        assert_eq!(json!(Value::Null).self_string(), None);

        //key_string
        assert_eq!(value.key_string("string_key"), string_value);
        assert_eq!(some_value.key_string("string_key"), string_value);
        assert_eq!(value.key_string("dummy"), None);

        //self_bool
        assert_eq!(json!(Value::Bool(true)).self_bool(), Some(true));
        assert_eq!(Some(json!(Value::Bool(true))).self_bool(), Some(true));
        assert_eq!(json!(Value::Null).self_bool(), None);

        //key_bool
        assert_eq!(value.key_bool("bool_key"), Some(true));
        assert_eq!(some_value.key_bool("bool_key"), Some(true));
        assert_eq!(value.key_bool("dummy"), None);

        //self_i64
        assert_eq!(json!(Value::from(12345)).self_i64(), Some(12345i64));
        assert_eq!(Some(json!(Value::from(12345))).self_i64(), Some(12345i64));
        assert_eq!(json!(Value::Null).self_i64(), None);

        //key_i64
        assert_eq!(value.key_i64("i64_key"), Some(12345i64));
        assert_eq!(some_value.key_i64("i64_key"), Some(12345i64));
        assert_eq!(value.key_i64("dummy"), None);

        //self_datetime
        let datetime = chrono::DateTime::<Local>::default();
        let datetime_value = json!(Value::from(datetime.to_string()));
        assert_eq!(datetime_value.clone().self_datetime(), Some(datetime));
        assert_eq!(Some(datetime_value).self_datetime(), Some(datetime));
        assert_eq!(json!(Value::Null).self_datetime(), None);

        //key_datetime
        assert_eq!(value.key_datetime("datetime_key"), Some(datetime));
        assert_eq!(some_value.key_datetime("datetime_key"), Some(datetime));
        assert_eq!(value.key_datetime("dummy"), None);
    }
}