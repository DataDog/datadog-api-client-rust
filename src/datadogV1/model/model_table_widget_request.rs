// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Updated table widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TableWidgetRequest {
    /// Aggregator used for the request.
    #[serde(rename = "aggregator")]
    pub aggregator: Option<crate::datadogV1::model::WidgetAggregator>,
    /// The column name (defaults to the metric name).
    #[serde(rename = "alias")]
    pub alias: Option<String>,
    /// The log query.
    #[serde(rename = "apm_query")]
    pub apm_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// The APM stats query for table and distributions widgets.
    #[serde(rename = "apm_stats_query")]
    pub apm_stats_query: Option<crate::datadogV1::model::ApmStatsQueryDefinition>,
    /// A list of display modes for each table cell.
    #[serde(rename = "cell_display_mode")]
    pub cell_display_mode: Option<Vec<crate::datadogV1::model::TableWidgetCellDisplayMode>>,
    /// List of conditional formats.
    #[serde(rename = "conditional_formats")]
    pub conditional_formats: Option<Vec<crate::datadogV1::model::WidgetConditionalFormat>>,
    /// The log query.
    #[serde(rename = "event_query")]
    pub event_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// List of formulas that operate on queries.
    #[serde(rename = "formulas")]
    pub formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>>,
    /// For metric queries, the number of lines to show in the table. Only one request should have this property.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// The log query.
    #[serde(rename = "log_query")]
    pub log_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// The log query.
    #[serde(rename = "network_query")]
    pub network_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// Widget sorting methods.
    #[serde(rename = "order")]
    pub order: Option<crate::datadogV1::model::WidgetSort>,
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
    /// The controls for sorting the widget.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1::model::WidgetSortBy>,
    /// List of text formats for columns produced by tags.
    #[serde(rename = "text_formats")]
    pub text_formats: Option<Vec<Vec<crate::datadogV1::model::TableWidgetTextFormatRule>>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TableWidgetRequest {
    pub fn new() -> TableWidgetRequest {
        TableWidgetRequest {
            aggregator: None,
            alias: None,
            apm_query: None,
            apm_stats_query: None,
            cell_display_mode: None,
            conditional_formats: None,
            event_query: None,
            formulas: None,
            limit: None,
            log_query: None,
            network_query: None,
            order: None,
            process_query: None,
            profile_metrics_query: None,
            q: None,
            queries: None,
            response_format: None,
            rum_query: None,
            security_query: None,
            sort: None,
            text_formats: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn aggregator(mut self, value: crate::datadogV1::model::WidgetAggregator) -> Self {
        self.aggregator = Some(value);
        self
    }

    pub fn alias(mut self, value: String) -> Self {
        self.alias = Some(value);
        self
    }

    pub fn apm_query(mut self, value: crate::datadogV1::model::LogQueryDefinition) -> Self {
        self.apm_query = Some(value);
        self
    }

    pub fn apm_stats_query(
        mut self,
        value: crate::datadogV1::model::ApmStatsQueryDefinition,
    ) -> Self {
        self.apm_stats_query = Some(value);
        self
    }

    pub fn cell_display_mode(
        mut self,
        value: Vec<crate::datadogV1::model::TableWidgetCellDisplayMode>,
    ) -> Self {
        self.cell_display_mode = Some(value);
        self
    }

    pub fn conditional_formats(
        mut self,
        value: Vec<crate::datadogV1::model::WidgetConditionalFormat>,
    ) -> Self {
        self.conditional_formats = Some(value);
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

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
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

    pub fn order(mut self, value: crate::datadogV1::model::WidgetSort) -> Self {
        self.order = Some(value);
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

    pub fn sort(mut self, value: crate::datadogV1::model::WidgetSortBy) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn text_formats(
        mut self,
        value: Vec<Vec<crate::datadogV1::model::TableWidgetTextFormatRule>>,
    ) -> Self {
        self.text_formats = Some(value);
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

impl Default for TableWidgetRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TableWidgetRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TableWidgetRequestVisitor;
        impl<'a> Visitor<'a> for TableWidgetRequestVisitor {
            type Value = TableWidgetRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregator: Option<crate::datadogV1::model::WidgetAggregator> = None;
                let mut alias: Option<String> = None;
                let mut apm_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut apm_stats_query: Option<crate::datadogV1::model::ApmStatsQueryDefinition> =
                    None;
                let mut cell_display_mode: Option<
                    Vec<crate::datadogV1::model::TableWidgetCellDisplayMode>,
                > = None;
                let mut conditional_formats: Option<
                    Vec<crate::datadogV1::model::WidgetConditionalFormat>,
                > = None;
                let mut event_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>> = None;
                let mut limit: Option<i64> = None;
                let mut log_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut network_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut order: Option<crate::datadogV1::model::WidgetSort> = None;
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
                let mut sort: Option<crate::datadogV1::model::WidgetSortBy> = None;
                let mut text_formats: Option<
                    Vec<Vec<crate::datadogV1::model::TableWidgetTextFormatRule>>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregator" => {
                            if v.is_null() {
                                continue;
                            }
                            aggregator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aggregator) = aggregator {
                                match _aggregator {
                                    crate::datadogV1::model::WidgetAggregator::UnparsedObject(
                                        _aggregator,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "alias" => {
                            if v.is_null() {
                                continue;
                            }
                            alias = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        "cell_display_mode" => {
                            if v.is_null() {
                                continue;
                            }
                            cell_display_mode =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "conditional_formats" => {
                            if v.is_null() {
                                continue;
                            }
                            conditional_formats =
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
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "order" => {
                            if v.is_null() {
                                continue;
                            }
                            order = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _order) = order {
                                match _order {
                                    crate::datadogV1::model::WidgetSort::UnparsedObject(_order) => {
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
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "text_formats" => {
                            if v.is_null() {
                                continue;
                            }
                            text_formats =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TableWidgetRequest {
                    aggregator,
                    alias,
                    apm_query,
                    apm_stats_query,
                    cell_display_mode,
                    conditional_formats,
                    event_query,
                    formulas,
                    limit,
                    log_query,
                    network_query,
                    order,
                    process_query,
                    profile_metrics_query,
                    q,
                    queries,
                    response_format,
                    rum_query,
                    security_query,
                    sort,
                    text_formats,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TableWidgetRequestVisitor)
    }
}
