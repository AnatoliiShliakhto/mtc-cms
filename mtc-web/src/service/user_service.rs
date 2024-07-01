use std::collections::BTreeMap;
use std::iter::Map;
use mtc_model::user_details_model::UserDetailsModel;

pub trait UserService {
    fn from_string(data: &str) -> Self;
    fn remove_user(&mut self, user: &str) -> Self;
    fn get_users_json(&self) -> String;
    fn get_user_rank(&self, login: &str) -> String;
    fn get_user_name(&self, login: &str) -> String;
}

impl UserService for BTreeMap<String, UserDetailsModel> {
    fn from_string(data: &str) -> Self {
        let mut users_details = BTreeMap::<String, UserDetailsModel>::new();
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
                users_details.insert(
                    user_fields[2].trim().to_uppercase(),
                    UserDetailsModel {
                        rank: user_fields[0].trim().to_string(),
                        name: user_fields[1].trim().to_string(),
                    },
                );
            }
        }
        
        users_details
    }

    fn remove_user(&mut self, user: &str) -> Self {
        self.remove(user);
        self.to_owned()
    }

    fn get_users_json(&self) -> String {
        Map::collect(self
            .iter()
            .map(|(login, user)| format!("{:1}\t{:2}\t{:3}\n", user.rank, user.name, login)))
    }

    //todo REMOVE String operations
    fn get_user_rank(&self, login: &str) -> String {
        let key = login.to_uppercase();
        if !self.contains_key(&key) { return String::new() }
        self.get_key_value(&key).unwrap().1.rank.clone()
    }

    fn get_user_name(&self, login: &str) -> String {
        let key = login.to_uppercase();
        if !self.contains_key(&key) { return String::new() }
        self.get_key_value(&key).unwrap().1.name.clone()
    }
}