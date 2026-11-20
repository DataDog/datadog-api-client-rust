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
pub struct FormulaTableColumn {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "alias")]
    pub alias: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "comparison")]
    pub comparison: Option<crate::datadogV1_20270101::model::TableColumnComparison>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "format")]
    pub format: crate::datadogV1_20270101::model::TableColumnFormat,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "formula")]
    pub formula: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "is_hidden")]
    pub is_hidden: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1_20270101::model::FormulaTableColumnType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl FormulaTableColumn {
    pub fn new(format: crate::datadogV1_20270101::model::TableColumnFormat, formula: String, name: String, type_: crate::datadogV1_20270101::model::FormulaTableColumnType) -> FormulaTableColumn {
        FormulaTableColumn {
            alias: None,
            comparison: None,
            format,
            formula,
            is_hidden: None,
            name,
            type_,
            _unparsed: false,
        }
    }
    
    pub fn alias(mut self, value: String) -> Self {
        self.alias = Some(value);
        self
    }
    
    pub fn comparison(mut self, value: crate::datadogV1_20270101::model::TableColumnComparison) -> Self {
        self.comparison = Some(value);
        self
    }
    
    pub fn is_hidden(mut self, value: bool) -> Self {
        self.is_hidden = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for FormulaTableColumn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaTableColumnVisitor;
        impl<'a> Visitor<'a> for FormulaTableColumnVisitor {
            type Value = FormulaTableColumn;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alias: Option<String> = None;
                let mut comparison: Option<crate::datadogV1_20270101::model::TableColumnComparison> = None;
                let mut format: Option<crate::datadogV1_20270101::model::TableColumnFormat> = None;
                let mut formula: Option<String> = None;
                let mut is_hidden: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::FormulaTableColumnType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "alias" => {
                            if v.is_null() {
                                continue;
                            }
                            alias = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "comparison" => {
                            if v.is_null() {
                                continue;
                            }
                            comparison = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _comparison) = comparison {
                                match _comparison {
                                    crate::datadogV1_20270101::model::TableColumnComparison::UnparsedObject(_comparison) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "format" => {
                            format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _format) = format {
                                match _format {
                                    crate::datadogV1_20270101::model::TableColumnFormat::UnparsedObject(_format) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "formula" => {
                            formula = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "is_hidden" => {
                            if v.is_null() {
                                continue;
                            }
                            is_hidden = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1_20270101::model::FormulaTableColumnType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let format = format.ok_or_else(|| M::Error::missing_field("format"))?;
                let formula = formula.ok_or_else(|| M::Error::missing_field("formula"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = FormulaTableColumn {
                    alias,
                    comparison,
                    format,
                    formula,
                    is_hidden,
                    name,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaTableColumnVisitor)
    }
}
