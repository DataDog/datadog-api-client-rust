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
pub struct QueryValueBackendRequest20270101OneOf4460917152 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "aggregator")]
    pub aggregator: Option<crate::datadogV1::model::ScalarAggregator>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "comparison")]
    pub comparison: Option<crate::datadogV1::model::ComparisonRequest>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "conditional_formats")]
    pub conditional_formats: Option<Vec<crate::datadogV1::model::ThresholdFormattingRule>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "formulas")]
    pub formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "response_format")]
    pub response_format: crate::datadogV1::model::QueryValueBackendRequestAnyOfAnyOfItemResponseFormat4450609856,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1::model::ScalarFormulaSort>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl QueryValueBackendRequest20270101OneOf4460917152 {
    pub fn new(queries: Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>, response_format: crate::datadogV1::model::QueryValueBackendRequestAnyOfAnyOfItemResponseFormat4450609856) -> QueryValueBackendRequest20270101OneOf4460917152 {
        QueryValueBackendRequest20270101OneOf4460917152 {
            aggregator: None,
            comparison: None,
            conditional_formats: None,
            formulas: None,
            queries,
            response_format,
            sort: None,
            _unparsed: false,
        }
    }
    
    pub fn aggregator(mut self, value: crate::datadogV1::model::ScalarAggregator) -> Self {
        self.aggregator = Some(value);
        self
    }
    
    pub fn comparison(mut self, value: crate::datadogV1::model::ComparisonRequest) -> Self {
        self.comparison = Some(value);
        self
    }
    
    pub fn conditional_formats(mut self, value: Vec<crate::datadogV1::model::ThresholdFormattingRule>) -> Self {
        self.conditional_formats = Some(value);
        self
    }
    
    pub fn formulas(mut self, value: Vec<crate::datadogV1::model::WidgetFormula>) -> Self {
        self.formulas = Some(value);
        self
    }
    
    pub fn sort(mut self, value: crate::datadogV1::model::ScalarFormulaSort) -> Self {
        self.sort = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for QueryValueBackendRequest20270101OneOf4460917152 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct QueryValueBackendRequest20270101OneOf4460917152Visitor;
        impl<'a> Visitor<'a> for QueryValueBackendRequest20270101OneOf4460917152Visitor {
            type Value = QueryValueBackendRequest20270101OneOf4460917152;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregator: Option<crate::datadogV1::model::ScalarAggregator> = None;
                let mut comparison: Option<crate::datadogV1::model::ComparisonRequest> = None;
                let mut conditional_formats: Option<Vec<crate::datadogV1::model::ThresholdFormattingRule>> = None;
                let mut formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>> = None;
                let mut queries: Option<Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>> = None;
                let mut response_format: Option<crate::datadogV1::model::QueryValueBackendRequestAnyOfAnyOfItemResponseFormat4450609856> = None;
                let mut sort: Option<crate::datadogV1::model::ScalarFormulaSort> = None;
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
                                    crate::datadogV1::model::ScalarAggregator::UnparsedObject(_aggregator) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "comparison" => {
                            if v.is_null() {
                                continue;
                            }
                            comparison = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "queries" => {
                            queries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "response_format" => {
                            response_format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _response_format) = response_format {
                                match _response_format {
                                    crate::datadogV1::model::QueryValueBackendRequestAnyOfAnyOfItemResponseFormat4450609856::UnparsedObject(_response_format) => {
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
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let queries = queries.ok_or_else(|| M::Error::missing_field("queries"))?;
                let response_format = response_format.ok_or_else(|| M::Error::missing_field("response_format"))?;

                let content = QueryValueBackendRequest20270101OneOf4460917152 {
                    aggregator,
                    comparison,
                    conditional_formats,
                    formulas,
                    queries,
                    response_format,
                    sort,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(QueryValueBackendRequest20270101OneOf4460917152Visitor)
    }
}
