// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormulaAndFunctionApmResourceStatsQueryDefinition {
    /// Data source for APM resource stats queries.
    #[serde(rename = "data_source", skip_serializing_if = "Option::is_none")]
    pub data_source: FormulaAndFunctionApmResourceStatsDataSource,
    /// APM environment.
    #[serde(rename = "env", skip_serializing_if = "Option::is_none")]
    pub env: String,
    /// Array of fields to group results by.
    #[serde(rename = "group_by", skip_serializing_if = "Option::is_none")]
    pub group_by: Vec<String>,
    /// Name of this query to use in formulas.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Name of operation on service.
    #[serde(rename = "operation_name", skip_serializing_if = "Option::is_none")]
    pub operation_name: String,
    /// Name of the second primary tag used within APM. Required when `primary_tag_value` is specified. See https://docs.datadoghq.com/tracing/guide/setting_primary_tags_to_scope/#add-a-second-primary-tag-in-datadog
    #[serde(rename = "primary_tag_name", skip_serializing_if = "Option::is_none")]
    pub primary_tag_name: String,
    /// Value of the second primary tag by which to filter APM data. `primary_tag_name` must also be specified.
    #[serde(rename = "primary_tag_value", skip_serializing_if = "Option::is_none")]
    pub primary_tag_value: String,
    /// APM resource name.
    #[serde(rename = "resource_name", skip_serializing_if = "Option::is_none")]
    pub resource_name: String,
    /// APM service name.
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: String,
    /// APM resource stat name.
    #[serde(rename = "stat", skip_serializing_if = "Option::is_none")]
    pub stat: FormulaAndFunctionApmResourceStatName,
}

