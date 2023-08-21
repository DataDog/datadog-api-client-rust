// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateAPIKeyParams is a struct for passing parameters to the method [`CreateAPIKey`]
#[derive(Clone, Debug)]
pub struct CreateAPIKeyParams {
    pub body: ApiKey,
}

// CreateApplicationKeyParams is a struct for passing parameters to the method [`CreateApplicationKey`]
#[derive(Clone, Debug)]
pub struct CreateApplicationKeyParams {
    pub body: ApplicationKey,
}

// DeleteAPIKeyParams is a struct for passing parameters to the method [`DeleteAPIKey`]
#[derive(Clone, Debug)]
pub struct DeleteAPIKeyParams {
    /* The specific API key you are working with. */
    pub key: String,
}

// DeleteApplicationKeyParams is a struct for passing parameters to the method [`DeleteApplicationKey`]
#[derive(Clone, Debug)]
pub struct DeleteApplicationKeyParams {
    /* The specific APP key you are working with. */
    pub key: String,
}

// GetAPIKeyParams is a struct for passing parameters to the method [`GetAPIKey`]
#[derive(Clone, Debug)]
pub struct GetAPIKeyParams {
    /* The specific API key you are working with. */
    pub key: String,
}

// GetApplicationKeyParams is a struct for passing parameters to the method [`GetApplicationKey`]
#[derive(Clone, Debug)]
pub struct GetApplicationKeyParams {
    /* The specific APP key you are working with. */
    pub key: String,
}

// UpdateAPIKeyParams is a struct for passing parameters to the method [`UpdateAPIKey`]
#[derive(Clone, Debug)]
pub struct UpdateAPIKeyParams {
    /* The specific API key you are working with. */
    pub key: String,
    pub body: ApiKey,
}

// UpdateApplicationKeyParams is a struct for passing parameters to the method [`UpdateApplicationKey`]
#[derive(Clone, Debug)]
pub struct UpdateApplicationKeyParams {
    /* The specific APP key you are working with. */
    pub key: String,
    pub body: ApplicationKey,
}




/// CreateAPIKeyError is a struct for typed errors of method [`CreateAPIKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAPIKeyError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateApplicationKeyError is a struct for typed errors of method [`CreateApplicationKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateApplicationKeyError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteAPIKeyError is a struct for typed errors of method [`DeleteAPIKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAPIKeyError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteApplicationKeyError is a struct for typed errors of method [`DeleteApplicationKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteApplicationKeyError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetAPIKeyError is a struct for typed errors of method [`GetAPIKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAPIKeyError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetApplicationKeyError is a struct for typed errors of method [`GetApplicationKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApplicationKeyError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAPIKeysError is a struct for typed errors of method [`ListAPIKeys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAPIKeysError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListApplicationKeysError is a struct for typed errors of method [`ListApplicationKeys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListApplicationKeysError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateAPIKeyError is a struct for typed errors of method [`UpdateAPIKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAPIKeyError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateApplicationKeyError is a struct for typed errors of method [`UpdateApplicationKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApplicationKeyError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}