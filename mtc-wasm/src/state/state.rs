use super::*;

/// Initializes the application state with the provided internationalization (i18n) data.
///
/// This function parses the given i18n string data, expected to be in a key=value CSV format,
/// and populates a `BTreeMap` with language entries. It also sets up a new session using
/// a UUID stored in local storage, and provides a default state for various application
/// components including client, auth, breadcrumbs, dialog, roles, groups, menu, pages,
/// personnel, and platform.
///
/// # Parameters
/// - `i18n`: A string slice containing i18n data in CSV format.
///
/// # Returns
/// - A `Signal` containing the session string.
pub fn use_init_state(i18n: &str) -> Signal<Cow<'static, str>> {
    let mut lang_entries =
        BTreeMap::<Cow<'static, str>, Cow<'static, str>>::new();

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'=')
        .has_headers(false)
        .flexible(true)
        .trim(csv::Trim::All)
        .from_reader(i18n.as_bytes());

    for record in reader.deserialize::<I18nEntry>().flatten() {
        lang_entries.insert(record.key, record.value);
    }

    let uuid = uuid::Uuid::new_v4().to_string();
    let storage_session =
        use_local_storage("mtc_key1", || json!(uuid));
    let session = storage_session.get().self_str().unwrap_or(uuid.into()).to_string();

    use_context_provider(|| UseState {
        session: Signal::new(session.into()),
        client: Default::default(),
        i18n: Signal::new(lang_entries),
        auth: Default::default(),
        breadcrumbs: Default::default(),
        dialog: Default::default(),
        roles: Default::default(),
        groups: Default::default(),
        menu: Default::default(),
        pages: Default::default(),
        personnel: Default::default(),
        personnel_columns: Default::default(),
        search: Default::default(),
        platform: Signal::new(Cow::Borrowed("web")),
    }).session
}

/// Returns the global application state context.
///
/// This hook returns a `UseState` object, which contains all the
/// application state that can be accessed by the application.
pub fn use_state() -> UseState {
    consume_context::<UseState>()
}

/// WASM application state
#[derive(Clone, Copy)]
pub struct UseState {
    session: Signal<Cow<'static, str>>,
    client: Signal<Client>,
    i18n: Signal<BTreeMap<Cow<'static, str>, Cow<'static, str>>>,
    auth: Signal<AuthState>,
    breadcrumbs: Signal<Vec<(Cow<'static, str>, Cow<'static, str>)>>,
    dialog: Signal<Option<DialogBoxArgs>>,
    roles: Signal<Vec<Entry>>,
    groups: Signal<Vec<Entry>>,
    menu: Signal<bool>,
    pages: Signal<Vec<Entry>>,
    personnel: Signal<BTreeMap<Cow<'static, str>, UserDetails>>,
    personnel_columns: PersonnelColumns,
    search: SearchEngine,
    platform: Signal<Cow<'static,str>>,
}

impl UseState {
    pub fn client(&self) -> Signal<Client> {
        self.client
    }

    pub fn menu(&self) -> Signal<bool> {
        self.menu
    }
    pub fn set_menu(&self, state: bool) {
        *self.menu.write_unchecked() = state
    }

    pub fn auth(&self) -> Signal<AuthState> {
        self.auth
    }
    pub fn set_auth(&mut self, auth: AuthState) {
        *self.auth.write_unchecked() = auth
    }

    pub fn breadcrumbs(&self) -> Signal<Vec<(Cow<'static, str>, Cow<'static, str>)>> {
        self.breadcrumbs
    }
    pub fn set_breadcrumbs(&self, breadcrumbs: Vec<(Cow<'static, str>, Cow<'static, str>)>) {
        *self.breadcrumbs.write_unchecked() = breadcrumbs
    }

    pub fn i18n(&self) -> Signal<BTreeMap<Cow<'static, str>, Cow<'static, str>>> {
        self.i18n
    }

    pub fn dialog(&self) -> Signal<Option<DialogBoxArgs>> {
        self.dialog
    }
    pub fn set_dialog(&self, dialog: Option<DialogBoxArgs>) {
        *self.dialog.write_unchecked() = dialog
    }

    pub fn roles(&self) -> Signal<Vec<Entry>> {
        self.roles
    }
    pub fn set_roles(&mut self, roles: Vec<Entry>) {
        *self.roles.write_unchecked() = roles
    }

    pub fn groups(&self) -> Signal<Vec<Entry>> {
        self.groups
    }
    pub fn set_groups(&mut self, groups: Vec<Entry>) {
        *self.groups.write_unchecked() = groups
    }

    pub fn pages(&self) -> Signal<Vec<Entry>> {
        self.pages
    }
    pub fn set_pages(&self, pages: Vec<Entry>) {
        *self.pages.write_unchecked() = pages
    }

    pub fn personnel(&self) -> Signal<BTreeMap<Cow<'static, str>, UserDetails>> {
        self.personnel
    }
    pub fn set_personnel(&self, personnel: BTreeMap<Cow<'static, str>, UserDetails>) {
        *self.personnel.write_unchecked() = personnel
    }

    pub fn add_personnel(&self, user_details_dto: Vec<UserDetailsDto>) {
        let mut users = self.personnel;
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

    pub fn personnel_columns(&self) -> PersonnelColumns {
        self.personnel_columns
    }

    pub fn search_engine(&self) -> SearchEngine {
        self.search
    }
    pub fn search_engine_clear(&self) {
        let mut pattern = self.search.pattern;
        if !pattern().is_empty() {
            pattern.set("".into())
        }
    }

    pub fn platform(&self) -> Signal<Cow<'static, str>> {
        self.platform
    }
    pub fn set_platform(&self, platform: Cow<'static, str>) {
        *self.platform.write_unchecked() = platform
    }
}
