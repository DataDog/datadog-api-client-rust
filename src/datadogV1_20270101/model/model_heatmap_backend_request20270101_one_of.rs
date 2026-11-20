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
pub struct HeatmapBackendRequest20270101OneOf {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "formulas")]
    pub formulas: Option<Vec<crate::datadogV1_20270101::model::WidgetFormula>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::datadogV1_20270101::model::FormulaAndFunctionQueryDefinition>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "response_format")]
    pub response_format: crate::datadogV1_20270101::model::HeatmapBackendRequestAnyOfAnyOfItemResponseFormat,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1_20270101::model::RequestStyle>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl HeatmapBackendRequest20270101OneOf {
    pub fn new(queries: Vec<crate::datadogV1_20270101::model::FormulaAndFunctionQueryDefinition>, response_format: crate::datadogV1_20270101::model::HeatmapBackendRequestAnyOfAnyOfItemResponseFormat) -> HeatmapBackendRequest20270101OneOf {
        HeatmapBackendRequest20270101OneOf {
            formulas: None,
            queries,
            response_format,
            style: None,
            _unparsed: false,
        }
    }
    
    pub fn formulas(mut self, value: Vec<crate::datadogV1_20270101::model::WidgetFormula>) -> Self {
        self.formulas = Some(value);
        self
    }
    
    pub fn style(mut self, value: crate::datadogV1_20270101::model::RequestStyle) -> Self {
        self.style = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for HeatmapBackendRequest20270101OneOf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeatmapBackendRequest20270101OneOfVisitor;
        impl<'a> Visitor<'a> for HeatmapBackendRequest20270101OneOfVisitor {
            type Value = HeatmapBackendRequest20270101OneOf;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut formulas: Option<Vec<crate::datadogV1_20270101::model::WidgetFormula>> = None;
                let mut queries: Option<Vec<crate::datadogV1_20270101::model::FormulaAndFunctionQueryDefinition>> = None;
                let mut response_format: Option<crate::datadogV1_20270101::model::HeatmapBackendRequestAnyOfAnyOfItemResponseFormat> = None;
                let mut style: Option<crate::datadogV1_20270101::model::RequestStyle> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                                    crate::datadogV1_20270101::model::HeatmapBackendRequestAnyOfAnyOfItemResponseFormat::UnparsedObject(_response_format) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "style" => {
                            if v.is_null() {
                                continue;
                            }
                            style = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let queries = queries.ok_or_else(|| M::Error::missing_field("queries"))?;
                let response_format = response_format.ok_or_else(|| M::Error::missing_field("response_format"))?;

                let content = HeatmapBackendRequest20270101OneOf {
                    formulas,
                    queries,
                    response_format,
                    style,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HeatmapBackendRequest20270101OneOfVisitor)
    }
}
