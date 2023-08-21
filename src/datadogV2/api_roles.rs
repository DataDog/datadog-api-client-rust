// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// AddPermissionToRoleParams is a struct for passing parameters to the method [`AddPermissionToRole`]
#[derive(Clone, Debug)]
pub struct AddPermissionToRoleParams {
    /* The unique identifier of the role. */
    pub role_id: String,
    pub body: RelationshipToPermission,
}

// AddUserToRoleParams is a struct for passing parameters to the method [`AddUserToRole`]
#[derive(Clone, Debug)]
pub struct AddUserToRoleParams {
    /* The unique identifier of the role. */
    pub role_id: String,
    pub body: RelationshipToUser,
}

// CloneRoleParams is a struct for passing parameters to the method [`CloneRole`]
#[derive(Clone, Debug)]
pub struct CloneRoleParams {
    /* The unique identifier of the role. */
    pub role_id: String,
    pub body: RoleCloneRequest,
}

// CreateRoleParams is a struct for passing parameters to the method [`CreateRole`]
#[derive(Clone, Debug)]
pub struct CreateRoleParams {
    pub body: RoleCreateRequest,
}

// DeleteRoleParams is a struct for passing parameters to the method [`DeleteRole`]
#[derive(Clone, Debug)]
pub struct DeleteRoleParams {
    /* The unique identifier of the role. */
    pub role_id: String,
}

// GetRoleParams is a struct for passing parameters to the method [`GetRole`]
#[derive(Clone, Debug)]
pub struct GetRoleParams {
    /* The unique identifier of the role. */
    pub role_id: String,
}

// ListRolePermissionsParams is a struct for passing parameters to the method [`ListRolePermissions`]
#[derive(Clone, Debug)]
pub struct ListRolePermissionsParams {
    /* The unique identifier of the role. */
    pub role_id: String,
}

// ListRoleUsersParams is a struct for passing parameters to the method [`ListRoleUsers`]
#[derive(Clone, Debug)]
pub struct ListRoleUsersParams {
    /* The unique identifier of the role. */
    pub role_id: String,
    /* Size for a given page. The maximum allowed value is 100. */
    pub page_size: i64,
    /* Specific page number to return. */
    pub page_number: i64,
    /* User attribute to order results by. Sort order is **ascending** by default.
Sort order is **descending** if the field is prefixed by a negative sign,
for example `sort=-name`. Options: `name`, `email`, `status`. */
    pub sort: String,
    /* Filter all users by the given string. Defaults to no filtering. */
    pub filter: String,
}

// ListRolesParams is a struct for passing parameters to the method [`ListRoles`]
#[derive(Clone, Debug)]
pub struct ListRolesParams {
    /* Size for a given page. The maximum allowed value is 100. */
    pub page_size: i64,
    /* Specific page number to return. */
    pub page_number: i64,
    /* Sort roles depending on the given field. Sort order is **ascending** by default.
Sort order is **descending** if the field is prefixed by a negative sign, for example:
`sort=-name`. */
    pub sort: RolesSort,
    /* Filter all roles by the given string. */
    pub filter: String,
}

// RemovePermissionFromRoleParams is a struct for passing parameters to the method [`RemovePermissionFromRole`]
#[derive(Clone, Debug)]
pub struct RemovePermissionFromRoleParams {
    /* The unique identifier of the role. */
    pub role_id: String,
    pub body: RelationshipToPermission,
}

// RemoveUserFromRoleParams is a struct for passing parameters to the method [`RemoveUserFromRole`]
#[derive(Clone, Debug)]
pub struct RemoveUserFromRoleParams {
    /* The unique identifier of the role. */
    pub role_id: String,
    pub body: RelationshipToUser,
}

// UpdateRoleParams is a struct for passing parameters to the method [`UpdateRole`]
#[derive(Clone, Debug)]
pub struct UpdateRoleParams {
    /* The unique identifier of the role. */
    pub role_id: String,
    pub body: RoleUpdateRequest,
}




/// AddPermissionToRoleError is a struct for typed errors of method [`AddPermissionToRole`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddPermissionToRoleError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// AddUserToRoleError is a struct for typed errors of method [`AddUserToRole`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddUserToRoleError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CloneRoleError is a struct for typed errors of method [`CloneRole`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloneRoleError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateRoleError is a struct for typed errors of method [`CreateRole`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateRoleError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteRoleError is a struct for typed errors of method [`DeleteRole`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRoleError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetRoleError is a struct for typed errors of method [`GetRole`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRoleError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListPermissionsError is a struct for typed errors of method [`ListPermissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPermissionsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListRolePermissionsError is a struct for typed errors of method [`ListRolePermissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRolePermissionsError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListRoleUsersError is a struct for typed errors of method [`ListRoleUsers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRoleUsersError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListRolesError is a struct for typed errors of method [`ListRoles`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRolesError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// RemovePermissionFromRoleError is a struct for typed errors of method [`RemovePermissionFromRole`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemovePermissionFromRoleError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// RemoveUserFromRoleError is a struct for typed errors of method [`RemoveUserFromRole`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveUserFromRoleError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateRoleError is a struct for typed errors of method [`UpdateRole`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateRoleError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status422(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}