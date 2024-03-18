// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Request definition of sunburst widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SunburstWidgetRequest {
    /// The log query.
    #[serde(rename = "apm_query")]
    pub apm_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// The log query.
    #[serde(rename = "audit_query")]
    pub audit_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// The log query.
    #[serde(rename = "event_query")]
    pub event_query: Option<crate::datadogV1::model::LogQueryDefinition>,
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
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SunburstWidgetRequest {
    pub fn new() -> SunburstWidgetRequest {
        SunburstWidgetRequest {
            apm_query: None,
            audit_query: None,
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
            _unparsed: false,
        }
    }

    pub fn apm_query(mut self, value: crate::datadogV1::model::LogQueryDefinition) -> Self {
        self.apm_query = Some(value);
        self
    }

    pub fn audit_query(mut self, value: crate::datadogV1::model::LogQueryDefinition) -> Self {
        self.audit_query = Some(value);
        self
    }

    pub fn event_query(mut self, value: crate::datadogV1::model::LogQueryDefinition) -> Self {
        self.event_query = Some(value);
        self
    }

    pub fn formulas(mut self, value: Vec<crate::datadogV1::model::WidgetFormula>) -> Self {
        self.formulas = Some(value);
        self
    }

    pub fn log_query(mut self, value: crate::datadogV1::model::LogQueryDefinition) -> Self {
        self.log_query = Some(value);
        self
    }

    pub fn network_query(mut self, value: crate::datadogV1::model::LogQueryDefinition) -> Self {
        self.network_query = Some(value);
        self
    }

    pub fn process_query(mut self, value: crate::datadogV1::model::ProcessQueryDefinition) -> Self {
        self.process_query = Some(value);
        self
    }

    pub fn profile_metrics_query(
        mut self,
        value: crate::datadogV1::model::LogQueryDefinition,
    ) -> Self {
        self.profile_metrics_query = Some(value);
        self
    }

    pub fn q(mut self, value: String) -> Self {
        self.q = Some(value);
        self
    }

    pub fn queries(
        mut self,
        value: Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>,
    ) -> Self {
        self.queries = Some(value);
        self
    }

    pub fn response_format(
        mut self,
        value: crate::datadogV1::model::FormulaAndFunctionResponseFormat,
    ) -> Self {
        self.response_format = Some(value);
        self
    }

    pub fn rum_query(mut self, value: crate::datadogV1::model::LogQueryDefinition) -> Self {
        self.rum_query = Some(value);
        self
    }

    pub fn security_query(mut self, value: crate::datadogV1::model::LogQueryDefinition) -> Self {
        self.security_query = Some(value);
        self
    }

    pub fn style(mut self, value: crate::datadogV1::model::WidgetStyle) -> Self {
        self.style = Some(value);
        self
    }
}

impl Default for SunburstWidgetRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SunburstWidgetRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SunburstWidgetRequestVisitor;
        impl<'a> Visitor<'a> for SunburstWidgetRequestVisitor {
            type Value = SunburstWidgetRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut apm_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut audit_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut event_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>> = None;
                let mut log_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut network_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut process_query: Option<crate::datadogV1::model::ProcessQueryDefinition> =
                    None;
                let mut profile_metrics_query: Option<crate::datadogV1::model::LogQueryDefinition> =
                    None;
                let mut q: Option<String> = None;
                let mut queries: Option<
                    Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>,
                > = None;
                let mut response_format: Option<
                    crate::datadogV1::model::FormulaAndFunctionResponseFormat,
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
                        "audit_query" => {
                            if v.is_null() {
                                continue;
                            }
                            audit_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event_query" => {
                            if v.is_null() {
                                continue;
                            }
                            event_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "formulas" => {
                            if v.is_null() {
                                continue;
                            }
                            formulas = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "queries" => {
                            if v.is_null() {
                                continue;
                            }
                            queries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "response_format" => {
                            if v.is_null() {
                                continue;
                            }
                            response_format =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _response_format) = response_format {
                                match _response_format {
                                    crate::datadogV1::model::FormulaAndFunctionResponseFormat::UnparsedObject(_response_format) => {
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

                let content = SunburstWidgetRequest {
                    apm_query,
                    audit_query,
                    event_query,
                    formulas,
                    log_query,
                    network_query,
                    process_query,
                    profile_metrics_query,
                    q,
                    queries,
                    response_format,
                    rum_query,
                    security_query,
                    style,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SunburstWidgetRequestVisitor)
    }
}
