// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateGCPSTSAccountParams is a struct for passing parameters to the method [`CreateGCPSTSAccount`]
#[derive(Clone, Debug)]
pub struct CreateGCPSTSAccountParams {
    pub body: GCPSTSServiceAccountCreateRequest,
}

// DeleteGCPSTSAccountParams is a struct for passing parameters to the method [`DeleteGCPSTSAccount`]
#[derive(Clone, Debug)]
pub struct DeleteGCPSTSAccountParams {
    /* Your GCP STS enabled service account's unique ID. */
    pub account_id: String,
}

// MakeGCPSTSDelegateParams is a struct for passing parameters to the method [`MakeGCPSTSDelegate`]
#[derive(Clone, Debug)]
pub struct MakeGCPSTSDelegateParams {
    /* Create a delegate service account within Datadog. */
    pub body: interface{},
}

// UpdateGCPSTSAccountParams is a struct for passing parameters to the method [`UpdateGCPSTSAccount`]
#[derive(Clone, Debug)]
pub struct UpdateGCPSTSAccountParams {
    /* Your GCP STS enabled service account's unique ID. */
    pub account_id: String,
    pub body: GCPSTSServiceAccountUpdateRequest,
}




/// CreateGCPSTSAccountError is a struct for typed errors of method [`CreateGCPSTSAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGCPSTSAccountError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteGCPSTSAccountError is a struct for typed errors of method [`DeleteGCPSTSAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGCPSTSAccountError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetGCPSTSDelegateError is a struct for typed errors of method [`GetGCPSTSDelegate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGCPSTSDelegateError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListGCPSTSAccountsError is a struct for typed errors of method [`ListGCPSTSAccounts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGCPSTSAccountsError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// MakeGCPSTSDelegateError is a struct for typed errors of method [`MakeGCPSTSDelegate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MakeGCPSTSDelegateError {
    Status403(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateGCPSTSAccountError is a struct for typed errors of method [`UpdateGCPSTSAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGCPSTSAccountError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}