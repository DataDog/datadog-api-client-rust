// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunnelQuery {
    /// Source from which to query items to display in the funnel.
    #[serde(rename = "data_source", skip_serializing_if = "Option::is_none")]
    pub data_source: FunnelSource,
    /// The widget query.
    #[serde(rename = "query_string", skip_serializing_if = "Option::is_none")]
    pub query_string: String,
    /// List of funnel steps.
    #[serde(rename = "steps", skip_serializing_if = "Option::is_none")]
    pub steps: Vec<FunnelStep>,
}

