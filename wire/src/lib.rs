//! Collection of types used on both sides of the wire during avina API communication.
//!
//! This crate therefore mostly contains structs and derives the usual traits:
//! [Clone], [Debug], [PartialEq], [serde::Deserialize], [serde::Serialize].
//! Depending on set features it may also derive or implement additional traits.
//!
//! - The `sqlx` feature derives the [sqlx::FromRow] trait, which is used in the
//!   [avina-api](https://docs.rs/avina-api) crate for reading and writing these types from and to the
//!   database.
//! - The `tabled` feature derives the [tabled::Tabled] trait, which is used in the
//!   [avina-cli](https://docs.rs/avina-cli) crate for displaying these types.

pub mod common;
pub mod error;

#[cfg(feature = "accounting")]
pub mod accounting;
#[cfg(feature = "budgeting")]
pub mod budgeting;
#[cfg(feature = "hello")]
pub mod hello;
#[cfg(feature = "pricing")]
pub mod pricing;
#[cfg(feature = "quota")]
pub mod quota;
#[cfg(feature = "resources")]
pub mod resources;
#[cfg(feature = "user")]
pub mod user;
