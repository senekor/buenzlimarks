/// This module provides the persistence interface and implementation.
/// The current implementation is file system based, this may be replaced
/// by an SQL database in the future.
pub mod db;

/// This module contains the http handlers of the server.
pub mod handlers;

/// This module contains the router. It is responsible for routing
/// incoming requests from users to the appropriate handlers,
/// based on the [path] and [method] of the request.
///
/// [path]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Identifying_resources_on_the_Web#path
/// [method]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
pub mod router;

/// This module contains the domain models of buenzlimarks.
/// That includes the data models and business logic.
pub mod models;

/// This module contains the routes to the graphical user interface.
pub mod frontend;
