// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreatePagerDutyIntegrationServiceParams is a struct for passing parameters to the method [`CreatePagerDutyIntegrationService`]
#[derive(Clone, Debug)]
pub struct CreatePagerDutyIntegrationServiceParams {
    /* Create a new service object request body. */
    pub body: PagerDutyService,
}

// DeletePagerDutyIntegrationServiceParams is a struct for passing parameters to the method [`DeletePagerDutyIntegrationService`]
#[derive(Clone, Debug)]
pub struct DeletePagerDutyIntegrationServiceParams {
    /* The service name */
    pub service_name: String,
}

// GetPagerDutyIntegrationServiceParams is a struct for passing parameters to the method [`GetPagerDutyIntegrationService`]
#[derive(Clone, Debug)]
pub struct GetPagerDutyIntegrationServiceParams {
    /* The service name. */
    pub service_name: String,
}

// UpdatePagerDutyIntegrationServiceParams is a struct for passing parameters to the method [`UpdatePagerDutyIntegrationService`]
#[derive(Clone, Debug)]
pub struct UpdatePagerDutyIntegrationServiceParams {
    /* The service name */
    pub service_name: String,
    /* Update an existing service object request body. */
    pub body: PagerDutyServiceKey,
}




/// CreatePagerDutyIntegrationServiceError is a struct for typed errors of method [`CreatePagerDutyIntegrationService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePagerDutyIntegrationServiceError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeletePagerDutyIntegrationServiceError is a struct for typed errors of method [`DeletePagerDutyIntegrationService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePagerDutyIntegrationServiceError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetPagerDutyIntegrationServiceError is a struct for typed errors of method [`GetPagerDutyIntegrationService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPagerDutyIntegrationServiceError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdatePagerDutyIntegrationServiceError is a struct for typed errors of method [`UpdatePagerDutyIntegrationService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePagerDutyIntegrationServiceError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}