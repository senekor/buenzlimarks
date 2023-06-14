use std::{fmt::Display, marker::PhantomData};

use serde::{Deserialize, Serialize};

use super::user::User;

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

impl<T> Id<T> {
    pub fn random() -> Self {
        Id {
            id: uuid::Uuid::new_v4().to_string(),
            _type: PhantomData,
        }
    }
}

impl Id<User> {
    pub fn dev_user_id() -> Self {
        Self {
            id: super::user::DEV_USER_ID_STR.into(),
            _type: PhantomData,
        }
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
