use std::{fmt::Display, hash::Hash, marker::PhantomData};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id<T> {
    id: String,
    #[serde(skip)]
    _type: PhantomData<T>,
}

impl<T> std::fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.id)
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            _type: PhantomData,
        }
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T> Eq for Id<T> {}

impl<T> Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> Id<T> {
    pub fn random() -> Self {
        Id {
            // Note that not all IDs are necessarily UUIDs !
            // For example, IDs of users are either user defined strings
            // during development or the IDs received from OAuth providers
            // in production.
            id: uuid::Uuid::new_v4().to_string(),
            _type: PhantomData,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.id.is_empty()
    }
}

impl<T> From<String> for Id<T> {
    fn from(value: String) -> Self {
        Id {
            id: value,
            _type: PhantomData,
        }
    }
}

impl<T> From<&str> for Id<T> {
    fn from(value: &str) -> Self {
        Id::from(String::from(value))
    }
}

impl<T> From<Id<T>> for String {
    fn from(value: Id<T>) -> Self {
        value.id
    }
}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
