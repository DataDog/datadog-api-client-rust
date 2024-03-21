// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The style to apply to the widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HostMapWidgetDefinitionStyle {
    /// Max value to use to color the map.
    #[serde(rename = "fill_max")]
    pub fill_max: Option<String>,
    /// Min value to use to color the map.
    #[serde(rename = "fill_min")]
    pub fill_min: Option<String>,
    /// Color palette to apply to the widget.
    #[serde(rename = "palette")]
    pub palette: Option<String>,
    /// Whether to flip the palette tones.
    #[serde(rename = "palette_flip")]
    pub palette_flip: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HostMapWidgetDefinitionStyle {
    pub fn new() -> HostMapWidgetDefinitionStyle {
        HostMapWidgetDefinitionStyle {
            fill_max: None,
            fill_min: None,
            palette: None,
            palette_flip: None,
            _unparsed: false,
        }
    }

    pub fn fill_max(mut self, value: String) -> Self {
        self.fill_max = Some(value);
        self
    }

    pub fn fill_min(mut self, value: String) -> Self {
        self.fill_min = Some(value);
        self
    }

    pub fn palette(mut self, value: String) -> Self {
        self.palette = Some(value);
        self
    }

    pub fn palette_flip(mut self, value: bool) -> Self {
        self.palette_flip = Some(value);
        self
    }
}

impl Default for HostMapWidgetDefinitionStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HostMapWidgetDefinitionStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostMapWidgetDefinitionStyleVisitor;
        impl<'a> Visitor<'a> for HostMapWidgetDefinitionStyleVisitor {
            type Value = HostMapWidgetDefinitionStyle;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut fill_max: Option<String> = None;
                let mut fill_min: Option<String> = None;
                let mut palette: Option<String> = None;
                let mut palette_flip: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "fill_max" => {
                            if v.is_null() {
                                continue;
                            }
                            fill_max = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fill_min" => {
                            if v.is_null() {
                                continue;
                            }
                            fill_min = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "palette" => {
                            if v.is_null() {
                                continue;
                            }
                            palette = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "palette_flip" => {
                            if v.is_null() {
                                continue;
                            }
                            palette_flip =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = HostMapWidgetDefinitionStyle {
                    fill_max,
                    fill_min,
                    palette,
                    palette_flip,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HostMapWidgetDefinitionStyleVisitor)
    }
}
