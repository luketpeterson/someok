
#![doc = include_str!("../README.md")]

/// Wraps [Option::is_some] and [Result::is_ok] under one interface
pub trait IsSuccess {
    fn is_success(&self) -> bool;
}

impl<T> IsSuccess for Option<T> {
    fn is_success(&self) -> bool {
        self.is_some()
    }
}

impl<T, E> IsSuccess for Result<T, E> {
    fn is_success(&self) -> bool {
        self.is_ok()
    }
}

