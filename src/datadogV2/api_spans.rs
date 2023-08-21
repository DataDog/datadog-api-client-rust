// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// AggregateSpansParams is a struct for passing parameters to the method [`AggregateSpans`]
#[derive(Clone, Debug)]
pub struct AggregateSpansParams {
    pub body: SpansAggregateRequest,
}

// ListSpansParams is a struct for passing parameters to the method [`ListSpans`]
#[derive(Clone, Debug)]
pub struct ListSpansParams {
    pub body: SpansListRequest,
}

// ListSpansGetParams is a struct for passing parameters to the method [`ListSpansGet`]
#[derive(Clone, Debug)]
pub struct ListSpansGetParams {
    /* Search query following spans syntax. */
    pub filter_query: String,
    /* Minimum timestamp for requested spans. Supports date-time ISO8601, date math, and regular timestamps (milliseconds). */
    pub filter_from: String,
    /* Maximum timestamp for requested spans. Supports date-time ISO8601, date math, and regular timestamps (milliseconds). */
    pub filter_to: String,
    /* Order of spans in results. */
    pub sort: SpansSort,
    /* List following results with a cursor provided in the previous query. */
    pub page_cursor: String,
    /* Maximum number of spans in the response. */
    pub page_limit: i32,
}




/// AggregateSpansError is a struct for typed errors of method [`AggregateSpans`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AggregateSpansError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListSpansError is a struct for typed errors of method [`ListSpans`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSpansError {
    Status400(JSONAPIErrorResponse),
    Status403(JSONAPIErrorResponse),
    Status422(JSONAPIErrorResponse),
    Status429(JSONAPIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListSpansGetError is a struct for typed errors of method [`ListSpansGet`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSpansGetError {
    Status400(JSONAPIErrorResponse),
    Status403(JSONAPIErrorResponse),
    Status422(JSONAPIErrorResponse),
    Status429(JSONAPIErrorResponse),
    UnknownValue(serde_json::Value),
}