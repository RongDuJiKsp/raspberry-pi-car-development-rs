use std::{
    error::Error,
    sync::{Arc, Mutex},
};

pub type EResult<T> = Result<T, Box<dyn Error>>;
pub type SharePtr<T> = Arc<Mutex<T>>;
pub struct Ptr;
impl Ptr {
    pub fn shared<T>(x: T) -> SharePtr<T> {
        Arc::new(Mutex::new(x))
    }
}
