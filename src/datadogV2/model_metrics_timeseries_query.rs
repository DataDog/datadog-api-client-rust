// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricsTimeseriesQuery {
    /// A data source that is powered by the Metrics platform.
    #[serde(rename = "data_source", skip_serializing_if = "Option::is_none")]
    pub data_source: MetricsDataSource,
    /// The variable name for use in formulas.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// A classic metrics query string.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: String,
}

