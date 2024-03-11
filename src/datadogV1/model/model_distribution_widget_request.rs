// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Updated distribution widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            _unparsed: false,
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

impl<'de> Deserialize<'de> for DistributionWidgetRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DistributionWidgetRequestVisitor;
        impl<'a> Visitor<'a> for DistributionWidgetRequestVisitor {
            type Value = DistributionWidgetRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut apm_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut apm_stats_query: Option<crate::datadogV1::model::ApmStatsQueryDefinition> =
                    None;
                let mut event_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut log_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut network_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut process_query: Option<crate::datadogV1::model::ProcessQueryDefinition> =
                    None;
                let mut profile_metrics_query: Option<crate::datadogV1::model::LogQueryDefinition> =
                    None;
                let mut q: Option<String> = None;
                let mut query: Option<
                    crate::datadogV1::model::DistributionWidgetHistogramRequestQuery,
                > = None;
                let mut request_type: Option<
                    crate::datadogV1::model::DistributionWidgetHistogramRequestType,
                > = None;
                let mut rum_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut security_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut style: Option<crate::datadogV1::model::WidgetStyle> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "apm_query" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "apm_stats_query" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_stats_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event_query" => {
                            if v.is_null() {
                                continue;
                            }
                            event_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "log_query" => {
                            if v.is_null() {
                                continue;
                            }
                            log_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "network_query" => {
                            if v.is_null() {
                                continue;
                            }
                            network_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "process_query" => {
                            if v.is_null() {
                                continue;
                            }
                            process_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "profile_metrics_query" => {
                            if v.is_null() {
                                continue;
                            }
                            profile_metrics_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "q" => {
                            if v.is_null() {
                                continue;
                            }
                            q = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _query) = query {
                                match _query {
                                    crate::datadogV1::model::DistributionWidgetHistogramRequestQuery::UnparsedObject(_query) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "request_type" => {
                            if v.is_null() {
                                continue;
                            }
                            request_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _request_type) = request_type {
                                match _request_type {
                                    crate::datadogV1::model::DistributionWidgetHistogramRequestType::UnparsedObject(_request_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "rum_query" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "security_query" => {
                            if v.is_null() {
                                continue;
                            }
                            security_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "style" => {
                            if v.is_null() {
                                continue;
                            }
                            style = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = DistributionWidgetRequest {
                    apm_query,
                    apm_stats_query,
                    event_query,
                    log_query,
                    network_query,
                    process_query,
                    profile_metrics_query,
                    q,
                    query,
                    request_type,
                    rum_query,
                    security_query,
                    style,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DistributionWidgetRequestVisitor)
    }
}
