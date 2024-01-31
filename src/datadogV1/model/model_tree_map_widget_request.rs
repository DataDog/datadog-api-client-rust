// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An updated treemap widget.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TreeMapWidgetRequest {
    /// List of formulas that operate on queries.
    #[serde(rename = "formulas")]
    pub formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>>,
    /// The widget metrics query.
    #[serde(rename = "q")]
    pub q: Option<String>,
    /// List of queries that can be returned directly or used in formulas.
    #[serde(rename = "queries")]
    pub queries: Option<Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>>,
    /// Timeseries, scalar, or event list response. Event list response formats are supported by Geomap widgets.
    #[serde(rename = "response_format")]
    pub response_format: Option<crate::datadogV1::model::FormulaAndFunctionResponseFormat>,
}

impl TreeMapWidgetRequest {
    pub fn new() -> TreeMapWidgetRequest {
        TreeMapWidgetRequest {
            formulas: None,
            q: None,
            queries: None,
            response_format: None,
        }
    }

    pub fn with_formulas(
        &mut self,
        value: Vec<crate::datadogV1::model::WidgetFormula>,
    ) -> &mut Self {
        self.formulas = Some(value);
        self
    }

    pub fn with_q(&mut self, value: String) -> &mut Self {
        self.q = Some(value);
        self
    }

    pub fn with_queries(
        &mut self,
        value: Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>,
    ) -> &mut Self {
        self.queries = Some(value);
        self
    }

    pub fn with_response_format(
        &mut self,
        value: crate::datadogV1::model::FormulaAndFunctionResponseFormat,
    ) -> &mut Self {
        self.response_format = Some(value);
        self
    }
}
impl Default for TreeMapWidgetRequest {
    fn default() -> Self {
        Self::new()
    }
}
