// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Cell display mode options for the widget formula. (only if `cell_display_mode` is set to `trend`).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WidgetFormulaCellDisplayModeOptions {
    /// Trend type for the cell display mode options.
    #[serde(rename = "trend_type")]
    pub trend_type: Option<crate::datadogV1::model::WidgetFormulaCellDisplayModeOptionsTrendType>,
    /// Y scale for the cell display mode options.
    #[serde(rename = "y_scale")]
    pub y_scale: Option<crate::datadogV1::model::WidgetFormulaCellDisplayModeOptionsYScale>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WidgetFormulaCellDisplayModeOptions {
    pub fn new() -> WidgetFormulaCellDisplayModeOptions {
        WidgetFormulaCellDisplayModeOptions {
            trend_type: None,
            y_scale: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn trend_type(
        mut self,
        value: crate::datadogV1::model::WidgetFormulaCellDisplayModeOptionsTrendType,
    ) -> Self {
        self.trend_type = Some(value);
        self
    }

    pub fn y_scale(
        mut self,
        value: crate::datadogV1::model::WidgetFormulaCellDisplayModeOptionsYScale,
    ) -> Self {
        self.y_scale = Some(value);
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

impl Default for WidgetFormulaCellDisplayModeOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for WidgetFormulaCellDisplayModeOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetFormulaCellDisplayModeOptionsVisitor;
        impl<'a> Visitor<'a> for WidgetFormulaCellDisplayModeOptionsVisitor {
            type Value = WidgetFormulaCellDisplayModeOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut trend_type: Option<
                    crate::datadogV1::model::WidgetFormulaCellDisplayModeOptionsTrendType,
                > = None;
                let mut y_scale: Option<
                    crate::datadogV1::model::WidgetFormulaCellDisplayModeOptionsYScale,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "trend_type" => {
                            if v.is_null() {
                                continue;
                            }
                            trend_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _trend_type) = trend_type {
                                match _trend_type {
                                    crate::datadogV1::model::WidgetFormulaCellDisplayModeOptionsTrendType::UnparsedObject(_trend_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "y_scale" => {
                            if v.is_null() {
                                continue;
                            }
                            y_scale = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _y_scale) = y_scale {
                                match _y_scale {
                                    crate::datadogV1::model::WidgetFormulaCellDisplayModeOptionsYScale::UnparsedObject(_y_scale) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = WidgetFormulaCellDisplayModeOptions {
                    trend_type,
                    y_scale,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetFormulaCellDisplayModeOptionsVisitor)
    }
}
