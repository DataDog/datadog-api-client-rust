// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// APM resource stats query using formulas and functions.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormulaAndFunctionApmResourceStatsQueryDefinition {
    /// Data source for APM resource stats queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::FormulaAndFunctionApmResourceStatsDataSource,
    /// APM environment.
    #[serde(rename = "env")]
    pub env: String,
    /// Array of fields to group results by.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<String>>,
    /// Name of this query to use in formulas.
    #[serde(rename = "name")]
    pub name: String,
    /// Name of operation on service.
    #[serde(rename = "operation_name")]
    pub operation_name: Option<String>,
    /// Name of the second primary tag used within APM. Required when `primary_tag_value` is specified. See <https://docs.datadoghq.com/tracing/guide/setting_primary_tags_to_scope/#add-a-second-primary-tag-in-datadog>
    #[serde(rename = "primary_tag_name")]
    pub primary_tag_name: Option<String>,
    /// Value of the second primary tag by which to filter APM data. `primary_tag_name` must also be specified.
    #[serde(rename = "primary_tag_value")]
    pub primary_tag_value: Option<String>,
    /// APM resource name.
    #[serde(rename = "resource_name")]
    pub resource_name: Option<String>,
    /// APM service name.
    #[serde(rename = "service")]
    pub service: String,
    /// APM resource stat name.
    #[serde(rename = "stat")]
    pub stat: crate::datadogV1::model::FormulaAndFunctionApmResourceStatName,
}

impl FormulaAndFunctionApmResourceStatsQueryDefinition {
    pub fn new(
        data_source: crate::datadogV1::model::FormulaAndFunctionApmResourceStatsDataSource,
        env: String,
        name: String,
        service: String,
        stat: crate::datadogV1::model::FormulaAndFunctionApmResourceStatName,
    ) -> FormulaAndFunctionApmResourceStatsQueryDefinition {
        FormulaAndFunctionApmResourceStatsQueryDefinition {
            data_source,
            env,
            group_by: None,
            name,
            operation_name: None,
            primary_tag_name: None,
            primary_tag_value: None,
            resource_name: None,
            service,
            stat,
        }
    }
}
