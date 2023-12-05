// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response Object that includes your query and the list of metrics retrieved.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MetricsQueryResponse {
    /// Message indicating the errors if status is not `ok`.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Start of requested time window, milliseconds since Unix epoch.
    #[serde(rename = "from_date")]
    pub from_date: Option<i64>,
    /// List of tag keys on which to group.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<String>>,
    /// Message indicating `success` if status is `ok`.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// Query string
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Type of response.
    #[serde(rename = "res_type")]
    pub res_type: Option<String>,
    /// List of timeseries queried.
    #[serde(rename = "series")]
    pub series: Option<Vec<crate::datadogV1::model::MetricsQueryMetadata>>,
    /// Status of the query.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// End of requested time window, milliseconds since Unix epoch.
    #[serde(rename = "to_date")]
    pub to_date: Option<i64>,
}

impl MetricsQueryResponse {
    pub fn new() -> MetricsQueryResponse {
        MetricsQueryResponse {
            error: None,
            from_date: None,
            group_by: None,
            message: None,
            query: None,
            res_type: None,
            series: None,
            status: None,
            to_date: None,
        }
    }
}