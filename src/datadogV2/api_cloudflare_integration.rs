// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateCloudflareAccountParams is a struct for passing parameters to the method [`CreateCloudflareAccount`]
#[derive(Clone, Debug)]
pub struct CreateCloudflareAccountParams {
    pub body: CloudflareAccountCreateRequest,
}

// DeleteCloudflareAccountParams is a struct for passing parameters to the method [`DeleteCloudflareAccount`]
#[derive(Clone, Debug)]
pub struct DeleteCloudflareAccountParams {
    /* None */
    pub account_id: String,
}

// GetCloudflareAccountParams is a struct for passing parameters to the method [`GetCloudflareAccount`]
#[derive(Clone, Debug)]
pub struct GetCloudflareAccountParams {
    /* None */
    pub account_id: String,
}

// UpdateCloudflareAccountParams is a struct for passing parameters to the method [`UpdateCloudflareAccount`]
#[derive(Clone, Debug)]
pub struct UpdateCloudflareAccountParams {
    /* None */
    pub account_id: String,
    pub body: CloudflareAccountUpdateRequest,
}




/// CreateCloudflareAccountError is a struct for typed errors of method [`CreateCloudflareAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCloudflareAccountError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteCloudflareAccountError is a struct for typed errors of method [`DeleteCloudflareAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCloudflareAccountError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCloudflareAccountError is a struct for typed errors of method [`GetCloudflareAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCloudflareAccountError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCloudflareAccountsError is a struct for typed errors of method [`ListCloudflareAccounts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCloudflareAccountsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateCloudflareAccountError is a struct for typed errors of method [`UpdateCloudflareAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCloudflareAccountError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}