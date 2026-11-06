// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration of the group label column. Its width is sized automatically when omitted.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HeatgridLabelColumn {
    /// Ordered string replacement rules for the group labels.
    #[serde(rename = "text_formats")]
    pub text_formats: Option<Vec<crate::datadogV1::model::HeatgridTextFormatRule>>,
    /// Width of the label column.
    #[serde(rename = "width")]
    pub width: Option<crate::datadogV1::model::HeatgridLabelColumnWidth>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HeatgridLabelColumn {
    pub fn new() -> HeatgridLabelColumn {
        HeatgridLabelColumn {
            text_formats: None,
            width: None,
            _unparsed: false,
        }
    }

    pub fn text_formats(
        mut self,
        value: Vec<crate::datadogV1::model::HeatgridTextFormatRule>,
    ) -> Self {
        self.text_formats = Some(value);
        self
    }

    pub fn width(mut self, value: crate::datadogV1::model::HeatgridLabelColumnWidth) -> Self {
        self.width = Some(value);
        self
    }
}

impl Default for HeatgridLabelColumn {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HeatgridLabelColumn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeatgridLabelColumnVisitor;
        impl<'a> Visitor<'a> for HeatgridLabelColumnVisitor {
            type Value = HeatgridLabelColumn;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut text_formats: Option<Vec<crate::datadogV1::model::HeatgridTextFormatRule>> =
                    None;
                let mut width: Option<crate::datadogV1::model::HeatgridLabelColumnWidth> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "text_formats" => {
                            if v.is_null() {
                                continue;
                            }
                            text_formats =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "width" => {
                            if v.is_null() {
                                continue;
                            }
                            width = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _width) = width {
                                match _width {
                                    crate::datadogV1::model::HeatgridLabelColumnWidth::UnparsedObject(_width) => {
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

                let content = HeatgridLabelColumn {
                    text_formats,
                    width,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HeatgridLabelColumnVisitor)
    }
}
