// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateIncidentServiceParams is a struct for passing parameters to the method [`CreateIncidentService`]
#[derive(Clone, Debug)]
pub struct CreateIncidentServiceParams {
    /* Incident Service Payload. */
    pub body: IncidentServiceCreateRequest,
}

// DeleteIncidentServiceParams is a struct for passing parameters to the method [`DeleteIncidentService`]
#[derive(Clone, Debug)]
pub struct DeleteIncidentServiceParams {
    /* The ID of the incident service. */
    pub service_id: String,
}

// GetIncidentServiceParams is a struct for passing parameters to the method [`GetIncidentService`]
#[derive(Clone, Debug)]
pub struct GetIncidentServiceParams {
    /* The ID of the incident service. */
    pub service_id: String,
    /* Specifies which types of related objects should be included in the response. */
    pub include: IncidentRelatedObject,
}

// ListIncidentServicesParams is a struct for passing parameters to the method [`ListIncidentServices`]
#[derive(Clone, Debug)]
pub struct ListIncidentServicesParams {
    /* Specifies which types of related objects should be included in the response. */
    pub include: IncidentRelatedObject,
    /* Size for a given page. The maximum allowed value is 100. */
    pub page_size: i64,
    /* Specific offset to use as the beginning of the returned page. */
    pub page_offset: i64,
    /* A search query that filters services by name. */
    pub filter: String,
}

// UpdateIncidentServiceParams is a struct for passing parameters to the method [`UpdateIncidentService`]
#[derive(Clone, Debug)]
pub struct UpdateIncidentServiceParams {
    /* The ID of the incident service. */
    pub service_id: String,
    /* Incident Service Payload. */
    pub body: IncidentServiceUpdateRequest,
}




/// CreateIncidentServiceError is a struct for typed errors of method [`CreateIncidentService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateIncidentServiceError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteIncidentServiceError is a struct for typed errors of method [`DeleteIncidentService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteIncidentServiceError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetIncidentServiceError is a struct for typed errors of method [`GetIncidentService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIncidentServiceError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListIncidentServicesError is a struct for typed errors of method [`ListIncidentServices`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListIncidentServicesError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateIncidentServiceError is a struct for typed errors of method [`UpdateIncidentService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateIncidentServiceError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}