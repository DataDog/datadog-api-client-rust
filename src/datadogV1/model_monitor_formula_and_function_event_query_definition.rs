// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorFormulaAndFunctionEventQueryDefinition {
    /// Compute options.
    #[serde(rename = "compute")]
    pub compute: MonitorFormulaAndFunctionEventQueryDefinitionCompute,
    /// Data source for event platform-based queries.
    #[serde(rename = "data_source", skip_serializing_if = "Option::is_none")]
    pub data_source: MonitorFormulaAndFunctionEventsDataSource,
    /// Group by options.
    #[serde(rename = "group_by", skip_serializing_if = "Option::is_none")]
    pub group_by: Vec<MonitorFormulaAndFunctionEventQueryGroupBy>,
    /// An array of index names to query in the stream. Omit or use `[]` to query all indexes at once.
    #[serde(rename = "indexes", skip_serializing_if = "Option::is_none")]
    pub indexes: Vec<String>,
    /// Name of the query for use in formulas.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Search options.
    #[serde(rename = "search")]
    pub search: MonitorFormulaAndFunctionEventQueryDefinitionSearch,
}

