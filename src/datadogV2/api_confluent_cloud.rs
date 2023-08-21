// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateConfluentAccountParams is a struct for passing parameters to the method [`CreateConfluentAccount`]
#[derive(Clone, Debug)]
pub struct CreateConfluentAccountParams {
    /* Confluent payload */
    pub body: ConfluentAccountCreateRequest,
}

// CreateConfluentResourceParams is a struct for passing parameters to the method [`CreateConfluentResource`]
#[derive(Clone, Debug)]
pub struct CreateConfluentResourceParams {
    /* Confluent Account id. */
    pub account_id: String,
    /* Confluent payload */
    pub body: ConfluentResourceRequest,
}

// DeleteConfluentAccountParams is a struct for passing parameters to the method [`DeleteConfluentAccount`]
#[derive(Clone, Debug)]
pub struct DeleteConfluentAccountParams {
    /* Confluent Account id. */
    pub account_id: String,
}

// DeleteConfluentResourceParams is a struct for passing parameters to the method [`DeleteConfluentResource`]
#[derive(Clone, Debug)]
pub struct DeleteConfluentResourceParams {
    /* Confluent Account id. */
    pub account_id: String,
    /* Confluent Account Resource ID. */
    pub resource_id: String,
}

// GetConfluentAccountParams is a struct for passing parameters to the method [`GetConfluentAccount`]
#[derive(Clone, Debug)]
pub struct GetConfluentAccountParams {
    /* Confluent Account id. */
    pub account_id: String,
}

// GetConfluentResourceParams is a struct for passing parameters to the method [`GetConfluentResource`]
#[derive(Clone, Debug)]
pub struct GetConfluentResourceParams {
    /* Confluent Account id. */
    pub account_id: String,
    /* Confluent Account Resource ID. */
    pub resource_id: String,
}

// ListConfluentResourceParams is a struct for passing parameters to the method [`ListConfluentResource`]
#[derive(Clone, Debug)]
pub struct ListConfluentResourceParams {
    /* Confluent Account id. */
    pub account_id: String,
}

// UpdateConfluentAccountParams is a struct for passing parameters to the method [`UpdateConfluentAccount`]
#[derive(Clone, Debug)]
pub struct UpdateConfluentAccountParams {
    /* Confluent Account id. */
    pub account_id: String,
    /* Confluent payload */
    pub body: ConfluentAccountUpdateRequest,
}

// UpdateConfluentResourceParams is a struct for passing parameters to the method [`UpdateConfluentResource`]
#[derive(Clone, Debug)]
pub struct UpdateConfluentResourceParams {
    /* Confluent Account id. */
    pub account_id: String,
    /* Confluent Account Resource ID. */
    pub resource_id: String,
    /* Confluent payload */
    pub body: ConfluentResourceRequest,
}




/// CreateConfluentAccountError is a struct for typed errors of method [`CreateConfluentAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateConfluentAccountError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateConfluentResourceError is a struct for typed errors of method [`CreateConfluentResource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateConfluentResourceError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteConfluentAccountError is a struct for typed errors of method [`DeleteConfluentAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteConfluentAccountError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteConfluentResourceError is a struct for typed errors of method [`DeleteConfluentResource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteConfluentResourceError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetConfluentAccountError is a struct for typed errors of method [`GetConfluentAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetConfluentAccountError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetConfluentResourceError is a struct for typed errors of method [`GetConfluentResource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetConfluentResourceError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListConfluentAccountError is a struct for typed errors of method [`ListConfluentAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListConfluentAccountError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListConfluentResourceError is a struct for typed errors of method [`ListConfluentResource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListConfluentResourceError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateConfluentAccountError is a struct for typed errors of method [`UpdateConfluentAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateConfluentAccountError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateConfluentResourceError is a struct for typed errors of method [`UpdateConfluentResource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateConfluentResourceError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}