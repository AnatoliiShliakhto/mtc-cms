use std::collections::BTreeMap;

use serde_json::{Map, Value};

use mtc_model::user_details_model::UserDetailsModel;

pub trait UserService {
    fn import_str(&self, data: &str) -> Self;
    fn import_json(&mut self, data: &str);
    fn remove_user(&mut self, user: &str) -> Self;
    fn get_users_string(&self) -> Value;
    fn get_users_json(&self) -> Value;
    fn get_user_rank(&self, login: &str) -> String;
    fn get_user_name(&self, login: &str) -> String;
}

impl UserService for BTreeMap<String, UserDetailsModel> {
    fn import_str(&self, data: &str) -> Self {
        let mut user_details = self.clone();

        let user_array = data
            .split('\n')
            .filter(|val| !val.is_empty())
            //.map(|val| val.to_string())
            .collect::<Vec<&str>>();

        for item in user_array {
            let user_fields = item
                .split('\t')
                //                .filter(|val| !val.trim().is_empty())
                //                .map(|val| val.to_string())
                .collect::<Vec<&str>>();

            if user_fields.len().ge(&3) {
                user_details.insert(
                    user_fields[2].trim().to_uppercase(),
                    UserDetailsModel {
                        rank: user_fields[0].trim().to_string(),
                        name: user_fields[1].trim().to_string(),
                    },
                );
            }
        }

        user_details
    }

    fn import_json(&mut self, data: &str) {
        let users =
            serde_json::from_str(data).unwrap_or(BTreeMap::<String, UserDetailsModel>::new());
        users.iter().for_each(|(login, details)| {
            self.insert(login.clone(), details.clone());
        })
    }

    fn remove_user(&mut self, user: &str) -> Self {
        self.remove(user);
        self.to_owned()
    }

    fn get_users_string(&self) -> Value {
        std::iter::Map::collect::<String>(
            self.iter()
                //.map(|(login, user)| format!("{:1}\t{:2}\t{:3}\n", user.rank, user.name, login)))
                .map(|(login, details)| {
                    [&details.rank, "\t", &details.name, "\t", login, "\n"].concat()
                }),
        )
        .into()
    }

    fn get_users_json(&self) -> Value {
        Value::Object(
            self.iter()
                .map(|(login, details)| (login.clone(), serde_json::to_value(details).unwrap()))
                .collect::<Map<String, Value>>(),
        )
    }

    fn get_user_rank(&self, login: &str) -> String {
        let key = login.to_uppercase();
        if !self.contains_key(&key) {
            return String::new();
        }
        self.get_key_value(&key).unwrap().1.rank.clone()
    }

    fn get_user_name(&self, login: &str) -> String {
        let key = login.to_uppercase();
        if !self.contains_key(&key) {
            return String::new();
        }
        self.get_key_value(&key).unwrap().1.name.clone()
    }
}
