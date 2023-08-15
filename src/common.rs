use serde::{ser::Serializer, Serialize};
use std::{collections::HashMap, sync::Mutex};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Default)]
pub struct MyState(pub Mutex<HashMap<String, String>>);

pub type Result<T> = std::result::Result<T, Error>;
