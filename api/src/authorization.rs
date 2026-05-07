//! Helper functions for checking user authorization.

use avina_wire::user::User;

use crate::error::{AuthOnlyError, NotFoundOnlyError};

/// Return an authorization error if the given user is not an admin.
pub fn require_admin_user(user: &User) -> Result<(), AuthOnlyError> {
    if !user.is_staff {
        return Err(AuthOnlyError::AuthorizationError(
            "Admin privileges required".to_string(),
        ));
    }
    Ok(())
}

/// Return an not-found error if the given user is not an admin.
///
/// This is supposed to be used instead of [require_admin_user] on endpoints that might return
/// not-found, to ensure that non-admin users can't even see something exists.
pub fn require_admin_user_or_return_not_found(
    user: &User,
) -> Result<(), NotFoundOnlyError> {
    if !user.is_staff {
        return Err(NotFoundOnlyError::NotFoundError);
    }
    Ok(())
}

/// Return an authorization error if the given user is not a master user of the given project.
pub fn require_master_user(
    user: &User,
    project_id: u32,
) -> Result<(), AuthOnlyError> {
    if !user.is_staff && (user.role != 2 || user.project != project_id) {
        return Err(AuthOnlyError::AuthorizationError(
            "Admin or master user privileges for respective project required"
                .to_string(),
        ));
    }
    Ok(())
}

/// Return an not-found error if the given user is not a master user of the given project
///
/// This is supposed to be used instead of [require_master_user] on endpoints that might return
/// not-found, to ensure that non-master users can't even see something exists.
pub fn require_master_user_or_return_not_found(
    user: &User,
    project_id: u32,
) -> Result<(), NotFoundOnlyError> {
    if !user.is_staff && (user.role != 2 || user.project != project_id) {
        return Err(NotFoundOnlyError::NotFoundError);
    }
    Ok(())
}

/// Return an authorization error if the given user is not a user of the given project.
pub fn require_project_user(
    user: &User,
    project_id: u32,
) -> Result<(), AuthOnlyError> {
    if !user.is_staff && user.project != project_id {
        return Err(AuthOnlyError::AuthorizationError(
            "Must be admin or user of respective project".to_string(),
        ));
    }
    Ok(())
}

/// Return an not-found error if the given user is not a user of the given project
///
/// This is supposed to be used instead of [require_project_user] on endpoints that might return
/// not-found, to ensure that non-project users can't even see something exists.
pub fn require_project_user_or_return_not_found(
    user: &User,
    project_id: u32,
) -> Result<(), NotFoundOnlyError> {
    if !user.is_staff && user.project != project_id {
        return Err(NotFoundOnlyError::NotFoundError);
    }
    Ok(())
}

/// Return an not-found error if the given user is neither admin, nor master user of the given
/// project, nor the user with the given ID itself.
///
/// This is useful for the complex but common scenario to return information only to the owning
/// user, their master user, or admins.
pub fn require_user_or_project_master_or_not_found(
    user: &User,
    user_id: u32,
    project_id: u32,
) -> Result<(), NotFoundOnlyError> {
    #[allow(clippy::nonminimal_bool)]
    if !user.is_staff
        && !(user.role == 1 && user.id == user_id)
        && !(user.role == 2 && user.project == project_id)
    {
        return Err(NotFoundOnlyError::NotFoundError);
    }
    Ok(())
}
