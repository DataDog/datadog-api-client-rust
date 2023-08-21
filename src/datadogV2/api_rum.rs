// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// AggregateRUMEventsParams is a struct for passing parameters to the method [`AggregateRUMEvents`]
#[derive(Clone, Debug)]
pub struct AggregateRUMEventsParams {
    pub body: RUMAggregateRequest,
}

// CreateRUMApplicationParams is a struct for passing parameters to the method [`CreateRUMApplication`]
#[derive(Clone, Debug)]
pub struct CreateRUMApplicationParams {
    pub body: RUMApplicationCreateRequest,
}

// DeleteRUMApplicationParams is a struct for passing parameters to the method [`DeleteRUMApplication`]
#[derive(Clone, Debug)]
pub struct DeleteRUMApplicationParams {
    /* RUM application ID. */
    pub id: String,
}

// GetRUMApplicationParams is a struct for passing parameters to the method [`GetRUMApplication`]
#[derive(Clone, Debug)]
pub struct GetRUMApplicationParams {
    /* RUM application ID. */
    pub id: String,
}

// ListRUMEventsParams is a struct for passing parameters to the method [`ListRUMEvents`]
#[derive(Clone, Debug)]
pub struct ListRUMEventsParams {
    /* Search query following RUM syntax. */
    pub filter_query: String,
    /* Minimum timestamp for requested events. */
    pub filter_from: String,
    /* Maximum timestamp for requested events. */
    pub filter_to: String,
    /* Order of events in results. */
    pub sort: RUMSort,
    /* List following results with a cursor provided in the previous query. */
    pub page_cursor: String,
    /* Maximum number of events in the response. */
    pub page_limit: i32,
}

// SearchRUMEventsParams is a struct for passing parameters to the method [`SearchRUMEvents`]
#[derive(Clone, Debug)]
pub struct SearchRUMEventsParams {
    pub body: RUMSearchEventsRequest,
}

// UpdateRUMApplicationParams is a struct for passing parameters to the method [`UpdateRUMApplication`]
#[derive(Clone, Debug)]
pub struct UpdateRUMApplicationParams {
    /* RUM application ID. */
    pub id: String,
    pub body: RUMApplicationUpdateRequest,
}




/// AggregateRUMEventsError is a struct for typed errors of method [`AggregateRUMEvents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AggregateRUMEventsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateRUMApplicationError is a struct for typed errors of method [`CreateRUMApplication`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateRUMApplicationError {
    Status400(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteRUMApplicationError is a struct for typed errors of method [`DeleteRUMApplication`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRUMApplicationError {
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetRUMApplicationError is a struct for typed errors of method [`GetRUMApplication`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRUMApplicationError {
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetRUMApplicationsError is a struct for typed errors of method [`GetRUMApplications`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRUMApplicationsError {
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListRUMEventsError is a struct for typed errors of method [`ListRUMEvents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRUMEventsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SearchRUMEventsError is a struct for typed errors of method [`SearchRUMEvents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchRUMEventsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateRUMApplicationError is a struct for typed errors of method [`UpdateRUMApplication`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateRUMApplicationError {
    Status400(APIErrorResponse),
    Status404(APIErrorResponse),
    Status422(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}