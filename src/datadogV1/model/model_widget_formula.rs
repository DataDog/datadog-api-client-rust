// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Formula to be used in a widget query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WidgetFormula {
    /// Expression alias.
    #[serde(rename = "alias")]
    pub alias: Option<String>,
    /// Define a display mode for the table cell.
    #[serde(rename = "cell_display_mode")]
    pub cell_display_mode: Option<crate::datadogV1::model::TableWidgetCellDisplayMode>,
    /// Cell display mode options for the widget formula. (only if `cell_display_mode` is set to `trend`).
    #[serde(rename = "cell_display_mode_options")]
    pub cell_display_mode_options:
        Option<crate::datadogV1::model::WidgetFormulaCellDisplayModeOptions>,
    /// List of conditional formats.
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
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WidgetFormula {
    pub fn new(formula: String) -> WidgetFormula {
        WidgetFormula {
            alias: None,
            cell_display_mode: None,
            cell_display_mode_options: None,
            conditional_formats: None,
            formula,
            limit: None,
            number_format: None,
            style: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn alias(mut self, value: String) -> Self {
        self.alias = Some(value);
        self
    }

    pub fn cell_display_mode(
        mut self,
        value: crate::datadogV1::model::TableWidgetCellDisplayMode,
    ) -> Self {
        self.cell_display_mode = Some(value);
        self
    }

    pub fn cell_display_mode_options(
        mut self,
        value: crate::datadogV1::model::WidgetFormulaCellDisplayModeOptions,
    ) -> Self {
        self.cell_display_mode_options = Some(value);
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for WidgetFormula {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetFormulaVisitor;
        impl<'a> Visitor<'a> for WidgetFormulaVisitor {
            type Value = WidgetFormula;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alias: Option<String> = None;
                let mut cell_display_mode: Option<
                    crate::datadogV1::model::TableWidgetCellDisplayMode,
                > = None;
                let mut cell_display_mode_options: Option<
                    crate::datadogV1::model::WidgetFormulaCellDisplayModeOptions,
                > = None;
                let mut conditional_formats: Option<
                    Vec<crate::datadogV1::model::WidgetConditionalFormat>,
                > = None;
                let mut formula: Option<String> = None;
                let mut limit: Option<crate::datadogV1::model::WidgetFormulaLimit> = None;
                let mut number_format: Option<crate::datadogV1::model::WidgetNumberFormat> = None;
                let mut style: Option<crate::datadogV1::model::WidgetFormulaStyle> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "alias" => {
                            if v.is_null() {
                                continue;
                            }
                            alias = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cell_display_mode" => {
                            if v.is_null() {
                                continue;
                            }
                            cell_display_mode =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _cell_display_mode) = cell_display_mode {
                                match _cell_display_mode {
                                    crate::datadogV1::model::TableWidgetCellDisplayMode::UnparsedObject(_cell_display_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "cell_display_mode_options" => {
                            if v.is_null() {
                                continue;
                            }
                            cell_display_mode_options =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let formula = formula.ok_or_else(|| M::Error::missing_field("formula"))?;

                let content = WidgetFormula {
                    alias,
                    cell_display_mode,
                    cell_display_mode_options,
                    conditional_formats,
                    formula,
                    limit,
                    number_format,
                    style,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetFormulaVisitor)
    }
}
