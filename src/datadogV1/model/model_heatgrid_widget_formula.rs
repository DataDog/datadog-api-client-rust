// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A formula for a heatgrid widget request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HeatgridWidgetFormula {
    /// Expression alias.
    #[serde(rename = "alias")]
    pub alias: Option<String>,
    /// Conditional formatting rules. These rules do not affect heatgrid rendering.
    /// Use the widget-level `color` configuration to control cell colors.
    #[serde(rename = "conditional_formats")]
    pub conditional_formats: Option<Vec<crate::datadogV1::model::WidgetConditionalFormat>>,
    /// String expression built from queries, formulas, and functions.
    #[serde(rename = "formula")]
    pub formula: String,
    /// Options for limiting results returned.
    #[serde(rename = "limit")]
    pub limit: Option<crate::datadogV1::model::WidgetFormulaLimit>,
    /// Number format options for the widget.
    #[serde(rename = "number_format")]
    pub number_format: Option<crate::datadogV1::model::WidgetNumberFormat>,
    /// Styling options for widget formulas.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1::model::WidgetFormulaStyle>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HeatgridWidgetFormula {
    pub fn new(formula: String) -> HeatgridWidgetFormula {
        HeatgridWidgetFormula {
            alias: None,
            conditional_formats: None,
            formula,
            limit: None,
            number_format: None,
            style: None,
            _unparsed: false,
        }
    }

    pub fn alias(mut self, value: String) -> Self {
        self.alias = Some(value);
        self
    }

    pub fn conditional_formats(
        mut self,
        value: Vec<crate::datadogV1::model::WidgetConditionalFormat>,
    ) -> Self {
        self.conditional_formats = Some(value);
        self
    }

    pub fn limit(mut self, value: crate::datadogV1::model::WidgetFormulaLimit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn number_format(mut self, value: crate::datadogV1::model::WidgetNumberFormat) -> Self {
        self.number_format = Some(value);
        self
    }

    pub fn style(mut self, value: crate::datadogV1::model::WidgetFormulaStyle) -> Self {
        self.style = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for HeatgridWidgetFormula {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeatgridWidgetFormulaVisitor;
        impl<'a> Visitor<'a> for HeatgridWidgetFormulaVisitor {
            type Value = HeatgridWidgetFormula;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alias: Option<String> = None;
                let mut conditional_formats: Option<
                    Vec<crate::datadogV1::model::WidgetConditionalFormat>,
                > = None;
                let mut formula: Option<String> = None;
                let mut limit: Option<crate::datadogV1::model::WidgetFormulaLimit> = None;
                let mut number_format: Option<crate::datadogV1::model::WidgetNumberFormat> = None;
                let mut style: Option<crate::datadogV1::model::WidgetFormulaStyle> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "alias" => {
                            if v.is_null() {
                                continue;
                            }
                            alias = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "conditional_formats" => {
                            if v.is_null() {
                                continue;
                            }
                            conditional_formats =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "formula" => {
                            formula = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "number_format" => {
                            if v.is_null() {
                                continue;
                            }
                            number_format =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "style" => {
                            if v.is_null() {
                                continue;
                            }
                            style = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let formula = formula.ok_or_else(|| M::Error::missing_field("formula"))?;

                let content = HeatgridWidgetFormula {
                    alias,
                    conditional_formats,
                    formula,
                    limit,
                    number_format,
                    style,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HeatgridWidgetFormulaVisitor)
    }
}
