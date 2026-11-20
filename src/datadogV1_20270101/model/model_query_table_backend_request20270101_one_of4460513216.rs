// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct QueryTableBackendRequest20270101OneOf4460513216 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "aggregator")]
    pub aggregator: Option<crate::datadogV1_20270101::model::ScalarAggregator>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "alias")]
    pub alias: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "cell_display_mode")]
    pub cell_display_mode: Option<Vec<crate::datadogV1_20270101::model::CellDisplayModeOption>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "conditional_formats")]
    pub conditional_formats: Option<Vec<crate::datadogV1_20270101::model::ThresholdFormattingRule>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "formulas")]
    pub formulas: Option<Vec<crate::datadogV1_20270101::model::TableFormula>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "limit")]
    pub limit: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "order")]
    pub order: Option<crate::datadogV1_20270101::model::QuerySortOrder>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::datadogV1_20270101::model::FormulaAndFunctionQueryDefinition>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "response_format")]
    pub response_format: crate::datadogV1_20270101::model::QueryTableBackendRequestAnyOfAnyOfItemResponseFormat4450132960,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1_20270101::model::ScalarFormulaSort>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "text_formats")]
    pub text_formats: Option<Vec<Vec<crate::datadogV1_20270101::model::TextFormattingRule>>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl QueryTableBackendRequest20270101OneOf4460513216 {
    pub fn new(queries: Vec<crate::datadogV1_20270101::model::FormulaAndFunctionQueryDefinition>, response_format: crate::datadogV1_20270101::model::QueryTableBackendRequestAnyOfAnyOfItemResponseFormat4450132960) -> QueryTableBackendRequest20270101OneOf4460513216 {
        QueryTableBackendRequest20270101OneOf4460513216 {
            aggregator: None,
            alias: None,
            cell_display_mode: None,
            conditional_formats: None,
            formulas: None,
            limit: None,
            order: None,
            queries,
            response_format,
            sort: None,
            text_formats: None,
            _unparsed: false,
        }
    }
    
    pub fn aggregator(mut self, value: crate::datadogV1_20270101::model::ScalarAggregator) -> Self {
        self.aggregator = Some(value);
        self
    }
    
    pub fn alias(mut self, value: String) -> Self {
        self.alias = Some(value);
        self
    }
    
    pub fn cell_display_mode(mut self, value: Vec<crate::datadogV1_20270101::model::CellDisplayModeOption>) -> Self {
        self.cell_display_mode = Some(value);
        self
    }
    
    pub fn conditional_formats(mut self, value: Vec<crate::datadogV1_20270101::model::ThresholdFormattingRule>) -> Self {
        self.conditional_formats = Some(value);
        self
    }
    
    pub fn formulas(mut self, value: Vec<crate::datadogV1_20270101::model::TableFormula>) -> Self {
        self.formulas = Some(value);
        self
    }
    
    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);
        self
    }
    
    pub fn order(mut self, value: crate::datadogV1_20270101::model::QuerySortOrder) -> Self {
        self.order = Some(value);
        self
    }
    
    pub fn sort(mut self, value: crate::datadogV1_20270101::model::ScalarFormulaSort) -> Self {
        self.sort = Some(value);
        self
    }
    
    pub fn text_formats(mut self, value: Vec<Vec<crate::datadogV1_20270101::model::TextFormattingRule>>) -> Self {
        self.text_formats = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for QueryTableBackendRequest20270101OneOf4460513216 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct QueryTableBackendRequest20270101OneOf4460513216Visitor;
        impl<'a> Visitor<'a> for QueryTableBackendRequest20270101OneOf4460513216Visitor {
            type Value = QueryTableBackendRequest20270101OneOf4460513216;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregator: Option<crate::datadogV1_20270101::model::ScalarAggregator> = None;
                let mut alias: Option<String> = None;
                let mut cell_display_mode: Option<Vec<crate::datadogV1_20270101::model::CellDisplayModeOption>> = None;
                let mut conditional_formats: Option<Vec<crate::datadogV1_20270101::model::ThresholdFormattingRule>> = None;
                let mut formulas: Option<Vec<crate::datadogV1_20270101::model::TableFormula>> = None;
                let mut limit: Option<f64> = None;
                let mut order: Option<crate::datadogV1_20270101::model::QuerySortOrder> = None;
                let mut queries: Option<Vec<crate::datadogV1_20270101::model::FormulaAndFunctionQueryDefinition>> = None;
                let mut response_format: Option<crate::datadogV1_20270101::model::QueryTableBackendRequestAnyOfAnyOfItemResponseFormat4450132960> = None;
                let mut sort: Option<crate::datadogV1_20270101::model::ScalarFormulaSort> = None;
                let mut text_formats: Option<Vec<Vec<crate::datadogV1_20270101::model::TextFormattingRule>>> = None;
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
                                    crate::datadogV1_20270101::model::ScalarAggregator::UnparsedObject(_aggregator) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "alias" => {
                            if v.is_null() {
                                continue;
                            }
                            alias = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "cell_display_mode" => {
                            if v.is_null() {
                                continue;
                            }
                            cell_display_mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "conditional_formats" => {
                            if v.is_null() {
                                continue;
                            }
                            conditional_formats = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "formulas" => {
                            if v.is_null() {
                                continue;
                            }
                            formulas = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "limit" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "order" => {
                            if v.is_null() {
                                continue;
                            }
                            order = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _order) = order {
                                match _order {
                                    crate::datadogV1_20270101::model::QuerySortOrder::UnparsedObject(_order) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "queries" => {
                            queries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "response_format" => {
                            response_format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _response_format) = response_format {
                                match _response_format {
                                    crate::datadogV1_20270101::model::QueryTableBackendRequestAnyOfAnyOfItemResponseFormat4450132960::UnparsedObject(_response_format) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "text_formats" => {
                            if v.is_null() {
                                continue;
                            }
                            text_formats = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let queries = queries.ok_or_else(|| M::Error::missing_field("queries"))?;
                let response_format = response_format.ok_or_else(|| M::Error::missing_field("response_format"))?;

                let content = QueryTableBackendRequest20270101OneOf4460513216 {
                    aggregator,
                    alias,
                    cell_display_mode,
                    conditional_formats,
                    formulas,
                    limit,
                    order,
                    queries,
                    response_format,
                    sort,
                    text_formats,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(QueryTableBackendRequest20270101OneOf4460513216Visitor)
    }
}
