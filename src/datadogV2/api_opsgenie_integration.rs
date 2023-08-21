// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateOpsgenieServiceParams is a struct for passing parameters to the method [`CreateOpsgenieService`]
#[derive(Clone, Debug)]
pub struct CreateOpsgenieServiceParams {
    /* Opsgenie service payload */
    pub body: OpsgenieServiceCreateRequest,
}

// DeleteOpsgenieServiceParams is a struct for passing parameters to the method [`DeleteOpsgenieService`]
#[derive(Clone, Debug)]
pub struct DeleteOpsgenieServiceParams {
    /* The UUID of the service. */
    pub integration_service_id: String,
}

// GetOpsgenieServiceParams is a struct for passing parameters to the method [`GetOpsgenieService`]
#[derive(Clone, Debug)]
pub struct GetOpsgenieServiceParams {
    /* The UUID of the service. */
    pub integration_service_id: String,
}

// UpdateOpsgenieServiceParams is a struct for passing parameters to the method [`UpdateOpsgenieService`]
#[derive(Clone, Debug)]
pub struct UpdateOpsgenieServiceParams {
    /* The UUID of the service. */
    pub integration_service_id: String,
    /* Opsgenie service payload. */
    pub body: OpsgenieServiceUpdateRequest,
}




/// CreateOpsgenieServiceError is a struct for typed errors of method [`CreateOpsgenieService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOpsgenieServiceError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteOpsgenieServiceError is a struct for typed errors of method [`DeleteOpsgenieService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOpsgenieServiceError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetOpsgenieServiceError is a struct for typed errors of method [`GetOpsgenieService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOpsgenieServiceError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListOpsgenieServicesError is a struct for typed errors of method [`ListOpsgenieServices`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListOpsgenieServicesError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateOpsgenieServiceError is a struct for typed errors of method [`UpdateOpsgenieService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateOpsgenieServiceError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}