// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// ListAuditLogsParams is a struct for passing parameters to the method [`ListAuditLogs`]
#[derive(Clone, Debug)]
pub struct ListAuditLogsParams {
    /* Search query following Audit Logs syntax. */
    pub filter_query: String,
    /* Minimum timestamp for requested events. */
    pub filter_from: String,
    /* Maximum timestamp for requested events. */
    pub filter_to: String,
    /* Order of events in results. */
    pub sort: AuditLogsSort,
    /* List following results with a cursor provided in the previous query. */
    pub page_cursor: String,
    /* Maximum number of events in the response. */
    pub page_limit: i32,
}

// SearchAuditLogsParams is a struct for passing parameters to the method [`SearchAuditLogs`]
#[derive(Clone, Debug)]
pub struct SearchAuditLogsParams {
    pub body: AuditLogsSearchEventsRequest,
}




/// ListAuditLogsError is a struct for typed errors of method [`ListAuditLogs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAuditLogsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SearchAuditLogsError is a struct for typed errors of method [`SearchAuditLogs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchAuditLogsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}