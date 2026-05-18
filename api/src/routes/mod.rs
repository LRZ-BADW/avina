//! Routes and endpoints of the API.
//!
//! The submodules contain `*_scopes` that link up routes to functions
//! for their endpoints. Usually these are situated in subsubmodules
//! which contain only this endpoint, e.g., `user_get` in `user::user::get`,
//! sometimes along with helper functions.
//!
//! For resources like a `User` or `FlavorPrice`, there are usually at least
//! the typical CRUD endpoints:
//!
//!   * `POST ./`: create a new instance based of the request data.
//!   * `GET .`: get a list of instances of this resource. Filters are passed via URL encoding.
//!   * `GET ./{id}`: get a single instance of this resource by its ID.
//!   * `PATCH ./{id}/`: modify the instance of the resource with the given ID according to the
//!     request data.
//!   * `DELETE ./{id}/`: delete the instance of the resource with the given ID.
//!
//! Additional endpoints not following this convention may exist on further routes.

mod accounting;
mod budgeting;
mod health_check;
mod hello;
mod pricing;
pub mod quota;
mod resources;
pub mod user;

pub use accounting::*;
pub use budgeting::*;
pub use health_check::*;
pub use hello::*;
pub use pricing::*;
pub use quota::*;
pub use resources::*;
pub use user::*;
