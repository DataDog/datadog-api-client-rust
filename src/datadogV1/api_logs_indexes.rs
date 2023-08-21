// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateLogsIndexParams is a struct for passing parameters to the method [`CreateLogsIndex`]
#[derive(Clone, Debug)]
pub struct CreateLogsIndexParams {
    /* Object containing the new index. */
    pub body: LogsIndex,
}

// GetLogsIndexParams is a struct for passing parameters to the method [`GetLogsIndex`]
#[derive(Clone, Debug)]
pub struct GetLogsIndexParams {
    /* Name of the log index. */
    pub name: String,
}

// UpdateLogsIndexParams is a struct for passing parameters to the method [`UpdateLogsIndex`]
#[derive(Clone, Debug)]
pub struct UpdateLogsIndexParams {
    /* Name of the log index. */
    pub name: String,
    /* Object containing the new `LogsIndexUpdateRequest`. */
    pub body: LogsIndexUpdateRequest,
}

// UpdateLogsIndexOrderParams is a struct for passing parameters to the method [`UpdateLogsIndexOrder`]
#[derive(Clone, Debug)]
pub struct UpdateLogsIndexOrderParams {
    /* Object containing the new ordered list of index names */
    pub body: LogsIndexesOrder,
}




/// CreateLogsIndexError is a struct for typed errors of method [`CreateLogsIndex`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLogsIndexError {
    Status400(LogsAPIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetLogsIndexError is a struct for typed errors of method [`GetLogsIndex`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogsIndexError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    Status404(LogsAPIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetLogsIndexOrderError is a struct for typed errors of method [`GetLogsIndexOrder`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogsIndexOrderError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListLogIndexesError is a struct for typed errors of method [`ListLogIndexes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLogIndexesError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateLogsIndexError is a struct for typed errors of method [`UpdateLogsIndex`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLogsIndexError {
    Status400(LogsAPIErrorResponse),
    Status429(LogsAPIErrorResponse),
    Status403(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateLogsIndexOrderError is a struct for typed errors of method [`UpdateLogsIndexOrder`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLogsIndexOrderError {
    Status400(LogsAPIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}