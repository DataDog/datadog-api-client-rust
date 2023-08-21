// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// AggregateLogsParams is a struct for passing parameters to the method [`AggregateLogs`]
#[derive(Clone, Debug)]
pub struct AggregateLogsParams {
    pub body: LogsAggregateRequest,
}

// ListLogsParams is a struct for passing parameters to the method [`ListLogs`]
#[derive(Clone, Debug)]
pub struct ListLogsParams {
    pub body: LogsListRequest,
}

// ListLogsGetParams is a struct for passing parameters to the method [`ListLogsGet`]
#[derive(Clone, Debug)]
pub struct ListLogsGetParams {
    /* Search query following logs syntax. */
    pub filter_query: String,
    /* For customers with multiple indexes, the indexes to search.
Defaults to '*' which means all indexes */
    pub filter_indexes: Vec<String>,
    /* Minimum timestamp for requested logs. */
    pub filter_from: String,
    /* Maximum timestamp for requested logs. */
    pub filter_to: String,
    /* Specifies the storage type to be used */
    pub filter_storage_tier: LogsStorageTier,
    /* Order of logs in results. */
    pub sort: LogsSort,
    /* List following results with a cursor provided in the previous query. */
    pub page_cursor: String,
    /* Maximum number of logs in the response. */
    pub page_limit: i32,
}

// SubmitLogParams is a struct for passing parameters to the method [`SubmitLog`]
#[derive(Clone, Debug)]
pub struct SubmitLogParams {
    /* Log to send (JSON format). */
    pub body: Vec<HTTPLogItem>,
    /* HTTP header used to compress the media-type. */
    pub content_encoding: ContentEncoding,
    /* Log tags can be passed as query parameters with `text/plain` content type. */
    pub ddtags: String,
}




/// AggregateLogsError is a struct for typed errors of method [`AggregateLogs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AggregateLogsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListLogsError is a struct for typed errors of method [`ListLogs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLogsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListLogsGetError is a struct for typed errors of method [`ListLogsGet`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLogsGetError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SubmitLogError is a struct for typed errors of method [`SubmitLog`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitLogError {
    Status400(HTTPLogErrors),
    Status401(HTTPLogErrors),
    Status403(HTTPLogErrors),
    Status408(HTTPLogErrors),
    Status413(HTTPLogErrors),
    Status429(HTTPLogErrors),
    Status500(HTTPLogErrors),
    Status503(HTTPLogErrors),
    UnknownValue(serde_json::Value),
}