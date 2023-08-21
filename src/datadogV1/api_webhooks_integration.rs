// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateWebhooksIntegrationParams is a struct for passing parameters to the method [`CreateWebhooksIntegration`]
#[derive(Clone, Debug)]
pub struct CreateWebhooksIntegrationParams {
    /* Create a webhooks integration request body. */
    pub body: WebhooksIntegration,
}

// CreateWebhooksIntegrationCustomVariableParams is a struct for passing parameters to the method [`CreateWebhooksIntegrationCustomVariable`]
#[derive(Clone, Debug)]
pub struct CreateWebhooksIntegrationCustomVariableParams {
    /* Define a custom variable request body. */
    pub body: WebhooksIntegrationCustomVariable,
}

// DeleteWebhooksIntegrationParams is a struct for passing parameters to the method [`DeleteWebhooksIntegration`]
#[derive(Clone, Debug)]
pub struct DeleteWebhooksIntegrationParams {
    /* The name of the webhook. */
    pub webhook_name: String,
}

// DeleteWebhooksIntegrationCustomVariableParams is a struct for passing parameters to the method [`DeleteWebhooksIntegrationCustomVariable`]
#[derive(Clone, Debug)]
pub struct DeleteWebhooksIntegrationCustomVariableParams {
    /* The name of the custom variable. */
    pub custom_variable_name: String,
}

// GetWebhooksIntegrationParams is a struct for passing parameters to the method [`GetWebhooksIntegration`]
#[derive(Clone, Debug)]
pub struct GetWebhooksIntegrationParams {
    /* The name of the webhook. */
    pub webhook_name: String,
}

// GetWebhooksIntegrationCustomVariableParams is a struct for passing parameters to the method [`GetWebhooksIntegrationCustomVariable`]
#[derive(Clone, Debug)]
pub struct GetWebhooksIntegrationCustomVariableParams {
    /* The name of the custom variable. */
    pub custom_variable_name: String,
}

// UpdateWebhooksIntegrationParams is a struct for passing parameters to the method [`UpdateWebhooksIntegration`]
#[derive(Clone, Debug)]
pub struct UpdateWebhooksIntegrationParams {
    /* The name of the webhook. */
    pub webhook_name: String,
    /* Update an existing Datadog-Webhooks integration. */
    pub body: WebhooksIntegrationUpdateRequest,
}

// UpdateWebhooksIntegrationCustomVariableParams is a struct for passing parameters to the method [`UpdateWebhooksIntegrationCustomVariable`]
#[derive(Clone, Debug)]
pub struct UpdateWebhooksIntegrationCustomVariableParams {
    /* The name of the custom variable. */
    pub custom_variable_name: String,
    /* Update an existing custom variable request body. */
    pub body: WebhooksIntegrationCustomVariableUpdateRequest,
}




/// CreateWebhooksIntegrationError is a struct for typed errors of method [`CreateWebhooksIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWebhooksIntegrationError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateWebhooksIntegrationCustomVariableError is a struct for typed errors of method [`CreateWebhooksIntegrationCustomVariable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWebhooksIntegrationCustomVariableError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteWebhooksIntegrationError is a struct for typed errors of method [`DeleteWebhooksIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWebhooksIntegrationError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteWebhooksIntegrationCustomVariableError is a struct for typed errors of method [`DeleteWebhooksIntegrationCustomVariable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWebhooksIntegrationCustomVariableError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetWebhooksIntegrationError is a struct for typed errors of method [`GetWebhooksIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWebhooksIntegrationError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetWebhooksIntegrationCustomVariableError is a struct for typed errors of method [`GetWebhooksIntegrationCustomVariable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWebhooksIntegrationCustomVariableError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateWebhooksIntegrationError is a struct for typed errors of method [`UpdateWebhooksIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWebhooksIntegrationError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateWebhooksIntegrationCustomVariableError is a struct for typed errors of method [`UpdateWebhooksIntegrationCustomVariable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWebhooksIntegrationCustomVariableError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}