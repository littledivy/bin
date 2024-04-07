pub mod index;
pub mod pretty_retrieve;
pub mod retrieve;
pub mod static_files;
pub mod submit;
pub mod upload;

use rocket_basicauth::BasicAuth;

static USERNAME: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
    std::env::var("BIN_USERNAME").expect("USERNAME must be set")
});

static PASSWORD: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
    std::env::var("BIN_PASSWORD").expect("PASSWORD must be set")
});

pub fn authenicate(auth: BasicAuth) -> bool {
    auth.username == *USERNAME && auth.password == *PASSWORD
}
