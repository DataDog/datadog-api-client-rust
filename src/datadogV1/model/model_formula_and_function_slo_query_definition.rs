// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A formula and functions metrics query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormulaAndFunctionSLOQueryDefinition {
    /// Additional filters applied to the SLO query.
    #[serde(rename = "additional_query_filters")]
    pub additional_query_filters: Option<String>,
    /// Data source for SLO measures queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::FormulaAndFunctionSLODataSource,
    /// Group mode to query measures.
    #[serde(rename = "group_mode")]
    pub group_mode: Option<crate::datadogV1::model::FormulaAndFunctionSLOGroupMode>,
    /// SLO measures queries.
    #[serde(rename = "measure")]
    pub measure: crate::datadogV1::model::FormulaAndFunctionSLOMeasure,
    /// Name of the query for use in formulas.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// ID of an SLO to query measures.
    #[serde(rename = "slo_id")]
    pub slo_id: String,
    /// Name of the query for use in formulas.
    #[serde(rename = "slo_query_type")]
    pub slo_query_type: Option<crate::datadogV1::model::FormulaAndFunctionSLOQueryType>,
}

impl FormulaAndFunctionSLOQueryDefinition {
    pub fn new(
        data_source: crate::datadogV1::model::FormulaAndFunctionSLODataSource,
        measure: crate::datadogV1::model::FormulaAndFunctionSLOMeasure,
        slo_id: String,
    ) -> FormulaAndFunctionSLOQueryDefinition {
        FormulaAndFunctionSLOQueryDefinition {
            additional_query_filters: None,
            data_source,
            group_mode: None,
            measure,
            name: None,
            slo_id,
            slo_query_type: None,
        }
    }

    pub fn additional_query_filters(&mut self, value: String) -> &mut Self {
        self.additional_query_filters = Some(value);
        self
    }

    pub fn group_mode(
        &mut self,
        value: crate::datadogV1::model::FormulaAndFunctionSLOGroupMode,
    ) -> &mut Self {
        self.group_mode = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn slo_query_type(
        &mut self,
        value: crate::datadogV1::model::FormulaAndFunctionSLOQueryType,
    ) -> &mut Self {
        self.slo_query_type = Some(value);
        self
    }
}
