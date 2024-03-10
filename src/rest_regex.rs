use regex::Regex;
use once_cell::sync::Lazy;

pub static ACCOUNT_ID_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Za-z0-9\-]{1,30}$").expect("Invalid regex pattern ACCOUNT_ID_REGEX!")
});

pub static CURRENCY_CODE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Z]{3}$").expect("Invalid regex pattern CURRENCY_CODE_REGEX!")
});

pub static DEAL_ID_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Za-z0-9\-]{1,30}$").expect("Invalid regex pattern DEAL_ID_REGEX!")
});

pub static DEAL_REFERENCE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Za-z0-9_\-]{1,30}$").expect("Invalid regex pattern DEAL_REFERENCE_REGEX!")
});

pub static EPIC_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Za-z0-9._]{6,30}$").expect("Invalid regex pattern EPIC_REGEX!")
});

pub static EXPIRY_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(^\d{2}-)?[A-Z]{3}-\d{2}$|-|DFB").expect("Invalid regex pattern EXPIRY_REGEX!")
});

pub static IDENTIFIER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Za-z0-9\-_]{1,30}$").expect("Invalid regex pattern IDENTIFIER_REGEX!")
});

pub static PASSWORD_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^.{1,350}$").expect("Invalid regex pattern PASSWORD_REGEX!")
});