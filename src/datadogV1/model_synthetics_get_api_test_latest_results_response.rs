// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsGetAPITestLatestResultsResponse {
    /// Timestamp of the latest API test run.
    #[serde(rename = "last_timestamp_fetched", skip_serializing_if = "Option::is_none")]
    pub last_timestamp_fetched: i64,
    /// Result of the latest API test run.
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Vec<SyntheticsAPITestResultShort>,
}

