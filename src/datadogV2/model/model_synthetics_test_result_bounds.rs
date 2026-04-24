// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Bounding box of an element on the page.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultBounds {
    /// Height in pixels.
    #[serde(rename = "height")]
    pub height: Option<i64>,
    /// Width in pixels.
    #[serde(rename = "width")]
    pub width: Option<i64>,
    /// Horizontal position in pixels.
    #[serde(rename = "x")]
    pub x: Option<i64>,
    /// Vertical position in pixels.
    #[serde(rename = "y")]
    pub y: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultBounds {
    pub fn new() -> SyntheticsTestResultBounds {
        SyntheticsTestResultBounds {
            height: None,
            width: None,
            x: None,
            y: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    pub fn x(mut self, value: i64) -> Self {
        self.x = Some(value);
        self
    }

    pub fn y(mut self, value: i64) -> Self {
        self.y = Some(value);
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

impl Default for SyntheticsTestResultBounds {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultBounds {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultBoundsVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultBoundsVisitor {
            type Value = SyntheticsTestResultBounds;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut height: Option<i64> = None;
                let mut width: Option<i64> = None;
                let mut x: Option<i64> = None;
                let mut y: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultBounds {
                    height,
                    width,
                    x,
                    y,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultBoundsVisitor)
    }
}
