// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricsQueryResponse {
    /// Message indicating the errors if status is not `ok`.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: String,
    /// Start of requested time window, milliseconds since Unix epoch.
    #[serde(rename = "from_date", skip_serializing_if = "Option::is_none")]
    pub from_date: i64,
    /// List of tag keys on which to group.
    #[serde(rename = "group_by", skip_serializing_if = "Option::is_none")]
    pub group_by: Vec<String>,
    /// Message indicating `success` if status is `ok`.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: String,
    /// Query string
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: String,
    /// Type of response.
    #[serde(rename = "res_type", skip_serializing_if = "Option::is_none")]
    pub res_type: String,
    /// List of timeseries queried.
    #[serde(rename = "series", skip_serializing_if = "Option::is_none")]
    pub series: Vec<MetricsQueryMetadata>,
    /// Status of the query.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: String,
    /// End of requested time window, milliseconds since Unix epoch.
    #[serde(rename = "to_date", skip_serializing_if = "Option::is_none")]
    pub to_date: i64,
}

