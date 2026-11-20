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
pub struct ToplistBackendRequest20270101OneOf4463319360 {
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
    pub response_format: crate::datadogV1::model::ToplistBackendRequestAnyOfAnyOfItemResponseFormat4453033088,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1::model::ScalarFormulaSort>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ToplistBackendRequest20270101OneOf4463319360 {
    pub fn new(queries: Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>, response_format: crate::datadogV1::model::ToplistBackendRequestAnyOfAnyOfItemResponseFormat4453033088) -> ToplistBackendRequest20270101OneOf4463319360 {
        ToplistBackendRequest20270101OneOf4463319360 {
            conditional_formats: None,
            formulas: None,
            queries,
            response_format,
            sort: None,
            _unparsed: false,
        }
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



impl<'de> Deserialize<'de> for ToplistBackendRequest20270101OneOf4463319360 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ToplistBackendRequest20270101OneOf4463319360Visitor;
        impl<'a> Visitor<'a> for ToplistBackendRequest20270101OneOf4463319360Visitor {
            type Value = ToplistBackendRequest20270101OneOf4463319360;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut conditional_formats: Option<Vec<crate::datadogV1::model::ThresholdFormattingRule>> = None;
                let mut formulas: Option<Vec<crate::datadogV1::model::WidgetFormula>> = None;
                let mut queries: Option<Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>> = None;
                let mut response_format: Option<crate::datadogV1::model::ToplistBackendRequestAnyOfAnyOfItemResponseFormat4453033088> = None;
                let mut sort: Option<crate::datadogV1::model::ScalarFormulaSort> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                                    crate::datadogV1::model::ToplistBackendRequestAnyOfAnyOfItemResponseFormat4453033088::UnparsedObject(_response_format) => {
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

                let content = ToplistBackendRequest20270101OneOf4463319360 {
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

        deserializer.deserialize_any(ToplistBackendRequest20270101OneOf4463319360Visitor)
    }
}
