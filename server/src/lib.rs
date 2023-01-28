/// This module provides the persistence interface and implementation.
/// The current implementation is file system based, this may be replaced
/// by an SQL database in the future.
pub mod db;

/// This module contains the http handlers of the server.
pub mod handlers;

/// This module contains the domain models of buenzlimarks.
/// That includes the data models and business logic.
pub mod models;
