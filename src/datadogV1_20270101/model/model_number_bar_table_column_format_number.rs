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
pub struct NumberBarTableColumnFormatNumber {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "conditional_formats")]
    pub conditional_formats: Option<crate::datadogV1_20270101::model::ConditionalFormats>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "mode")]
    pub mode: crate::datadogV1_20270101::model::NumberBarTableColumnFormatNumberMode,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "number_format")]
    pub number_format: Option<crate::datadogV1_20270101::model::WidgetNumberFormat>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl NumberBarTableColumnFormatNumber {
    pub fn new(mode: crate::datadogV1_20270101::model::NumberBarTableColumnFormatNumberMode) -> NumberBarTableColumnFormatNumber {
        NumberBarTableColumnFormatNumber {
            conditional_formats: None,
            mode,
            number_format: None,
            _unparsed: false,
        }
    }
    
    pub fn conditional_formats(mut self, value: crate::datadogV1_20270101::model::ConditionalFormats) -> Self {
        self.conditional_formats = Some(value);
        self
    }
    
    pub fn number_format(mut self, value: crate::datadogV1_20270101::model::WidgetNumberFormat) -> Self {
        self.number_format = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for NumberBarTableColumnFormatNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NumberBarTableColumnFormatNumberVisitor;
        impl<'a> Visitor<'a> for NumberBarTableColumnFormatNumberVisitor {
            type Value = NumberBarTableColumnFormatNumber;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut conditional_formats: Option<crate::datadogV1_20270101::model::ConditionalFormats> = None;
                let mut mode: Option<crate::datadogV1_20270101::model::NumberBarTableColumnFormatNumberMode> = None;
                let mut number_format: Option<crate::datadogV1_20270101::model::WidgetNumberFormat> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                        "mode" => {
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mode) = mode {
                                match _mode {
                                    crate::datadogV1_20270101::model::NumberBarTableColumnFormatNumberMode::UnparsedObject(_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "number_format" => {
                            if v.is_null() {
                                continue;
                            }
                            number_format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let mode = mode.ok_or_else(|| M::Error::missing_field("mode"))?;

                let content = NumberBarTableColumnFormatNumber {
                    conditional_formats,
                    mode,
                    number_format,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NumberBarTableColumnFormatNumberVisitor)
    }
}
