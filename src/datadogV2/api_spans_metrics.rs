// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateSpansMetricParams is a struct for passing parameters to the method [`CreateSpansMetric`]
#[derive(Clone, Debug)]
pub struct CreateSpansMetricParams {
    /* The definition of the new span-based metric. */
    pub body: SpansMetricCreateRequest,
}

// DeleteSpansMetricParams is a struct for passing parameters to the method [`DeleteSpansMetric`]
#[derive(Clone, Debug)]
pub struct DeleteSpansMetricParams {
    /* The name of the span-based metric. */
    pub metric_id: String,
}

// GetSpansMetricParams is a struct for passing parameters to the method [`GetSpansMetric`]
#[derive(Clone, Debug)]
pub struct GetSpansMetricParams {
    /* The name of the span-based metric. */
    pub metric_id: String,
}

// UpdateSpansMetricParams is a struct for passing parameters to the method [`UpdateSpansMetric`]
#[derive(Clone, Debug)]
pub struct UpdateSpansMetricParams {
    /* The name of the span-based metric. */
    pub metric_id: String,
    /* New definition of the span-based metric. */
    pub body: SpansMetricUpdateRequest,
}




/// CreateSpansMetricError is a struct for typed errors of method [`CreateSpansMetric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSpansMetricError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteSpansMetricError is a struct for typed errors of method [`DeleteSpansMetric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSpansMetricError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSpansMetricError is a struct for typed errors of method [`GetSpansMetric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSpansMetricError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListSpansMetricsError is a struct for typed errors of method [`ListSpansMetrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSpansMetricsError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateSpansMetricError is a struct for typed errors of method [`UpdateSpansMetric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSpansMetricError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}