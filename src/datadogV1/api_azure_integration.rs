// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateAzureIntegrationParams is a struct for passing parameters to the method [`CreateAzureIntegration`]
#[derive(Clone, Debug)]
pub struct CreateAzureIntegrationParams {
    /* Create a Datadog-Azure integration for your Datadog account request body. */
    pub body: AzureAccount,
}

// DeleteAzureIntegrationParams is a struct for passing parameters to the method [`DeleteAzureIntegration`]
#[derive(Clone, Debug)]
pub struct DeleteAzureIntegrationParams {
    /* Delete a given Datadog-Azure integration request body. */
    pub body: AzureAccount,
}

// UpdateAzureHostFiltersParams is a struct for passing parameters to the method [`UpdateAzureHostFilters`]
#[derive(Clone, Debug)]
pub struct UpdateAzureHostFiltersParams {
    /* Update a Datadog-Azure integration's host filters request body. */
    pub body: AzureAccount,
}

// UpdateAzureIntegrationParams is a struct for passing parameters to the method [`UpdateAzureIntegration`]
#[derive(Clone, Debug)]
pub struct UpdateAzureIntegrationParams {
    /* Update a Datadog-Azure integration request body. */
    pub body: AzureAccount,
}




/// CreateAzureIntegrationError is a struct for typed errors of method [`CreateAzureIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAzureIntegrationError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteAzureIntegrationError is a struct for typed errors of method [`DeleteAzureIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAzureIntegrationError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAzureIntegrationError is a struct for typed errors of method [`ListAzureIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAzureIntegrationError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateAzureHostFiltersError is a struct for typed errors of method [`UpdateAzureHostFilters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAzureHostFiltersError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateAzureIntegrationError is a struct for typed errors of method [`UpdateAzureIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAzureIntegrationError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}