// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateLogsMetricParams is a struct for passing parameters to the method [`CreateLogsMetric`]
#[derive(Clone, Debug)]
pub struct CreateLogsMetricParams {
    /* The definition of the new log-based metric. */
    pub body: LogsMetricCreateRequest,
}

// DeleteLogsMetricParams is a struct for passing parameters to the method [`DeleteLogsMetric`]
#[derive(Clone, Debug)]
pub struct DeleteLogsMetricParams {
    /* The name of the log-based metric. */
    pub metric_id: String,
}

// GetLogsMetricParams is a struct for passing parameters to the method [`GetLogsMetric`]
#[derive(Clone, Debug)]
pub struct GetLogsMetricParams {
    /* The name of the log-based metric. */
    pub metric_id: String,
}

// UpdateLogsMetricParams is a struct for passing parameters to the method [`UpdateLogsMetric`]
#[derive(Clone, Debug)]
pub struct UpdateLogsMetricParams {
    /* The name of the log-based metric. */
    pub metric_id: String,
    /* New definition of the log-based metric. */
    pub body: LogsMetricUpdateRequest,
}




/// CreateLogsMetricError is a struct for typed errors of method [`CreateLogsMetric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLogsMetricError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteLogsMetricError is a struct for typed errors of method [`DeleteLogsMetric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLogsMetricError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetLogsMetricError is a struct for typed errors of method [`GetLogsMetric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogsMetricError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListLogsMetricsError is a struct for typed errors of method [`ListLogsMetrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLogsMetricsError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateLogsMetricError is a struct for typed errors of method [`UpdateLogsMetric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLogsMetricError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}