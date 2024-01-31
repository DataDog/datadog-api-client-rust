// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Updated scatter plot.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScatterPlotRequest {
    /// Aggregator used for the request.
    #[serde(rename = "aggregator")]
    pub aggregator: Option<crate::datadogV1::model::ScatterplotWidgetAggregator>,
    /// The log query.
    #[serde(rename = "apm_query")]
    pub apm_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// The log query.
    #[serde(rename = "event_query")]
    pub event_query: Option<crate::datadogV1::model::LogQueryDefinition>,
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
    /// Query definition.
    #[serde(rename = "q")]
    pub q: Option<String>,
    /// The log query.
    #[serde(rename = "rum_query")]
    pub rum_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// The log query.
    #[serde(rename = "security_query")]
    pub security_query: Option<crate::datadogV1::model::LogQueryDefinition>,
}

impl ScatterPlotRequest {
    pub fn new() -> ScatterPlotRequest {
        ScatterPlotRequest {
            aggregator: None,
            apm_query: None,
            event_query: None,
            log_query: None,
            network_query: None,
            process_query: None,
            profile_metrics_query: None,
            q: None,
            rum_query: None,
            security_query: None,
        }
    }

    pub fn with_aggregator(
        &mut self,
        value: crate::datadogV1::model::ScatterplotWidgetAggregator,
    ) -> &mut Self {
        self.aggregator = Some(value);
        self
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
        value: crate::datadogV1::model::LogQueryDefinition,
    ) -> &mut Self {
        self.event_query = Some(value);
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
}
impl Default for ScatterPlotRequest {
    fn default() -> Self {
        Self::new()
    }
}
