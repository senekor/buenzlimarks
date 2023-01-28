//! This error handling is terrible. It should be improved!
//! 
//! See the [corresponding issue](https://github.com/remlse/buenzlimarks/issues/30)
//! on GitHub.

use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum DbError {
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
            println!("{e:?}");
            DbError::WhoopsieDoopsie
        })
    }
}
