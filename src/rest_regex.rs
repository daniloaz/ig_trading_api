use regex::Regex;
use once_cell::sync::Lazy;

pub static ACCOUNT_ID_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Za-z0-9\-]{1,30}$").expect("Invalid regex pattern!")
});

pub static IDENTIFIER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Za-z0-9\-_]{1,30}$").expect("Invalid regex pattern for username!")
});

pub static PASSWORD_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^.{1,350}$").expect("Invalid regex pattern for password!")
});