//! This error handling is terrible. It should be improved!
//!
//! See the [corresponding issue](https://github.com/senekor/buenzlimarks/issues/30)
//! on GitHub.

use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub enum DbError {
    NotFound,
    AlreadyExists,
    WhoopsieDoopsie,
}
pub type DbResult<T = ()> = Result<T, DbError>;

pub trait Whoopsie<T> {
    fn whoopsie(self) -> DbResult<T>;
}
impl<T: Debug, E: Debug> Whoopsie<T> for Result<T, E> {
    #[track_caller]
    fn whoopsie(self) -> DbResult<T> {
        self.map_err(|e| {
            tracing::error!("{e:?}");
            DbError::WhoopsieDoopsie
        })
    }
}
