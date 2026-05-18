//! A collection of functions for various database queries.
//!
//! All functions in this module are very similar in structure. The take a
//! [sqlx::Transaction] and optional arguments, e.g., the ID to select by,
//! perform the query and return the result or and error. For complex
//! arguments or return values, they often outsource types to [avina_wire].
//!
//! The type of query is usually clear from the name, e.g.,
//! [user::user::select_user_from_db], which selects a user with the given ID.
//!
//! # `select_maybe_` functions
//!
//! Among select functions, a common pattern in this module is to find a pair of
//! `select_maybe_[some_resource]` and `select_[some_resource]` functions.
//! The `maybe` variant returns the requested data or returns [None] if nothing was found,
//! while the other function variant uses this, and turns the [None] into a
//! "Not Found" error. As an example, see [user::user::select_maybe_user_from_db]
//! and [user::user::select_user_from_db].

pub mod accounting;
pub mod budgeting;
pub mod pricing;
pub mod quota;
pub mod resources;
pub mod user;
