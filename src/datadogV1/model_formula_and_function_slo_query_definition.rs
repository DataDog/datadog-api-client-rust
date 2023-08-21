// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormulaAndFunctionSLOQueryDefinition {
    /// Additional filters applied to the SLO query.
    #[serde(rename = "additional_query_filters", skip_serializing_if = "Option::is_none")]
    pub additional_query_filters: String,
    /// Data source for SLO measures queries.
    #[serde(rename = "data_source", skip_serializing_if = "Option::is_none")]
    pub data_source: FormulaAndFunctionSLODataSource,
    /// Group mode to query measures.
    #[serde(rename = "group_mode", skip_serializing_if = "Option::is_none")]
    pub group_mode: FormulaAndFunctionSLOGroupMode,
    /// SLO measures queries.
    #[serde(rename = "measure", skip_serializing_if = "Option::is_none")]
    pub measure: FormulaAndFunctionSLOMeasure,
    /// Name of the query for use in formulas.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// ID of an SLO to query measures.
    #[serde(rename = "slo_id", skip_serializing_if = "Option::is_none")]
    pub slo_id: String,
    /// Name of the query for use in formulas.
    #[serde(rename = "slo_query_type", skip_serializing_if = "Option::is_none")]
    pub slo_query_type: FormulaAndFunctionSLOQueryType,
}

