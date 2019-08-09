pub mod list_files;

pub trait AuthenticationDelegate {
    // Should return a future instead
    // This method is in charge of returning an authentication token
    fn get_authorization_token(&self) -> String;
}
