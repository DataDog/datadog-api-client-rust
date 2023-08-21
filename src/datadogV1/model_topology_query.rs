// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopologyQuery {
    /// Name of the data source
    #[serde(rename = "data_source", skip_serializing_if = "Option::is_none")]
    pub data_source: TopologyQueryDataSource,
    /// Your environment and primary tag (or * if enabled for your account).
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Vec<String>,
    /// Name of the service
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: String,
}

