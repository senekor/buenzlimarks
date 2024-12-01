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

/// Contains the runtime configruation of the server,
/// including which port to run on and the database location.
pub mod config;

/// Contains the shared state of the server.
pub mod state;

/// This module contains the routes to the graphical user interface.
pub mod app;

/// This module contains the routes to the documentation.
pub mod docs;
