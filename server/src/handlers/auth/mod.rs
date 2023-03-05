//! This module is still under construction. Its behavior changes significantly
//! between debug and release builds (during development and in production.)
//!
//! ## During development (debug build)
//!
//! Without doing anything, incoming requests will be authenticated as the
//! default development user. In order to test behavior for multible users,
//! a cookie "buenzlimarks-auth" may be provided with the ID of a different
//! user as its value. The other user must be initialized first in the
//! database by making a request to `/api/auth/login/:user_id`, which will
//! also set the appropriate cookie.
//!
//! ## In production (release build)
//!
//! TODO oauth2 based authentication for production.

static COOKIE_NAME: &str = "buenzlimarks-auth";

#[cfg(debug_assertions)]
mod dev;
#[cfg(debug_assertions)]
pub use dev::*;

#[cfg(not(debug_assertions))]
mod prod;
#[cfg(not(debug_assertions))]
pub use prod::routes;
