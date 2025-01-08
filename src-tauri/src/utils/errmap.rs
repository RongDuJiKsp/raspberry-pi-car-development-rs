use std::fmt::Display;

pub struct ErrMap;

impl ErrMap {
    pub fn string<T: Display>(e: T) -> String {
        e.to_string()
    }
}
