// mod app_web_authenticator;
mod authenticator;
mod user_cli_authenticator;

// pub use self::app_web_authenticator::{AppWebAuthenticator};
pub use self::authenticator::Authenticator;
pub use self::user_cli_authenticator::UserCliAuthenticator;
