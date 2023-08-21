// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApmStatsQueryDefinition {
    /// Column properties used by the front end for display.
    #[serde(rename = "columns", skip_serializing_if = "Option::is_none")]
    pub columns: Vec<ApmStatsQueryColumnType>,
    /// Environment name.
    #[serde(rename = "env", skip_serializing_if = "Option::is_none")]
    pub env: String,
    /// Operation name associated with service.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// The organization's host group name and value.
    #[serde(rename = "primary_tag", skip_serializing_if = "Option::is_none")]
    pub primary_tag: String,
    /// Resource name.
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: String,
    /// The level of detail for the request.
    #[serde(rename = "row_type", skip_serializing_if = "Option::is_none")]
    pub row_type: ApmStatsQueryRowType,
    /// Service name.
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: String,
}

