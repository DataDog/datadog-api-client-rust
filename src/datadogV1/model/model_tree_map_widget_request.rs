// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An updated treemap widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TreeMapWidgetRequest {
    /// List of formulas that operate on queries.
    #[serde(rename = "formulas")]
    pub formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>>,
    /// The widget metrics query.
    #[serde(rename = "q")]
    pub q: Option<String>,
    /// List of queries that can be returned directly or used in formulas.
    #[serde(rename = "queries")]
    pub queries: Option<Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>>,
    /// Timeseries, scalar, or event list response. Event list response formats are supported by Geomap widgets.
    #[serde(rename = "response_format")]
    pub response_format: Option<crate::datadogV1::model::FormulaAndFunctionResponseFormat>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TreeMapWidgetRequest {
    pub fn new() -> TreeMapWidgetRequest {
        TreeMapWidgetRequest {
            formulas: None,
            q: None,
            queries: None,
            response_format: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn formulas(mut self, value: Vec<crate::datadogV1::model::WidgetFormula>) -> Self {
        self.formulas = Some(value);
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for TreeMapWidgetRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TreeMapWidgetRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TreeMapWidgetRequestVisitor;
        impl<'a> Visitor<'a> for TreeMapWidgetRequestVisitor {
            type Value = TreeMapWidgetRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>> = None;
                let mut q: Option<String> = None;
                let mut queries: Option<
                    Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>,
                > = None;
                let mut response_format: Option<
                    crate::datadogV1::model::FormulaAndFunctionResponseFormat,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "formulas" => {
                            if v.is_null() {
                                continue;
                            }
                            formulas = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TreeMapWidgetRequest {
                    formulas,
                    q,
                    queries,
                    response_format,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TreeMapWidgetRequestVisitor)
    }
}
