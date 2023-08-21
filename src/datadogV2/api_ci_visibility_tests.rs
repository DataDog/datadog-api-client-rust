// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// AggregateCIAppTestEventsParams is a struct for passing parameters to the method [`AggregateCIAppTestEvents`]
#[derive(Clone, Debug)]
pub struct AggregateCIAppTestEventsParams {
    pub body: CIAppTestsAggregateRequest,
}

// ListCIAppTestEventsParams is a struct for passing parameters to the method [`ListCIAppTestEvents`]
#[derive(Clone, Debug)]
pub struct ListCIAppTestEventsParams {
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

// SearchCIAppTestEventsParams is a struct for passing parameters to the method [`SearchCIAppTestEvents`]
#[derive(Clone, Debug)]
pub struct SearchCIAppTestEventsParams {
    pub body: CIAppTestEventsRequest,
}




/// AggregateCIAppTestEventsError is a struct for typed errors of method [`AggregateCIAppTestEvents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AggregateCIAppTestEventsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCIAppTestEventsError is a struct for typed errors of method [`ListCIAppTestEvents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCIAppTestEventsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SearchCIAppTestEventsError is a struct for typed errors of method [`SearchCIAppTestEvents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchCIAppTestEventsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}