// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Updated heat map widget.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HeatMapWidgetRequest {
    /// The log query.
    #[serde(rename = "apm_query")]
    pub apm_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// The event query.
    #[serde(rename = "event_query")]
    pub event_query: Option<crate::datadogV1::model::EventQueryDefinition>,
    /// List of formulas that operate on queries.
    #[serde(rename = "formulas")]
    pub formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>>,
    /// The log query.
    #[serde(rename = "log_query")]
    pub log_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// The log query.
    #[serde(rename = "network_query")]
    pub network_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// The process query to use in the widget.
    #[serde(rename = "process_query")]
    pub process_query: Option<crate::datadogV1::model::ProcessQueryDefinition>,
    /// The log query.
    #[serde(rename = "profile_metrics_query")]
    pub profile_metrics_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// Widget query.
    #[serde(rename = "q")]
    pub q: Option<String>,
    /// List of queries that can be returned directly or used in formulas.
    #[serde(rename = "queries")]
    pub queries: Option<Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>>,
    /// Timeseries, scalar, or event list response. Event list response formats are supported by Geomap widgets.
    #[serde(rename = "response_format")]
    pub response_format: Option<crate::datadogV1::model::FormulaAndFunctionResponseFormat>,
    /// The log query.
    #[serde(rename = "rum_query")]
    pub rum_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// The log query.
    #[serde(rename = "security_query")]
    pub security_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// Widget style definition.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1::model::WidgetStyle>,
}

impl HeatMapWidgetRequest {
    pub fn new() -> HeatMapWidgetRequest {
        HeatMapWidgetRequest {
            apm_query: None,
            event_query: None,
            formulas: None,
            log_query: None,
            network_query: None,
            process_query: None,
            profile_metrics_query: None,
            q: None,
            queries: None,
            response_format: None,
            rum_query: None,
            security_query: None,
            style: None,
        }
    }

    pub fn with_apm_query(
        &mut self,
        value: crate::datadogV1::model::LogQueryDefinition,
    ) -> &mut Self {
        self.apm_query = Some(value);
        self
    }

    pub fn with_event_query(
        &mut self,
        value: crate::datadogV1::model::EventQueryDefinition,
    ) -> &mut Self {
        self.event_query = Some(value);
        self
    }

    pub fn with_formulas(
        &mut self,
        value: Vec<crate::datadogV1::model::WidgetFormula>,
    ) -> &mut Self {
        self.formulas = Some(value);
        self
    }

    pub fn with_log_query(
        &mut self,
        value: crate::datadogV1::model::LogQueryDefinition,
    ) -> &mut Self {
        self.log_query = Some(value);
        self
    }

    pub fn with_network_query(
        &mut self,
        value: crate::datadogV1::model::LogQueryDefinition,
    ) -> &mut Self {
        self.network_query = Some(value);
        self
    }

    pub fn with_process_query(
        &mut self,
        value: crate::datadogV1::model::ProcessQueryDefinition,
    ) -> &mut Self {
        self.process_query = Some(value);
        self
    }

    pub fn with_profile_metrics_query(
        &mut self,
        value: crate::datadogV1::model::LogQueryDefinition,
    ) -> &mut Self {
        self.profile_metrics_query = Some(value);
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

    pub fn with_rum_query(
        &mut self,
        value: crate::datadogV1::model::LogQueryDefinition,
    ) -> &mut Self {
        self.rum_query = Some(value);
        self
    }

    pub fn with_security_query(
        &mut self,
        value: crate::datadogV1::model::LogQueryDefinition,
    ) -> &mut Self {
        self.security_query = Some(value);
        self
    }

    pub fn with_style(&mut self, value: crate::datadogV1::model::WidgetStyle) -> &mut Self {
        self.style = Some(value);
        self
    }
}
impl Default for HeatMapWidgetRequest {
    fn default() -> Self {
        Self::new()
    }
}
