// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// ListEventsParams is a struct for passing parameters to the method [`ListEvents`]
#[derive(Clone, Debug)]
pub struct ListEventsParams {
    /* Search query following events syntax. */
    pub filter_query: String,
    /* Minimum timestamp for requested events. */
    pub filter_from: String,
    /* Maximum timestamp for requested events. */
    pub filter_to: String,
    /* Order of events in results. */
    pub sort: EventsSort,
    /* List following results with a cursor provided in the previous query. */
    pub page_cursor: String,
    /* Maximum number of events in the response. */
    pub page_limit: i32,
}

// SearchEventsParams is a struct for passing parameters to the method [`SearchEvents`]
#[derive(Clone, Debug)]
pub struct SearchEventsParams {
    pub body: EventsListRequest,
}




/// ListEventsError is a struct for typed errors of method [`ListEvents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListEventsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SearchEventsError is a struct for typed errors of method [`SearchEvents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchEventsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}