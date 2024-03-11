// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Axis controls for the widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WidgetAxis {
    /// Set to `true` to include zero.
    #[serde(rename = "include_zero")]
    pub include_zero: Option<bool>,
    /// The label of the axis to display on the graph. Only usable on Scatterplot Widgets.
    #[serde(rename = "label")]
    pub label: Option<String>,
    /// Specifies maximum numeric value to show on the axis. Defaults to `auto`.
    #[serde(rename = "max")]
    pub max: Option<String>,
    /// Specifies minimum numeric value to show on the axis. Defaults to `auto`.
    #[serde(rename = "min")]
    pub min: Option<String>,
    /// Specifies the scale type. Possible values are `linear`, `log`, `sqrt`, and `pow##` (for example `pow2` or `pow0.5`).
    #[serde(rename = "scale")]
    pub scale: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WidgetAxis {
    pub fn new() -> WidgetAxis {
        WidgetAxis {
            include_zero: None,
            label: None,
            max: None,
            min: None,
            scale: None,
            _unparsed: false,
        }
    }

    pub fn include_zero(&mut self, value: bool) -> &mut Self {
        self.include_zero = Some(value);
        self
    }

    pub fn label(&mut self, value: String) -> &mut Self {
        self.label = Some(value);
        self
    }

    pub fn max(&mut self, value: String) -> &mut Self {
        self.max = Some(value);
        self
    }

    pub fn min(&mut self, value: String) -> &mut Self {
        self.min = Some(value);
        self
    }

    pub fn scale(&mut self, value: String) -> &mut Self {
        self.scale = Some(value);
        self
    }
}

impl Default for WidgetAxis {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for WidgetAxis {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetAxisVisitor;
        impl<'a> Visitor<'a> for WidgetAxisVisitor {
            type Value = WidgetAxis;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut include_zero: Option<bool> = None;
                let mut label: Option<String> = None;
                let mut max: Option<String> = None;
                let mut min: Option<String> = None;
                let mut scale: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "include_zero" => {
                            if v.is_null() {
                                continue;
                            }
                            include_zero =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "label" => {
                            if v.is_null() {
                                continue;
                            }
                            label = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "max" => {
                            if v.is_null() {
                                continue;
                            }
                            max = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "min" => {
                            if v.is_null() {
                                continue;
                            }
                            min = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scale" => {
                            if v.is_null() {
                                continue;
                            }
                            scale = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = WidgetAxis {
                    include_zero,
                    label,
                    max,
                    min,
                    scale,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetAxisVisitor)
    }
}
