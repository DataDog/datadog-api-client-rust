// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// AggregateCIAppPipelineEventsParams is a struct for passing parameters to the method [`AggregateCIAppPipelineEvents`]
#[derive(Clone, Debug)]
pub struct AggregateCIAppPipelineEventsParams {
    pub body: CIAppPipelinesAggregateRequest,
}

// CreateCIAppPipelineEventParams is a struct for passing parameters to the method [`CreateCIAppPipelineEvent`]
#[derive(Clone, Debug)]
pub struct CreateCIAppPipelineEventParams {
    pub body: CIAppCreatePipelineEventRequest,
}

// ListCIAppPipelineEventsParams is a struct for passing parameters to the method [`ListCIAppPipelineEvents`]
#[derive(Clone, Debug)]
pub struct ListCIAppPipelineEventsParams {
    /* Search query following log syntax. */
    pub filter_query: String,
    /* Minimum timestamp for requested events. */
    pub filter_from: String,
    /* Maximum timestamp for requested events. */
    pub filter_to: String,
    /* Order of events in results. */
    pub sort: CIAppSort,
    /* List following results with a cursor provided in the previous query. */
    pub page_cursor: String,
    /* Maximum number of events in the response. */
    pub page_limit: i32,
}

// SearchCIAppPipelineEventsParams is a struct for passing parameters to the method [`SearchCIAppPipelineEvents`]
#[derive(Clone, Debug)]
pub struct SearchCIAppPipelineEventsParams {
    pub body: CIAppPipelineEventsRequest,
}




/// AggregateCIAppPipelineEventsError is a struct for typed errors of method [`AggregateCIAppPipelineEvents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AggregateCIAppPipelineEventsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateCIAppPipelineEventError is a struct for typed errors of method [`CreateCIAppPipelineEvent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCIAppPipelineEventError {
    Status400(HTTPCIAppErrors),
    Status401(HTTPCIAppErrors),
    Status403(HTTPCIAppErrors),
    Status408(HTTPCIAppErrors),
    Status413(HTTPCIAppErrors),
    Status429(HTTPCIAppErrors),
    Status500(HTTPCIAppErrors),
    Status503(HTTPCIAppErrors),
    UnknownValue(serde_json::Value),
}

/// ListCIAppPipelineEventsError is a struct for typed errors of method [`ListCIAppPipelineEvents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCIAppPipelineEventsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SearchCIAppPipelineEventsError is a struct for typed errors of method [`SearchCIAppPipelineEvents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchCIAppPipelineEventsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}