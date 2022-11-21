use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref MY_REGEX: Regex = Regex::new(r"hello (\w+)!").unwrap();
}
