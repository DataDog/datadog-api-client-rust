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
pub struct TableFormula20270101OneOf {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "alias")]
    pub alias: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "cell_display_mode")]
    pub cell_display_mode: Option<crate::datadogV1::model::TableFormulaAnyOfAnyOfItemCellDisplayMode>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "cell_display_mode_options")]
    pub cell_display_mode_options: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "conditional_formats")]
    pub conditional_formats: Option<crate::datadogV1::model::ConditionalFormats>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "formula")]
    pub formula: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "limit")]
    pub limit: Option<crate::datadogV1::model::TableFormulaAnyOfAnyOfItemLimit>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "number_format")]
    pub number_format: Option<crate::datadogV1::model::WidgetNumberFormat>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1::model::WidgetFormulaStyle>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TableFormula20270101OneOf {
    pub fn new(formula: String) -> TableFormula20270101OneOf {
        TableFormula20270101OneOf {
            alias: None,
            cell_display_mode: None,
            cell_display_mode_options: None,
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
    
    pub fn cell_display_mode(mut self, value: crate::datadogV1::model::TableFormulaAnyOfAnyOfItemCellDisplayMode) -> Self {
        self.cell_display_mode = Some(value);
        self
    }
    
    pub fn cell_display_mode_options(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.cell_display_mode_options = Some(value);
        self
    }
    
    pub fn conditional_formats(mut self, value: crate::datadogV1::model::ConditionalFormats) -> Self {
        self.conditional_formats = Some(value);
        self
    }
    
    pub fn limit(mut self, value: crate::datadogV1::model::TableFormulaAnyOfAnyOfItemLimit) -> Self {
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



impl<'de> Deserialize<'de> for TableFormula20270101OneOf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TableFormula20270101OneOfVisitor;
        impl<'a> Visitor<'a> for TableFormula20270101OneOfVisitor {
            type Value = TableFormula20270101OneOf;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alias: Option<String> = None;
                let mut cell_display_mode: Option<crate::datadogV1::model::TableFormulaAnyOfAnyOfItemCellDisplayMode> = None;
                let mut cell_display_mode_options: Option<std::collections::BTreeMap<String, serde_json::Value>> = None;
                let mut conditional_formats: Option<crate::datadogV1::model::ConditionalFormats> = None;
                let mut formula: Option<String> = None;
                let mut limit: Option<crate::datadogV1::model::TableFormulaAnyOfAnyOfItemLimit> = None;
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
                        },
                        "cell_display_mode" => {
                            if v.is_null() {
                                continue;
                            }
                            cell_display_mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _cell_display_mode) = cell_display_mode {
                                match _cell_display_mode {
                                    crate::datadogV1::model::TableFormulaAnyOfAnyOfItemCellDisplayMode::UnparsedObject(_cell_display_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "cell_display_mode_options" => {
                            if v.is_null() {
                                continue;
                            }
                            cell_display_mode_options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "conditional_formats" => {
                            if v.is_null() {
                                continue;
                            }
                            conditional_formats = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _conditional_formats) = conditional_formats {
                                match _conditional_formats {
                                    crate::datadogV1::model::ConditionalFormats::UnparsedObject(_conditional_formats) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "formula" => {
                            formula = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "number_format" => {
                            if v.is_null() {
                                continue;
                            }
                            number_format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let formula = formula.ok_or_else(|| M::Error::missing_field("formula"))?;

                let content = TableFormula20270101OneOf {
                    alias,
                    cell_display_mode,
                    cell_display_mode_options,
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

        deserializer.deserialize_any(TableFormula20270101OneOfVisitor)
    }
}
