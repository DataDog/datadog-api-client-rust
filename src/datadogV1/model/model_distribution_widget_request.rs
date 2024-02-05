// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Updated distribution widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistributionWidgetRequest {
    /// The log query.
    #[serde(rename = "apm_query")]
    pub apm_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// The APM stats query for table and distributions widgets.
    #[serde(rename = "apm_stats_query")]
    pub apm_stats_query: Option<crate::datadogV1::model::ApmStatsQueryDefinition>,
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
    /// Widget query.
    #[serde(rename = "q")]
    pub q: Option<String>,
    /// Query definition for Distribution Widget Histogram Request
    #[serde(rename = "query")]
    pub query: Option<crate::datadogV1::model::DistributionWidgetHistogramRequestQuery>,
    /// Request type for the histogram request.
    #[serde(rename = "request_type")]
    pub request_type: Option<crate::datadogV1::model::DistributionWidgetHistogramRequestType>,
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

impl DistributionWidgetRequest {
    pub fn new() -> DistributionWidgetRequest {
        DistributionWidgetRequest {
            apm_query: None,
            apm_stats_query: None,
            event_query: None,
            log_query: None,
            network_query: None,
            process_query: None,
            profile_metrics_query: None,
            q: None,
            query: None,
            request_type: None,
            rum_query: None,
            security_query: None,
            style: None,
        }
    }

    pub fn apm_query(&mut self, value: crate::datadogV1::model::LogQueryDefinition) -> &mut Self {
        self.apm_query = Some(value);
        self
    }

    pub fn apm_stats_query(
        &mut self,
        value: crate::datadogV1::model::ApmStatsQueryDefinition,
    ) -> &mut Self {
        self.apm_stats_query = Some(value);
        self
    }

    pub fn event_query(&mut self, value: crate::datadogV1::model::LogQueryDefinition) -> &mut Self {
        self.event_query = Some(value);
        self
    }

    pub fn log_query(&mut self, value: crate::datadogV1::model::LogQueryDefinition) -> &mut Self {
        self.log_query = Some(value);
        self
    }

    pub fn network_query(
        &mut self,
        value: crate::datadogV1::model::LogQueryDefinition,
    ) -> &mut Self {
        self.network_query = Some(value);
        self
    }

    pub fn process_query(
        &mut self,
        value: crate::datadogV1::model::ProcessQueryDefinition,
    ) -> &mut Self {
        self.process_query = Some(value);
        self
    }

    pub fn profile_metrics_query(
        &mut self,
        value: crate::datadogV1::model::LogQueryDefinition,
    ) -> &mut Self {
        self.profile_metrics_query = Some(value);
        self
    }

    pub fn q(&mut self, value: String) -> &mut Self {
        self.q = Some(value);
        self
    }

    pub fn query(
        &mut self,
        value: crate::datadogV1::model::DistributionWidgetHistogramRequestQuery,
    ) -> &mut Self {
        self.query = Some(value);
        self
    }

    pub fn request_type(
        &mut self,
        value: crate::datadogV1::model::DistributionWidgetHistogramRequestType,
    ) -> &mut Self {
        self.request_type = Some(value);
        self
    }

    pub fn rum_query(&mut self, value: crate::datadogV1::model::LogQueryDefinition) -> &mut Self {
        self.rum_query = Some(value);
        self
    }

    pub fn security_query(
        &mut self,
        value: crate::datadogV1::model::LogQueryDefinition,
    ) -> &mut Self {
        self.security_query = Some(value);
        self
    }

    pub fn style(&mut self, value: crate::datadogV1::model::WidgetStyle) -> &mut Self {
        self.style = Some(value);
        self
    }
}

impl Default for DistributionWidgetRequest {
    fn default() -> Self {
        Self::new()
    }
}
