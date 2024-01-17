// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An updated geomap widget.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeomapWidgetRequest {
    /// Widget columns.
    #[serde(rename = "columns")]
    pub columns: Option<Vec<crate::datadogV1::model::ListStreamColumn>>,
    /// List of formulas that operate on queries.
    #[serde(rename = "formulas")]
    pub formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>>,
    /// The log query.
    #[serde(rename = "log_query")]
    pub log_query: Option<Box<crate::datadogV1::model::LogQueryDefinition>>,
    /// The widget metrics query.
    #[serde(rename = "q")]
    pub q: Option<String>,
    /// List of queries that can be returned directly or used in formulas.
    #[serde(rename = "queries")]
    pub queries: Option<Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>>,
    /// Updated list stream widget.
    #[serde(rename = "query")]
    pub query: Option<Box<crate::datadogV1::model::ListStreamQuery>>,
    /// Timeseries, scalar, or event list response. Event list response formats are supported by Geomap widgets.
    #[serde(rename = "response_format")]
    pub response_format: Option<crate::datadogV1::model::FormulaAndFunctionResponseFormat>,
    /// The log query.
    #[serde(rename = "rum_query")]
    pub rum_query: Option<Box<crate::datadogV1::model::LogQueryDefinition>>,
    /// The log query.
    #[serde(rename = "security_query")]
    pub security_query: Option<Box<crate::datadogV1::model::LogQueryDefinition>>,
}

impl GeomapWidgetRequest {
    pub fn new() -> GeomapWidgetRequest {
        GeomapWidgetRequest {
            columns: None,
            formulas: None,
            log_query: None,
            q: None,
            queries: None,
            query: None,
            response_format: None,
            rum_query: None,
            security_query: None,
        }
    }
}
impl Default for GeomapWidgetRequest {
    fn default() -> Self {
        Self::new()
    }
}
