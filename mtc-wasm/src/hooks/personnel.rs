use super::*;

pub fn use_init_personnel() -> UsePersonnel {
    let user_details =
        BTreeMap::<Cow<'static, str>, UserDetails>::new();

    use_context_provider(|| UsePersonnel { users: Signal::new(user_details) })
}

pub fn use_personnel() -> UsePersonnel {
    consume_context::<UsePersonnel>()
}

pub fn use_personnel_assign_details(user_details_dto: Vec<UserDetailsDto>) {
    let mut users = use_personnel().users;
    user_details_dto.into_iter().for_each(|details| {
        if let Some(user) = users().get(&details.login) {
            users.write().insert(details.login, UserDetails {
                id: details.id,
                group: details.group,
                state: if details.blocked {
                    UserState::Inactive
                } else {
                    UserState::Active
                },
                password: details.password,
                last_access: details.last_access,
                access_count: details.access_count,
                ..user.clone()
            });
        }
    })
}

#[derive(Clone, Copy)]
pub struct UsePersonnel {
    pub users: Signal<BTreeMap<Cow<'static, str>, UserDetails>>,
}
