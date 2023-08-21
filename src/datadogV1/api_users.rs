// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateUserParams is a struct for passing parameters to the method [`CreateUser`]
#[derive(Clone, Debug)]
pub struct CreateUserParams {
    /* User object that needs to be created. */
    pub body: User,
}

// DisableUserParams is a struct for passing parameters to the method [`DisableUser`]
#[derive(Clone, Debug)]
pub struct DisableUserParams {
    /* The handle of the user. */
    pub user_handle: String,
}

// GetUserParams is a struct for passing parameters to the method [`GetUser`]
#[derive(Clone, Debug)]
pub struct GetUserParams {
    /* The ID of the user. */
    pub user_handle: String,
}

// UpdateUserParams is a struct for passing parameters to the method [`UpdateUser`]
#[derive(Clone, Debug)]
pub struct UpdateUserParams {
    /* The ID of the user. */
    pub user_handle: String,
    /* Description of the update. */
    pub body: User,
}




/// CreateUserError is a struct for typed errors of method [`CreateUser`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateUserError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DisableUserError is a struct for typed errors of method [`DisableUser`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DisableUserError {
    Status400(APIErrorResponse),
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

/// ListUsersError is a struct for typed errors of method [`ListUsers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListUsersError {
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
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}