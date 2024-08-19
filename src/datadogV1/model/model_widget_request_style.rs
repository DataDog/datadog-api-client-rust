// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Define request widget style.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WidgetRequestStyle {
    /// Type of lines displayed.
    #[serde(rename = "line_type")]
    pub line_type: Option<crate::datadogV1::model::WidgetLineType>,
    /// Width of line displayed.
    #[serde(rename = "line_width")]
    pub line_width: Option<crate::datadogV1::model::WidgetLineWidth>,
    /// Color palette to apply to the widget.
    #[serde(rename = "palette")]
    pub palette: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WidgetRequestStyle {
    pub fn new() -> WidgetRequestStyle {
        WidgetRequestStyle {
            line_type: None,
            line_width: None,
            palette: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn line_type(mut self, value: crate::datadogV1::model::WidgetLineType) -> Self {
        self.line_type = Some(value);
        self
    }

    pub fn line_width(mut self, value: crate::datadogV1::model::WidgetLineWidth) -> Self {
        self.line_width = Some(value);
        self
    }

    pub fn palette(mut self, value: String) -> Self {
        self.palette = Some(value);
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

impl Default for WidgetRequestStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for WidgetRequestStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetRequestStyleVisitor;
        impl<'a> Visitor<'a> for WidgetRequestStyleVisitor {
            type Value = WidgetRequestStyle;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut line_type: Option<crate::datadogV1::model::WidgetLineType> = None;
                let mut line_width: Option<crate::datadogV1::model::WidgetLineWidth> = None;
                let mut palette: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "line_type" => {
                            if v.is_null() {
                                continue;
                            }
                            line_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _line_type) = line_type {
                                match _line_type {
                                    crate::datadogV1::model::WidgetLineType::UnparsedObject(
                                        _line_type,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "line_width" => {
                            if v.is_null() {
                                continue;
                            }
                            line_width = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _line_width) = line_width {
                                match _line_width {
                                    crate::datadogV1::model::WidgetLineWidth::UnparsedObject(
                                        _line_width,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "palette" => {
                            if v.is_null() {
                                continue;
                            }
                            palette = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = WidgetRequestStyle {
                    line_type,
                    line_width,
                    palette,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetRequestStyleVisitor)
    }
}
