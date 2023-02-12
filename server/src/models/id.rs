use std::{fmt::Display, marker::PhantomData};

use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id<T> {
    id: String,
    #[serde(skip)]
    _type: PhantomData<T>,
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
            id: "dev_user".into(),
            _type: PhantomData,
        }
    }
}

impl<T, U: Into<String>> From<U> for Id<T> {
    fn from(value: U) -> Self {
        Id {
            id: value.into(),
            _type: PhantomData,
        }
    }
}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
