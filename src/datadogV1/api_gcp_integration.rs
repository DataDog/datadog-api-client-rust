// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateGCPIntegrationParams is a struct for passing parameters to the method [`CreateGCPIntegration`]
#[derive(Clone, Debug)]
pub struct CreateGCPIntegrationParams {
    /* Create a Datadog-GCP integration. */
    pub body: GCPAccount,
}

// DeleteGCPIntegrationParams is a struct for passing parameters to the method [`DeleteGCPIntegration`]
#[derive(Clone, Debug)]
pub struct DeleteGCPIntegrationParams {
    /* Delete a given Datadog-GCP integration. */
    pub body: GCPAccount,
}

// UpdateGCPIntegrationParams is a struct for passing parameters to the method [`UpdateGCPIntegration`]
#[derive(Clone, Debug)]
pub struct UpdateGCPIntegrationParams {
    /* Update a Datadog-GCP integration. */
    pub body: GCPAccount,
}




/// CreateGCPIntegrationError is a struct for typed errors of method [`CreateGCPIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGCPIntegrationError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteGCPIntegrationError is a struct for typed errors of method [`DeleteGCPIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGCPIntegrationError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListGCPIntegrationError is a struct for typed errors of method [`ListGCPIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGCPIntegrationError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateGCPIntegrationError is a struct for typed errors of method [`UpdateGCPIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGCPIntegrationError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}