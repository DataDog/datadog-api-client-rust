// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

// CreateFastlyAccountParams is a struct for passing parameters to the method [`CreateFastlyAccount`]
#[derive(Clone, Debug)]
pub struct CreateFastlyAccountParams {
    pub body: FastlyAccountCreateRequest,
}

// CreateFastlyServiceParams is a struct for passing parameters to the method [`CreateFastlyService`]
#[derive(Clone, Debug)]
pub struct CreateFastlyServiceParams {
    /* Fastly Account id. */
    pub account_id: String,
    pub body: FastlyServiceRequest,
}

// DeleteFastlyAccountParams is a struct for passing parameters to the method [`DeleteFastlyAccount`]
#[derive(Clone, Debug)]
pub struct DeleteFastlyAccountParams {
    /* Fastly Account id. */
    pub account_id: String,
}

// DeleteFastlyServiceParams is a struct for passing parameters to the method [`DeleteFastlyService`]
#[derive(Clone, Debug)]
pub struct DeleteFastlyServiceParams {
    /* Fastly Account id. */
    pub account_id: String,
    /* Fastly Service ID. */
    pub service_id: String,
}

// GetFastlyAccountParams is a struct for passing parameters to the method [`GetFastlyAccount`]
#[derive(Clone, Debug)]
pub struct GetFastlyAccountParams {
    /* Fastly Account id. */
    pub account_id: String,
}

// GetFastlyServiceParams is a struct for passing parameters to the method [`GetFastlyService`]
#[derive(Clone, Debug)]
pub struct GetFastlyServiceParams {
    /* Fastly Account id. */
    pub account_id: String,
    /* Fastly Service ID. */
    pub service_id: String,
}

// ListFastlyServicesParams is a struct for passing parameters to the method [`ListFastlyServices`]
#[derive(Clone, Debug)]
pub struct ListFastlyServicesParams {
    /* Fastly Account id. */
    pub account_id: String,
}

// UpdateFastlyAccountParams is a struct for passing parameters to the method [`UpdateFastlyAccount`]
#[derive(Clone, Debug)]
pub struct UpdateFastlyAccountParams {
    /* Fastly Account id. */
    pub account_id: String,
    pub body: FastlyAccountUpdateRequest,
}

// UpdateFastlyServiceParams is a struct for passing parameters to the method [`UpdateFastlyService`]
#[derive(Clone, Debug)]
pub struct UpdateFastlyServiceParams {
    /* Fastly Account id. */
    pub account_id: String,
    /* Fastly Service ID. */
    pub service_id: String,
    pub body: FastlyServiceRequest,
}

/// CreateFastlyAccountError is a struct for typed errors of method [`CreateFastlyAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFastlyAccountError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateFastlyServiceError is a struct for typed errors of method [`CreateFastlyService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFastlyServiceError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteFastlyAccountError is a struct for typed errors of method [`DeleteFastlyAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFastlyAccountError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteFastlyServiceError is a struct for typed errors of method [`DeleteFastlyService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFastlyServiceError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetFastlyAccountError is a struct for typed errors of method [`GetFastlyAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFastlyAccountError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetFastlyServiceError is a struct for typed errors of method [`GetFastlyService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFastlyServiceError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListFastlyAccountsError is a struct for typed errors of method [`ListFastlyAccounts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFastlyAccountsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListFastlyServicesError is a struct for typed errors of method [`ListFastlyServices`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFastlyServicesError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateFastlyAccountError is a struct for typed errors of method [`UpdateFastlyAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateFastlyAccountError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateFastlyServiceError is a struct for typed errors of method [`UpdateFastlyService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateFastlyServiceError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}
