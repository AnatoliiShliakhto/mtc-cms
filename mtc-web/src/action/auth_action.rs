#[allow(dead_code)]
pub enum AuthAction {
    SignIn(String, String),
    Credentials,
    SignOut,
}