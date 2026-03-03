use std::{borrow::Borrow, fmt::Display};

// TODO: use Option<&T> instead
pub fn display_option<T: Display>(option: &Option<T>) -> String {
    match option {
        Some(value) => value.to_string(),
        None => "".to_string(),
    }
}

#[allow(dead_code)]
pub fn is_true(b: impl Borrow<bool>) -> bool {
    *b.borrow()
}

pub fn is_false(b: impl Borrow<bool>) -> bool {
    !b.borrow()
}
