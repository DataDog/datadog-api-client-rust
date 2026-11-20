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
pub struct QueryTableProjectionDimension {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "cell_display_mode")]
    pub cell_display_mode: Option<crate::datadogV1_20270101::model::AdvancedQueryTableCellDisplayMode>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "column")]
    pub column: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "conditional_formats")]
    pub conditional_formats: Option<crate::datadogV1_20270101::model::ConditionalFormats>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "date_format")]
    pub date_format: Option<crate::datadogV1_20270101::model::QueryTableDateFormat>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "dimension")]
    pub dimension: crate::datadogV1_20270101::model::QueryTableProjectionDimensionDimension,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "is_hidden")]
    pub is_hidden: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "link")]
    pub link: Option<crate::datadogV1_20270101::model::QueryTableColumnLink>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "number_format")]
    pub number_format: Option<crate::datadogV1_20270101::model::WidgetNumberFormat>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "text_align")]
    pub text_align: Option<crate::datadogV1_20270101::model::WidgetTextAlign>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "text_formats")]
    pub text_formats: Option<Vec<crate::datadogV1_20270101::model::TextFormattingRule>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl QueryTableProjectionDimension {
    pub fn new(column: String, dimension: crate::datadogV1_20270101::model::QueryTableProjectionDimensionDimension) -> QueryTableProjectionDimension {
        QueryTableProjectionDimension {
            cell_display_mode: None,
            column,
            conditional_formats: None,
            date_format: None,
            dimension,
            is_hidden: None,
            link: None,
            number_format: None,
            text_align: None,
            text_formats: None,
            _unparsed: false,
        }
    }
    
    pub fn cell_display_mode(mut self, value: crate::datadogV1_20270101::model::AdvancedQueryTableCellDisplayMode) -> Self {
        self.cell_display_mode = Some(value);
        self
    }
    
    pub fn conditional_formats(mut self, value: crate::datadogV1_20270101::model::ConditionalFormats) -> Self {
        self.conditional_formats = Some(value);
        self
    }
    
    pub fn date_format(mut self, value: crate::datadogV1_20270101::model::QueryTableDateFormat) -> Self {
        self.date_format = Some(value);
        self
    }
    
    pub fn is_hidden(mut self, value: bool) -> Self {
        self.is_hidden = Some(value);
        self
    }
    
    pub fn link(mut self, value: crate::datadogV1_20270101::model::QueryTableColumnLink) -> Self {
        self.link = Some(value);
        self
    }
    
    pub fn number_format(mut self, value: crate::datadogV1_20270101::model::WidgetNumberFormat) -> Self {
        self.number_format = Some(value);
        self
    }
    
    pub fn text_align(mut self, value: crate::datadogV1_20270101::model::WidgetTextAlign) -> Self {
        self.text_align = Some(value);
        self
    }
    
    pub fn text_formats(mut self, value: Vec<crate::datadogV1_20270101::model::TextFormattingRule>) -> Self {
        self.text_formats = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for QueryTableProjectionDimension {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct QueryTableProjectionDimensionVisitor;
        impl<'a> Visitor<'a> for QueryTableProjectionDimensionVisitor {
            type Value = QueryTableProjectionDimension;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cell_display_mode: Option<crate::datadogV1_20270101::model::AdvancedQueryTableCellDisplayMode> = None;
                let mut column: Option<String> = None;
                let mut conditional_formats: Option<crate::datadogV1_20270101::model::ConditionalFormats> = None;
                let mut date_format: Option<crate::datadogV1_20270101::model::QueryTableDateFormat> = None;
                let mut dimension: Option<crate::datadogV1_20270101::model::QueryTableProjectionDimensionDimension> = None;
                let mut is_hidden: Option<bool> = None;
                let mut link: Option<crate::datadogV1_20270101::model::QueryTableColumnLink> = None;
                let mut number_format: Option<crate::datadogV1_20270101::model::WidgetNumberFormat> = None;
                let mut text_align: Option<crate::datadogV1_20270101::model::WidgetTextAlign> = None;
                let mut text_formats: Option<Vec<crate::datadogV1_20270101::model::TextFormattingRule>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cell_display_mode" => {
                            if v.is_null() {
                                continue;
                            }
                            cell_display_mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _cell_display_mode) = cell_display_mode {
                                match _cell_display_mode {
                                    crate::datadogV1_20270101::model::AdvancedQueryTableCellDisplayMode::UnparsedObject(_cell_display_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "column" => {
                            column = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "conditional_formats" => {
                            if v.is_null() {
                                continue;
                            }
                            conditional_formats = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _conditional_formats) = conditional_formats {
                                match _conditional_formats {
                                    crate::datadogV1_20270101::model::ConditionalFormats::UnparsedObject(_conditional_formats) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "date_format" => {
                            if v.is_null() {
                                continue;
                            }
                            date_format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _date_format) = date_format {
                                match _date_format {
                                    crate::datadogV1_20270101::model::QueryTableDateFormat::UnparsedObject(_date_format) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "dimension" => {
                            dimension = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _dimension) = dimension {
                                match _dimension {
                                    crate::datadogV1_20270101::model::QueryTableProjectionDimensionDimension::UnparsedObject(_dimension) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "is_hidden" => {
                            if v.is_null() {
                                continue;
                            }
                            is_hidden = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "link" => {
                            if v.is_null() {
                                continue;
                            }
                            link = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "number_format" => {
                            if v.is_null() {
                                continue;
                            }
                            number_format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "text_align" => {
                            if v.is_null() {
                                continue;
                            }
                            text_align = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _text_align) = text_align {
                                match _text_align {
                                    crate::datadogV1_20270101::model::WidgetTextAlign::UnparsedObject(_text_align) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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
                let column = column.ok_or_else(|| M::Error::missing_field("column"))?;
                let dimension = dimension.ok_or_else(|| M::Error::missing_field("dimension"))?;

                let content = QueryTableProjectionDimension {
                    cell_display_mode,
                    column,
                    conditional_formats,
                    date_format,
                    dimension,
                    is_hidden,
                    link,
                    number_format,
                    text_align,
                    text_formats,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(QueryTableProjectionDimensionVisitor)
    }
}
