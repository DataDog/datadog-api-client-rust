// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A request for a heatgrid widget that uses formulas and functions.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HeatgridWidgetRequest {
    /// The single displayed formula can combine multiple queries.
    #[serde(rename = "formulas")]
    pub formulas: Option<Vec<crate::datadogV1::model::HeatgridWidgetFormula>>,
    /// Queries returned directly or combined in a formula.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>,
    /// Response format for heatgrid queries.
    #[serde(rename = "response_format")]
    pub response_format: crate::datadogV1::model::HeatgridWidgetResponseFormat,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HeatgridWidgetRequest {
    pub fn new(
        queries: Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>,
        response_format: crate::datadogV1::model::HeatgridWidgetResponseFormat,
    ) -> HeatgridWidgetRequest {
        HeatgridWidgetRequest {
            formulas: None,
            queries,
            response_format,
            _unparsed: false,
        }
    }

    pub fn formulas(mut self, value: Vec<crate::datadogV1::model::HeatgridWidgetFormula>) -> Self {
        self.formulas = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for HeatgridWidgetRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeatgridWidgetRequestVisitor;
        impl<'a> Visitor<'a> for HeatgridWidgetRequestVisitor {
            type Value = HeatgridWidgetRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut formulas: Option<Vec<crate::datadogV1::model::HeatgridWidgetFormula>> =
                    None;
                let mut queries: Option<
                    Vec<crate::datadogV1::model::FormulaAndFunctionQueryDefinition>,
                > = None;
                let mut response_format: Option<
                    crate::datadogV1::model::HeatgridWidgetResponseFormat,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "formulas" => {
                            if v.is_null() {
                                continue;
                            }
                            formulas = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queries" => {
                            queries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "response_format" => {
                            response_format =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _response_format) = response_format {
                                match _response_format {
                                    crate::datadogV1::model::HeatgridWidgetResponseFormat::UnparsedObject(_response_format) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let queries = queries.ok_or_else(|| M::Error::missing_field("queries"))?;
                let response_format =
                    response_format.ok_or_else(|| M::Error::missing_field("response_format"))?;

                let content = HeatgridWidgetRequest {
                    formulas,
                    queries,
                    response_format,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HeatgridWidgetRequestVisitor)
    }
}
