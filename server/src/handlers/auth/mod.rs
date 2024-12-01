//! This module's behavior changes significantly between debug and release
//! builds (during development and in production.)
//!
//! ## During development (debug build)
//!
//! Incoming requests are expected to use bearer authentication, where the
//! bearer token is simply the user ID one wants to identify themselves as. The
//! development seed data has a user with ID "buenzli". This ID is also used in
//! the default collection of the thunderclient requests as well as the
//! frontend.
//!
//! The goal of this approach is to be as close to production auth as possible
//! (requests are required to use authentication) while preserving developer
//! convenience (clients authenticate themselves with a default user).
//!
//! Users other than the default one may be used after they login for the first
//! time by sending a GET request to `/api/auth/login/:user_id`, which
//! initializes the user's database entry.
//!
//! ## In production (release build)
//!
//! The production authentication is based on OAuth2. A successful login will
//! supply the user with a [JWT](jwt.io), containing the necessary information to
//! identify the user, which includes their ID as well as the OAuth provider.
//! (Currently, GitHub is the only supported provider.) Like during development,
//! all requests require bearer authentication, where the bearer token is the
//! JWT received from a successful login.
//!
//! The login flow is straight-forward OAuth2. Two requests need to be made
//! to the server.
//! 1. `/api/auth/<provider>/login`
//!    The response is a redirect to the provider's OAuth endpoint. Where the
//!    user my grant the server access to the requested resources.
//! 2. `/api/auth/<provider>/callback`
//!    The request must contain `code` and `state` in the query params, as
//!    received from the provider after the first request. If everything goes
//!    to plan, the response will have the JWT in its body.

mod user_extractor;

#[cfg(debug_assertions)]
mod devel;
#[cfg(debug_assertions)]
pub use devel::*;

#[cfg(not(debug_assertions))]
mod prod;
#[cfg(not(debug_assertions))]
pub use prod::*;
