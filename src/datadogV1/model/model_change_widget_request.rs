// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Updated change widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChangeWidgetRequest {
    /// The log query.
    #[serde(rename = "apm_query")]
    pub apm_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// Show the absolute or the relative change.
    #[serde(rename = "change_type")]
    pub change_type: Option<crate::datadogV1::model::WidgetChangeType>,
    /// Timeframe used for the change comparison.
    #[serde(rename = "compare_to")]
    pub compare_to: Option<crate::datadogV1::model::WidgetCompareTo>,
    /// The log query.
    #[serde(rename = "event_query")]
    pub event_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// List of formulas that operate on queries.
    #[serde(rename = "formulas")]
    pub formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>>,
    /// Whether to show increase as good.
    #[serde(rename = "increase_good")]
    pub increase_good: Option<bool>,
    /// The log query.
    #[serde(rename = "log_query")]
    pub log_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// The log query.
    #[serde(rename = "network_query")]
    pub network_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// What to order by.
    #[serde(rename = "order_by")]
    pub order_by: Option<crate::datadogV1::model::WidgetOrderBy>,
    /// Widget sorting methods.
    #[serde(rename = "order_dir")]
    pub order_dir: Option<crate::datadogV1::model::WidgetSort>,
    /// The process query to use in the widget.
    #[serde(rename = "process_query")]
    pub process_query: Option<crate::datadogV1::model::ProcessQueryDefinition>,
    /// The log query.
    #[serde(rename = "profile_metrics_query")]
    pub profile_metrics_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// Query definition.
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
    /// Whether to show the present value.
    #[serde(rename = "show_present")]
    pub show_present: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ChangeWidgetRequest {
    pub fn new() -> ChangeWidgetRequest {
        ChangeWidgetRequest {
            apm_query: None,
            change_type: None,
            compare_to: None,
            event_query: None,
            formulas: None,
            increase_good: None,
            log_query: None,
            network_query: None,
            order_by: None,
            order_dir: None,
            process_query: None,
            profile_metrics_query: None,
            q: None,
            queries: None,
            response_format: None,
            rum_query: None,
            security_query: None,
            show_present: None,
            _unparsed: false,
        }
    }

    pub fn apm_query(&mut self, value: crate::datadogV1::model::LogQueryDefinition) -> &mut Self {
        self.apm_query = Some(value);
        self
    }

    pub fn change_type(&mut self, value: crate::datadogV1::model::WidgetChangeType) -> &mut Self {
        self.change_type = Some(value);
        self
    }

    pub fn compare_to(&mut self, value: crate::datadogV1::model::WidgetCompareTo) -> &mut Self {
        self.compare_to = Some(value);
        self
    }

    pub fn event_query(&mut self, value: crate::datadogV1::model::LogQueryDefinition) -> &mut Self {
        self.event_query = Some(value);
        self
    }

    pub fn formulas(&mut self, value: Vec<crate::datadogV1::model::WidgetFormula>) -> &mut Self {
        self.formulas = Some(value);
        self
    }

    pub fn increase_good(&mut self, value: bool) -> &mut Self {
        self.increase_good = Some(value);
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

    pub fn order_by(&mut self, value: crate::datadogV1::model::WidgetOrderBy) -> &mut Self {
        self.order_by = Some(value);
        self
    }

    pub fn order_dir(&mut self, value: crate::datadogV1::model::WidgetSort) -> &mut Self {
        self.order_dir = Some(value);
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

    pub fn queries(
        &mut self,
        value: Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>,
    ) -> &mut Self {
        self.queries = Some(value);
        self
    }

    pub fn response_format(
        &mut self,
        value: crate::datadogV1::model::FormulaAndFunctionResponseFormat,
    ) -> &mut Self {
        self.response_format = Some(value);
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

    pub fn show_present(&mut self, value: bool) -> &mut Self {
        self.show_present = Some(value);
        self
    }
}

impl Default for ChangeWidgetRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ChangeWidgetRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChangeWidgetRequestVisitor;
        impl<'a> Visitor<'a> for ChangeWidgetRequestVisitor {
            type Value = ChangeWidgetRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut apm_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut change_type: Option<crate::datadogV1::model::WidgetChangeType> = None;
                let mut compare_to: Option<crate::datadogV1::model::WidgetCompareTo> = None;
                let mut event_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>> = None;
                let mut increase_good: Option<bool> = None;
                let mut log_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut network_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut order_by: Option<crate::datadogV1::model::WidgetOrderBy> = None;
                let mut order_dir: Option<crate::datadogV1::model::WidgetSort> = None;
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
                let mut show_present: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "apm_query" => {
                            if v.is_null() {
                                continue;
                            }
                            apm_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "change_type" => {
                            if v.is_null() {
                                continue;
                            }
                            change_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _change_type) = change_type {
                                match _change_type {
                                    crate::datadogV1::model::WidgetChangeType::UnparsedObject(
                                        _change_type,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "compare_to" => {
                            if v.is_null() {
                                continue;
                            }
                            compare_to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _compare_to) = compare_to {
                                match _compare_to {
                                    crate::datadogV1::model::WidgetCompareTo::UnparsedObject(
                                        _compare_to,
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
                        "increase_good" => {
                            if v.is_null() {
                                continue;
                            }
                            increase_good =
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
                        "order_by" => {
                            if v.is_null() {
                                continue;
                            }
                            order_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _order_by) = order_by {
                                match _order_by {
                                    crate::datadogV1::model::WidgetOrderBy::UnparsedObject(
                                        _order_by,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "order_dir" => {
                            if v.is_null() {
                                continue;
                            }
                            order_dir = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _order_dir) = order_dir {
                                match _order_dir {
                                    crate::datadogV1::model::WidgetSort::UnparsedObject(
                                        _order_dir,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
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
                        "show_present" => {
                            if v.is_null() {
                                continue;
                            }
                            show_present =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ChangeWidgetRequest {
                    apm_query,
                    change_type,
                    compare_to,
                    event_query,
                    formulas,
                    increase_good,
                    log_query,
                    network_query,
                    order_by,
                    order_dir,
                    process_query,
                    profile_metrics_query,
                    q,
                    queries,
                    response_format,
                    rum_query,
                    security_query,
                    show_present,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ChangeWidgetRequestVisitor)
    }
}
