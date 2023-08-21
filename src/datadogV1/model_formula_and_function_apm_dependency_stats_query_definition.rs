// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormulaAndFunctionApmDependencyStatsQueryDefinition {
    /// Data source for APM dependency stats queries.
    #[serde(rename = "data_source", skip_serializing_if = "Option::is_none")]
    pub data_source: FormulaAndFunctionApmDependencyStatsDataSource,
    /// APM environment.
    #[serde(rename = "env", skip_serializing_if = "Option::is_none")]
    pub env: String,
    /// Determines whether stats for upstream or downstream dependencies should be queried.
    #[serde(rename = "is_upstream", skip_serializing_if = "Option::is_none")]
    pub is_upstream: bool,
    /// Name of query to use in formulas.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Name of operation on service.
    #[serde(rename = "operation_name", skip_serializing_if = "Option::is_none")]
    pub operation_name: String,
    /// The name of the second primary tag used within APM; required when `primary_tag_value` is specified. See https://docs.datadoghq.com/tracing/guide/setting_primary_tags_to_scope/#add-a-second-primary-tag-in-datadog.
    #[serde(rename = "primary_tag_name", skip_serializing_if = "Option::is_none")]
    pub primary_tag_name: String,
    /// Filter APM data by the second primary tag. `primary_tag_name` must also be specified.
    #[serde(rename = "primary_tag_value", skip_serializing_if = "Option::is_none")]
    pub primary_tag_value: String,
    /// APM resource.
    #[serde(rename = "resource_name", skip_serializing_if = "Option::is_none")]
    pub resource_name: String,
    /// APM service.
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: String,
    /// APM statistic.
    #[serde(rename = "stat", skip_serializing_if = "Option::is_none")]
    pub stat: FormulaAndFunctionApmDependencyStatName,
}

