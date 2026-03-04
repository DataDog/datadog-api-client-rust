// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An updated geomap widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GeomapWidgetRequest {
    /// Widget columns.
    #[serde(rename = "columns")]
    pub columns: Option<Vec<crate::datadogV1::model::ListStreamColumn>>,
    /// Threshold (numeric) conditional formatting rules may be used by a regions layer.
    #[serde(rename = "conditional_formats")]
    pub conditional_formats: Option<Vec<crate::datadogV1::model::WidgetConditionalFormat>>,
    /// List of formulas that operate on queries.
    #[serde(rename = "formulas")]
    pub formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>>,
    /// The log query.
    #[serde(rename = "log_query")]
    pub log_query: Option<crate::datadogV1::model::LogQueryDefinition>,
    /// The widget metrics query. Deprecated - Use `queries` and `formulas` instead.
    #[deprecated]
    #[serde(rename = "q")]
    pub q: Option<String>,
    /// List of queries that can be returned directly or used in formulas.
    #[serde(rename = "queries")]
    pub queries: Option<Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>>,
    /// Updated list stream widget.
    #[serde(rename = "query")]
    pub query: Option<crate::datadogV1::model::ListStreamQuery>,
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
    /// The style to apply to the request for points layer.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1::model::GeomapWidgetRequestStyle>,
    /// Text formatting rules may be used by a points layer.
    #[serde(rename = "text_formats")]
    pub text_formats: Option<Vec<crate::datadogV1::model::TableWidgetTextFormatRule>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GeomapWidgetRequest {
    pub fn new() -> GeomapWidgetRequest {
        #[allow(deprecated)]
        GeomapWidgetRequest {
            columns: None,
            conditional_formats: None,
            formulas: None,
            log_query: None,
            q: None,
            queries: None,
            query: None,
            response_format: None,
            rum_query: None,
            security_query: None,
            sort: None,
            style: None,
            text_formats: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn columns(mut self, value: Vec<crate::datadogV1::model::ListStreamColumn>) -> Self {
        self.columns = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn conditional_formats(
        mut self,
        value: Vec<crate::datadogV1::model::WidgetConditionalFormat>,
    ) -> Self {
        self.conditional_formats = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn formulas(mut self, value: Vec<crate::datadogV1::model::WidgetFormula>) -> Self {
        self.formulas = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn log_query(mut self, value: crate::datadogV1::model::LogQueryDefinition) -> Self {
        self.log_query = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn q(mut self, value: String) -> Self {
        self.q = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn queries(
        mut self,
        value: Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>,
    ) -> Self {
        self.queries = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn query(mut self, value: crate::datadogV1::model::ListStreamQuery) -> Self {
        self.query = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn response_format(
        mut self,
        value: crate::datadogV1::model::FormulaAndFunctionResponseFormat,
    ) -> Self {
        self.response_format = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn rum_query(mut self, value: crate::datadogV1::model::LogQueryDefinition) -> Self {
        self.rum_query = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn security_query(mut self, value: crate::datadogV1::model::LogQueryDefinition) -> Self {
        self.security_query = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn sort(mut self, value: crate::datadogV1::model::WidgetSortBy) -> Self {
        self.sort = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn style(mut self, value: crate::datadogV1::model::GeomapWidgetRequestStyle) -> Self {
        self.style = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn text_formats(
        mut self,
        value: Vec<crate::datadogV1::model::TableWidgetTextFormatRule>,
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

impl Default for GeomapWidgetRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GeomapWidgetRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GeomapWidgetRequestVisitor;
        impl<'a> Visitor<'a> for GeomapWidgetRequestVisitor {
            type Value = GeomapWidgetRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut columns: Option<Vec<crate::datadogV1::model::ListStreamColumn>> = None;
                let mut conditional_formats: Option<
                    Vec<crate::datadogV1::model::WidgetConditionalFormat>,
                > = None;
                let mut formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>> = None;
                let mut log_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut q: Option<String> = None;
                let mut queries: Option<
                    Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>,
                > = None;
                let mut query: Option<crate::datadogV1::model::ListStreamQuery> = None;
                let mut response_format: Option<
                    crate::datadogV1::model::FormulaAndFunctionResponseFormat,
                > = None;
                let mut rum_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut security_query: Option<crate::datadogV1::model::LogQueryDefinition> = None;
                let mut sort: Option<crate::datadogV1::model::WidgetSortBy> = None;
                let mut style: Option<crate::datadogV1::model::GeomapWidgetRequestStyle> = None;
                let mut text_formats: Option<
                    Vec<crate::datadogV1::model::TableWidgetTextFormatRule>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "columns" => {
                            if v.is_null() {
                                continue;
                            }
                            columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "conditional_formats" => {
                            if v.is_null() {
                                continue;
                            }
                            conditional_formats =
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
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "style" => {
                            if v.is_null() {
                                continue;
                            }
                            style = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                #[allow(deprecated)]
                let content = GeomapWidgetRequest {
                    columns,
                    conditional_formats,
                    formulas,
                    log_query,
                    q,
                    queries,
                    query,
                    response_format,
                    rum_query,
                    security_query,
                    sort,
                    style,
                    text_formats,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GeomapWidgetRequestVisitor)
    }
}
