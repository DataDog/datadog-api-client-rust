// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A formula and functions APM dependency stats query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormulaAndFunctionApmDependencyStatsQueryDefinition {
    /// Data source for APM dependency stats queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::FormulaAndFunctionApmDependencyStatsDataSource,
    /// APM environment.
    #[serde(rename = "env")]
    pub env: String,
    /// Determines whether stats for upstream or downstream dependencies should be queried.
    #[serde(rename = "is_upstream")]
    pub is_upstream: Option<bool>,
    /// Name of query to use in formulas.
    #[serde(rename = "name")]
    pub name: String,
    /// Name of operation on service.
    #[serde(rename = "operation_name")]
    pub operation_name: String,
    /// The name of the second primary tag used within APM; required when `primary_tag_value` is specified. See <https://docs.datadoghq.com/tracing/guide/setting_primary_tags_to_scope/#add-a-second-primary-tag-in-datadog.>
    #[serde(rename = "primary_tag_name")]
    pub primary_tag_name: Option<String>,
    /// Filter APM data by the second primary tag. `primary_tag_name` must also be specified.
    #[serde(rename = "primary_tag_value")]
    pub primary_tag_value: Option<String>,
    /// APM resource.
    #[serde(rename = "resource_name")]
    pub resource_name: String,
    /// APM service.
    #[serde(rename = "service")]
    pub service: String,
    /// APM statistic.
    #[serde(rename = "stat")]
    pub stat: crate::datadogV1::model::FormulaAndFunctionApmDependencyStatName,
}

impl FormulaAndFunctionApmDependencyStatsQueryDefinition {
    pub fn new(
        data_source: crate::datadogV1::model::FormulaAndFunctionApmDependencyStatsDataSource,
        env: String,
        name: String,
        operation_name: String,
        resource_name: String,
        service: String,
        stat: crate::datadogV1::model::FormulaAndFunctionApmDependencyStatName,
    ) -> FormulaAndFunctionApmDependencyStatsQueryDefinition {
        FormulaAndFunctionApmDependencyStatsQueryDefinition {
            data_source,
            env,
            is_upstream: None,
            name,
            operation_name,
            primary_tag_name: None,
            primary_tag_value: None,
            resource_name,
            service,
            stat,
        }
    }

    pub fn is_upstream(&mut self, value: bool) -> &mut Self {
        self.is_upstream = Some(value);
        self
    }

    pub fn primary_tag_name(&mut self, value: String) -> &mut Self {
        self.primary_tag_name = Some(value);
        self
    }

    pub fn primary_tag_value(&mut self, value: String) -> &mut Self {
        self.primary_tag_value = Some(value);
        self
    }
}
