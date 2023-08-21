// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateServiceAccountParams is a struct for passing parameters to the method [`CreateServiceAccount`]
#[derive(Clone, Debug)]
pub struct CreateServiceAccountParams {
    pub body: ServiceAccountCreateRequest,
}

// CreateServiceAccountApplicationKeyParams is a struct for passing parameters to the method [`CreateServiceAccountApplicationKey`]
#[derive(Clone, Debug)]
pub struct CreateServiceAccountApplicationKeyParams {
    /* The ID of the service account. */
    pub service_account_id: String,
    pub body: ApplicationKeyCreateRequest,
}

// DeleteServiceAccountApplicationKeyParams is a struct for passing parameters to the method [`DeleteServiceAccountApplicationKey`]
#[derive(Clone, Debug)]
pub struct DeleteServiceAccountApplicationKeyParams {
    /* The ID of the service account. */
    pub service_account_id: String,
    /* The ID of the application key. */
    pub app_key_id: String,
}

// GetServiceAccountApplicationKeyParams is a struct for passing parameters to the method [`GetServiceAccountApplicationKey`]
#[derive(Clone, Debug)]
pub struct GetServiceAccountApplicationKeyParams {
    /* The ID of the service account. */
    pub service_account_id: String,
    /* The ID of the application key. */
    pub app_key_id: String,
}

// ListServiceAccountApplicationKeysParams is a struct for passing parameters to the method [`ListServiceAccountApplicationKeys`]
#[derive(Clone, Debug)]
pub struct ListServiceAccountApplicationKeysParams {
    /* The ID of the service account. */
    pub service_account_id: String,
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

// UpdateServiceAccountApplicationKeyParams is a struct for passing parameters to the method [`UpdateServiceAccountApplicationKey`]
#[derive(Clone, Debug)]
pub struct UpdateServiceAccountApplicationKeyParams {
    /* The ID of the service account. */
    pub service_account_id: String,
    /* The ID of the application key. */
    pub app_key_id: String,
    pub body: ApplicationKeyUpdateRequest,
}




/// CreateServiceAccountError is a struct for typed errors of method [`CreateServiceAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateServiceAccountError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateServiceAccountApplicationKeyError is a struct for typed errors of method [`CreateServiceAccountApplicationKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateServiceAccountApplicationKeyError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteServiceAccountApplicationKeyError is a struct for typed errors of method [`DeleteServiceAccountApplicationKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteServiceAccountApplicationKeyError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetServiceAccountApplicationKeyError is a struct for typed errors of method [`GetServiceAccountApplicationKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetServiceAccountApplicationKeyError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListServiceAccountApplicationKeysError is a struct for typed errors of method [`ListServiceAccountApplicationKeys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListServiceAccountApplicationKeysError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateServiceAccountApplicationKeyError is a struct for typed errors of method [`UpdateServiceAccountApplicationKey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateServiceAccountApplicationKeyError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}