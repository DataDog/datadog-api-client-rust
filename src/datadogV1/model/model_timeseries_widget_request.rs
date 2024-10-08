// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Updated timeseries widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TimeseriesWidgetRequest {
    /// The log query.
    #[serde(rename = "apm_query")]
    pub apm_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// The log query.
    #[serde(rename = "audit_query")]
    pub audit_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// Type of display to use for the request.
    #[serde(rename = "display_type")]
    pub display_type: Option<crate::datadogV1::model::WidgetDisplayType>,
    /// The log query.
    #[serde(rename = "event_query")]
    pub event_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// List of formulas that operate on queries.
    #[serde(rename = "formulas")]
    pub formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>>,
    /// The log query.
    #[serde(rename = "log_query")]
    pub log_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// Used to define expression aliases.
    #[serde(rename = "metadata")]
    pub metadata: Option<Vec<crate::datadogV1::model::TimeseriesWidgetExpressionAlias>>,
    /// The log query.
    #[serde(rename = "network_query")]
    pub network_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// Whether or not to display a second y-axis on the right.
    #[serde(rename = "on_right_yaxis")]
    pub on_right_yaxis: Option<bool>,
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
    /// Define request widget style.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1::model::WidgetRequestStyle>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TimeseriesWidgetRequest {
    pub fn new() -> TimeseriesWidgetRequest {
        TimeseriesWidgetRequest {
            apm_query: None,
            audit_query: None,
            display_type: None,
            event_query: None,
            formulas: None,
            log_query: None,
            metadata: None,
            network_query: None,
            on_right_yaxis: None,
            process_query: None,
            profile_metrics_query: None,
            q: None,
            queries: None,
            response_format: None,
            rum_query: None,
            security_query: None,
            style: None,
            additional_properties: std::collections::BTreeMap::new(),
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

    pub fn display_type(mut self, value: crate::datadogV1::model::WidgetDisplayType) -> Self {
        self.display_type = Some(value);
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

    pub fn metadata(
        mut self,
        value: Vec<crate::datadogV1::model::TimeseriesWidgetExpressionAlias>,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn network_query(mut self, value: crate::datadogV1::model::LogQueryDefinition) -> Self {
        self.network_query = Some(value);
        self
    }

    pub fn on_right_yaxis(mut self, value: bool) -> Self {
        self.on_right_yaxis = Some(value);
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

    pub fn style(mut self, value: crate::datadogV1::model::WidgetRequestStyle) -> Self {
        self.style = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for TimeseriesWidgetRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TimeseriesWidgetRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeseriesWidgetRequestVisitor;
        impl<'a> Visitor<'a> for TimeseriesWidgetRequestVisitor {
            type Value = TimeseriesWidgetRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut apm_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut audit_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut display_type: Option<crate::datadogV1::model::WidgetDisplayType> = None;
                let mut event_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>> = None;
                let mut log_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut metadata: Option<
                    Vec<crate::datadogV1::model::TimeseriesWidgetExpressionAlias>,
                > = None;
                let mut network_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut on_right_yaxis: Option<bool> = None;
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
                let mut style: Option<crate::datadogV1::model::WidgetRequestStyle> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                        "display_type" => {
                            if v.is_null() {
                                continue;
                            }
                            display_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _display_type) = display_type {
                                match _display_type {
                                    crate::datadogV1::model::WidgetDisplayType::UnparsedObject(
                                        _display_type,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
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
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "network_query" => {
                            if v.is_null() {
                                continue;
                            }
                            network_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "on_right_yaxis" => {
                            if v.is_null() {
                                continue;
                            }
                            on_right_yaxis =
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TimeseriesWidgetRequest {
                    apm_query,
                    audit_query,
                    display_type,
                    event_query,
                    formulas,
                    log_query,
                    metadata,
                    network_query,
                    on_right_yaxis,
                    process_query,
                    profile_metrics_query,
                    q,
                    queries,
                    response_format,
                    rum_query,
                    security_query,
                    style,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeseriesWidgetRequestVisitor)
    }
}
