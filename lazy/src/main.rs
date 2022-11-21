mod lazy_regexps;

use regex::Regex;
use crate::lazy_regexps::MY_REGEX;

fn main() {
    use_regex(MY_REGEX, "hello world!"); // Error is being solved by the clone()
}

fn use_regex(regex_expression: Regex, text: &str) {
    let result = regex_expression.captures_iter(text).collect::<Vec<regex::Captures<>>>();
    println!("{:#?}", result.get(0).unwrap().get(1).unwrap());
}