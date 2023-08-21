// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// ListLogsParams is a struct for passing parameters to the method [`ListLogs`]
#[derive(Clone, Debug)]
pub struct ListLogsParams {
    /* Logs filter */
    pub body: LogsListRequest,
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




/// ListLogsError is a struct for typed errors of method [`ListLogs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLogsError {
    Status400(LogsAPIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SubmitLogError is a struct for typed errors of method [`SubmitLog`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitLogError {
    Status400(HTTPLogError),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}