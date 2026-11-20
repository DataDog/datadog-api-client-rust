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
pub struct TimeseriesWidgetFormulaRequest {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "display_type")]
    pub display_type: Option<crate::datadogV1_20270101::model::WidgetDisplayType>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "formulas")]
    pub formulas: Option<Vec<crate::datadogV1_20270101::model::WidgetFormula>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "metadata")]
    pub metadata: Option<Vec<crate::datadogV1_20270101::model::TimeseriesWidgetExpressionAlias>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "on_right_yaxis")]
    pub on_right_yaxis: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::datadogV1_20270101::model::FormulaAndFunctionQueryDefinition>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "response_format")]
    pub response_format: crate::datadogV1_20270101::model::TimeseriesWidgetFormulaRequestResponseFormat,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1_20270101::model::TimeseriesRequestStyle>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TimeseriesWidgetFormulaRequest {
    pub fn new(queries: Vec<crate::datadogV1_20270101::model::FormulaAndFunctionQueryDefinition>, response_format: crate::datadogV1_20270101::model::TimeseriesWidgetFormulaRequestResponseFormat) -> TimeseriesWidgetFormulaRequest {
        TimeseriesWidgetFormulaRequest {
            display_type: None,
            formulas: None,
            metadata: None,
            on_right_yaxis: None,
            queries,
            response_format,
            style: None,
            _unparsed: false,
        }
    }
    
    pub fn display_type(mut self, value: crate::datadogV1_20270101::model::WidgetDisplayType) -> Self {
        self.display_type = Some(value);
        self
    }
    
    pub fn formulas(mut self, value: Vec<crate::datadogV1_20270101::model::WidgetFormula>) -> Self {
        self.formulas = Some(value);
        self
    }
    
    pub fn metadata(mut self, value: Vec<crate::datadogV1_20270101::model::TimeseriesWidgetExpressionAlias>) -> Self {
        self.metadata = Some(value);
        self
    }
    
    pub fn on_right_yaxis(mut self, value: bool) -> Self {
        self.on_right_yaxis = Some(value);
        self
    }
    
    pub fn style(mut self, value: crate::datadogV1_20270101::model::TimeseriesRequestStyle) -> Self {
        self.style = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for TimeseriesWidgetFormulaRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeseriesWidgetFormulaRequestVisitor;
        impl<'a> Visitor<'a> for TimeseriesWidgetFormulaRequestVisitor {
            type Value = TimeseriesWidgetFormulaRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut display_type: Option<crate::datadogV1_20270101::model::WidgetDisplayType> = None;
                let mut formulas: Option<Vec<crate::datadogV1_20270101::model::WidgetFormula>> = None;
                let mut metadata: Option<Vec<crate::datadogV1_20270101::model::TimeseriesWidgetExpressionAlias>> = None;
                let mut on_right_yaxis: Option<bool> = None;
                let mut queries: Option<Vec<crate::datadogV1_20270101::model::FormulaAndFunctionQueryDefinition>> = None;
                let mut response_format: Option<crate::datadogV1_20270101::model::TimeseriesWidgetFormulaRequestResponseFormat> = None;
                let mut style: Option<crate::datadogV1_20270101::model::TimeseriesRequestStyle> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "display_type" => {
                            if v.is_null() {
                                continue;
                            }
                            display_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _display_type) = display_type {
                                match _display_type {
                                    crate::datadogV1_20270101::model::WidgetDisplayType::UnparsedObject(_display_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "formulas" => {
                            if v.is_null() {
                                continue;
                            }
                            formulas = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "on_right_yaxis" => {
                            if v.is_null() {
                                continue;
                            }
                            on_right_yaxis = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "queries" => {
                            queries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "response_format" => {
                            response_format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _response_format) = response_format {
                                match _response_format {
                                    crate::datadogV1_20270101::model::TimeseriesWidgetFormulaRequestResponseFormat::UnparsedObject(_response_format) => {
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

                let content = TimeseriesWidgetFormulaRequest {
                    display_type,
                    formulas,
                    metadata,
                    on_right_yaxis,
                    queries,
                    response_format,
                    style,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeseriesWidgetFormulaRequestVisitor)
    }
}
