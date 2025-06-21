use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use std::fmt::Display;

const NON_ALPHANUMERIC_EXCEPT_DOT: &AsciiSet = &NON_ALPHANUMERIC.remove(b'.');

pub(super) fn encode_array(array: Vec<String>) -> String {
    utf8_percent_encode(&array.join(","), NON_ALPHANUMERIC_EXCEPT_DOT).to_string()
}

pub(super) fn encode(name: String) -> String {
    utf8_percent_encode(&name, NON_ALPHANUMERIC).to_string()
}

pub(super) fn param<T>(name: &str) -> impl Fn(T) -> String + '_
where
    T: Display,
{
    move |v| format!("{name}={v}")
}
