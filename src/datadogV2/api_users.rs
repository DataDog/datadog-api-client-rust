// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateUserParams is a struct for passing parameters to the method [`CreateUser`]
#[derive(Clone, Debug)]
pub struct CreateUserParams {
    pub body: UserCreateRequest,
}

// DisableUserParams is a struct for passing parameters to the method [`DisableUser`]
#[derive(Clone, Debug)]
pub struct DisableUserParams {
    /* The ID of the user. */
    pub user_id: String,
}

// GetInvitationParams is a struct for passing parameters to the method [`GetInvitation`]
#[derive(Clone, Debug)]
pub struct GetInvitationParams {
    /* The UUID of the user invitation. */
    pub user_invitation_uuid: String,
}

// GetUserParams is a struct for passing parameters to the method [`GetUser`]
#[derive(Clone, Debug)]
pub struct GetUserParams {
    /* The ID of the user. */
    pub user_id: String,
}

// ListUserOrganizationsParams is a struct for passing parameters to the method [`ListUserOrganizations`]
#[derive(Clone, Debug)]
pub struct ListUserOrganizationsParams {
    /* The ID of the user. */
    pub user_id: String,
}

// ListUserPermissionsParams is a struct for passing parameters to the method [`ListUserPermissions`]
#[derive(Clone, Debug)]
pub struct ListUserPermissionsParams {
    /* The ID of the user. */
    pub user_id: String,
}

// ListUsersParams is a struct for passing parameters to the method [`ListUsers`]
#[derive(Clone, Debug)]
pub struct ListUsersParams {
    /* Size for a given page. The maximum allowed value is 100. */
    pub page_size: i64,
    /* Specific page number to return. */
    pub page_number: i64,
    /* User attribute to order results by. Sort order is ascending by default.
Sort order is descending if the field
is prefixed by a negative sign, for example `sort=-name`. Options: `name`,
`modified_at`, `user_count`. */
    pub sort: String,
    /* Direction of sort. Options: `asc`, `desc`. */
    pub sort_dir: QuerySortOrder,
    /* Filter all users by the given string. Defaults to no filtering. */
    pub filter: String,
    /* Filter on status attribute.
Comma separated list, with possible values `Active`, `Pending`, and `Disabled`.
Defaults to no filtering. */
    pub filter_status: String,
}

// SendInvitationsParams is a struct for passing parameters to the method [`SendInvitations`]
#[derive(Clone, Debug)]
pub struct SendInvitationsParams {
    pub body: UserInvitationsRequest,
}

// UpdateUserParams is a struct for passing parameters to the method [`UpdateUser`]
#[derive(Clone, Debug)]
pub struct UpdateUserParams {
    /* The ID of the user. */
    pub user_id: String,
    pub body: UserUpdateRequest,
}




/// CreateUserError is a struct for typed errors of method [`CreateUser`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateUserError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DisableUserError is a struct for typed errors of method [`DisableUser`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DisableUserError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetInvitationError is a struct for typed errors of method [`GetInvitation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInvitationError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUserError is a struct for typed errors of method [`GetUser`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListUserOrganizationsError is a struct for typed errors of method [`ListUserOrganizations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListUserOrganizationsError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListUserPermissionsError is a struct for typed errors of method [`ListUserPermissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListUserPermissionsError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListUsersError is a struct for typed errors of method [`ListUsers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListUsersError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SendInvitationsError is a struct for typed errors of method [`SendInvitations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendInvitationsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateUserError is a struct for typed errors of method [`UpdateUser`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateUserError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status422(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}