// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateAPIKeyParams is a struct for passing parameters to the method [`CreateAPIKey`]
#[derive(Clone, Debug)]
pub struct CreateAPIKeyParams {
    pub body: APIKeyCreateRequest,
}

// CreateCurrentUserApplicationKeyParams is a struct for passing parameters to the method [`CreateCurrentUserApplicationKey`]
#[derive(Clone, Debug)]
pub struct CreateCurrentUserApplicationKeyParams {
    pub body: ApplicationKeyCreateRequest,
}

// DeleteAPIKeyParams is a struct for passing parameters to the method [`DeleteAPIKey`]
#[derive(Clone, Debug)]
pub struct DeleteAPIKeyParams {
    /* The ID of the API key. */
    pub api_key_id: String,
}

// DeleteApplicationKeyParams is a struct for passing parameters to the method [`DeleteApplicationKey`]
#[derive(Clone, Debug)]
pub struct DeleteApplicationKeyParams {
    /* The ID of the application key. */
    pub app_key_id: String,
}

// DeleteCurrentUserApplicationKeyParams is a struct for passing parameters to the method [`DeleteCurrentUserApplicationKey`]
#[derive(Clone, Debug)]
pub struct DeleteCurrentUserApplicationKeyParams {
    /* The ID of the application key. */
    pub app_key_id: String,
}

// GetAPIKeyParams is a struct for passing parameters to the method [`GetAPIKey`]
#[derive(Clone, Debug)]
pub struct GetAPIKeyParams {
    /* The ID of the API key. */
    pub api_key_id: String,
    /* Comma separated list of resource paths for related resources to include in the response. Supported resource paths are `created_by` and `modified_by`. */
    pub include: String,
}

// GetApplicationKeyParams is a struct for passing parameters to the method [`GetApplicationKey`]
#[derive(Clone, Debug)]
pub struct GetApplicationKeyParams {
    /* The ID of the application key. */
    pub app_key_id: String,
    /* Resource path for related resources to include in the response. Only `owned_by` is supported. */
    pub include: String,
}

// GetCurrentUserApplicationKeyParams is a struct for passing parameters to the method [`GetCurrentUserApplicationKey`]
#[derive(Clone, Debug)]
pub struct GetCurrentUserApplicationKeyParams {
    /* The ID of the application key. */
    pub app_key_id: String,
}

// ListAPIKeysParams is a struct for passing parameters to the method [`ListAPIKeys`]
#[derive(Clone, Debug)]
pub struct ListAPIKeysParams {
    /* Size for a given page. The maximum allowed value is 100. */
    pub page_size: i64,
    /* Specific page number to return. */
    pub page_number: i64,
    /* API key attribute used to sort results. Sort order is ascending
by default. In order to specify a descending sort, prefix the
attribute with a minus sign. */
    pub sort: APIKeysSort,
    /* Filter API keys by the specified string. */
    pub filter: String,
    /* Only include API keys created on or after the specified date. */
    pub filter_created_at_start: String,
    /* Only include API keys created on or before the specified date. */
    pub filter_created_at_end: String,
    /* Only include API keys modified on or after the specified date. */
    pub filter_modified_at_start: String,
    /* Only include API keys modified on or before the specified date. */
    pub filter_modified_at_end: String,
    /* Comma separated list of resource paths for related resources to include in the response. Supported resource paths are `created_by` and `modified_by`. */
    pub include: String,
}

// ListApplicationKeysParams is a struct for passing parameters to the method [`ListApplicationKeys`]
#[derive(Clone, Debug)]
pub struct ListApplicationKeysParams {
    /* Size for a given page. The maximum allowed value is 100. */
    pub page_size: i64,
    /* Specific page number to return. */
    pub page_number: i64,
    /* Application key attribute used to sort results. Sort order is ascending
by default. In order to specify a descending sort, prefix the
attribute with a minus sign. */
    pub sort: ApplicationKeysSort,
    /* Filter application keys by the specified string. */
    pub filter: String,
    /* Only include application keys created on or after the specified date. */
    pub filter_created_at_start: String,
    /* Only include application keys created on or before the specified date. */
    pub filter_created_at_end: String,
}

// ListCurrentUserApplicationKeysParams is a struct for passing parameters to the method [`ListCurrentUserApplicationKeys`]
#[derive(Clone, Debug)]
pub struct ListCurrentUserApplicationKeysParams {
    /* Size for a given page. The maximum allowed value is 100. */
    pub page_size: i64,
    /* Specific page number to return. */
    pub page_number: i64,
    /* Application key attribute used to sort results. Sort order is ascending
by default. In order to specify a descending sort, prefix the
attribute with a minus sign. */
    pub sort: ApplicationKeysSort,
    /* Filter application keys by the specified string. */
    pub filter: String,
    /* Only include application keys created on or after the specified date. */
    pub filter_created_at_start: String,
    /* Only include application keys created on or before the specified date. */
    pub filter_created_at_end: String,
}

// UpdateAPIKeyParams is a struct for passing parameters to the method [`UpdateAPIKey`]
#[derive(Clone, Debug)]
pub struct UpdateAPIKeyParams {
    /* The ID of the API key. */
    pub api_key_id: String,
    pub body: APIKeyUpdateRequest,
}

// UpdateApplicationKeyParams is a struct for passing parameters to the method [`UpdateApplicationKey`]
#[derive(Clone, Debug)]
pub struct UpdateApplicationKeyParams {
    /* The ID of the application key. */
    pub app_key_id: String,
    pub body: ApplicationKeyUpdateRequest,
}

// UpdateCurrentUserApplicationKeyParams is a struct for passing parameters to the method [`UpdateCurrentUserApplicationKey`]
#[derive(Clone, Debug)]
pub struct UpdateCurrentUserApplicationKeyParams {
    /* The ID of the application key. */
    pub app_key_id: String,
    pub body: ApplicationKeyUpdateRequest,
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

/// CreateCurrentUserApplicationKeyError is a struct for typed errors of method [`CreateCurrentUserApplicationKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCurrentUserApplicationKeyError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteAPIKeyError is a struct for typed errors of method [`DeleteAPIKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAPIKeyError {
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

/// DeleteCurrentUserApplicationKeyError is a struct for typed errors of method [`DeleteCurrentUserApplicationKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCurrentUserApplicationKeyError {
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
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCurrentUserApplicationKeyError is a struct for typed errors of method [`GetCurrentUserApplicationKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCurrentUserApplicationKeyError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAPIKeysError is a struct for typed errors of method [`ListAPIKeys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAPIKeysError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListApplicationKeysError is a struct for typed errors of method [`ListApplicationKeys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListApplicationKeysError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCurrentUserApplicationKeysError is a struct for typed errors of method [`ListCurrentUserApplicationKeys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCurrentUserApplicationKeysError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
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
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateCurrentUserApplicationKeyError is a struct for typed errors of method [`UpdateCurrentUserApplicationKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCurrentUserApplicationKeyError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}