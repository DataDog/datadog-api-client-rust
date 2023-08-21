// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// GetMetricMetadataParams is a struct for passing parameters to the method [`GetMetricMetadata`]
#[derive(Clone, Debug)]
pub struct GetMetricMetadataParams {
    /* Name of the metric for which to get metadata. */
    pub metric_name: String,
}

// ListActiveMetricsParams is a struct for passing parameters to the method [`ListActiveMetrics`]
#[derive(Clone, Debug)]
pub struct ListActiveMetricsParams {
    /* Seconds since the Unix epoch. */
    pub from: i64,
    /* Hostname for filtering the list of metrics returned.
If set, metrics retrieved are those with the corresponding hostname tag. */
    pub host: String,
    /* Filter metrics that have been submitted with the given tags. Supports boolean and wildcard expressions.
Cannot be combined with other filters. */
    pub tag_filter: String,
}

// ListMetricsParams is a struct for passing parameters to the method [`ListMetrics`]
#[derive(Clone, Debug)]
pub struct ListMetricsParams {
    /* Query string to search metrics upon. Can optionally be prefixed with `metrics:`. */
    pub q: String,
}

// QueryMetricsParams is a struct for passing parameters to the method [`QueryMetrics`]
#[derive(Clone, Debug)]
pub struct QueryMetricsParams {
    /* Start of the queried time period, seconds since the Unix epoch. */
    pub from: i64,
    /* End of the queried time period, seconds since the Unix epoch. */
    pub to: i64,
    /* Query string. */
    pub query: String,
}

// SubmitDistributionPointsParams is a struct for passing parameters to the method [`SubmitDistributionPoints`]
#[derive(Clone, Debug)]
pub struct SubmitDistributionPointsParams {
    pub body: DistributionPointsPayload,
    /* HTTP header used to compress the media-type. */
    pub content_encoding: DistributionPointsContentEncoding,
}

// SubmitMetricsParams is a struct for passing parameters to the method [`SubmitMetrics`]
#[derive(Clone, Debug)]
pub struct SubmitMetricsParams {
    pub body: MetricsPayload,
    /* HTTP header used to compress the media-type. */
    pub content_encoding: MetricContentEncoding,
}

// UpdateMetricMetadataParams is a struct for passing parameters to the method [`UpdateMetricMetadata`]
#[derive(Clone, Debug)]
pub struct UpdateMetricMetadataParams {
    /* Name of the metric for which to edit metadata. */
    pub metric_name: String,
    /* New metadata. */
    pub body: MetricMetadata,
}




/// GetMetricMetadataError is a struct for typed errors of method [`GetMetricMetadata`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMetricMetadataError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListActiveMetricsError is a struct for typed errors of method [`ListActiveMetrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListActiveMetricsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListMetricsError is a struct for typed errors of method [`ListMetrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMetricsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// QueryMetricsError is a struct for typed errors of method [`QueryMetrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryMetricsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SubmitDistributionPointsError is a struct for typed errors of method [`SubmitDistributionPoints`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitDistributionPointsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status408(APIErrorResponse),
    Status413(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SubmitMetricsError is a struct for typed errors of method [`SubmitMetrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitMetricsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status408(APIErrorResponse),
    Status413(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateMetricMetadataError is a struct for typed errors of method [`UpdateMetricMetadata`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMetricMetadataError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}