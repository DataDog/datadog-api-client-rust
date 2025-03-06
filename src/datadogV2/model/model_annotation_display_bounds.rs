// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `AnnotationDisplayBounds` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AnnotationDisplayBounds {
    /// The `bounds` `height`.
    #[serde(rename = "height")]
    pub height: Option<f64>,
    /// The `bounds` `width`.
    #[serde(rename = "width")]
    pub width: Option<f64>,
    /// The `bounds` `x`.
    #[serde(rename = "x")]
    pub x: Option<f64>,
    /// The `bounds` `y`.
    #[serde(rename = "y")]
    pub y: Option<f64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AnnotationDisplayBounds {
    pub fn new() -> AnnotationDisplayBounds {
        AnnotationDisplayBounds {
            height: None,
            width: None,
            x: None,
            y: None,
            _unparsed: false,
        }
    }

    pub fn height(mut self, value: f64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn width(mut self, value: f64) -> Self {
        self.width = Some(value);
        self
    }

    pub fn x(mut self, value: f64) -> Self {
        self.x = Some(value);
        self
    }

    pub fn y(mut self, value: f64) -> Self {
        self.y = Some(value);
        self
    }
}

impl Default for AnnotationDisplayBounds {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AnnotationDisplayBounds {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AnnotationDisplayBoundsVisitor;
        impl<'a> Visitor<'a> for AnnotationDisplayBoundsVisitor {
            type Value = AnnotationDisplayBounds;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut height: Option<f64> = None;
                let mut width: Option<f64> = None;
                let mut x: Option<f64> = None;
                let mut y: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "height" => {
                            if v.is_null() {
                                continue;
                            }
                            height = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "width" => {
                            if v.is_null() {
                                continue;
                            }
                            width = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "x" => {
                            if v.is_null() {
                                continue;
                            }
                            x = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "y" => {
                            if v.is_null() {
                                continue;
                            }
                            y = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = AnnotationDisplayBounds {
                    height,
                    width,
                    x,
                    y,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AnnotationDisplayBoundsVisitor)
    }
}
