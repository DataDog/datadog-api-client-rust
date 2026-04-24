// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Screen resolution of the device used to run the test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultDeviceResolution {
    /// Viewport height in pixels.
    #[serde(rename = "height")]
    pub height: Option<i64>,
    /// Device pixel ratio.
    #[serde(rename = "pixel_ratio")]
    pub pixel_ratio: Option<f64>,
    /// Viewport width in pixels.
    #[serde(rename = "width")]
    pub width: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultDeviceResolution {
    pub fn new() -> SyntheticsTestResultDeviceResolution {
        SyntheticsTestResultDeviceResolution {
            height: None,
            pixel_ratio: None,
            width: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn pixel_ratio(mut self, value: f64) -> Self {
        self.pixel_ratio = Some(value);
        self
    }

    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
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

impl Default for SyntheticsTestResultDeviceResolution {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultDeviceResolution {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultDeviceResolutionVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultDeviceResolutionVisitor {
            type Value = SyntheticsTestResultDeviceResolution;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut height: Option<i64> = None;
                let mut pixel_ratio: Option<f64> = None;
                let mut width: Option<i64> = None;
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
                        "pixel_ratio" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            pixel_ratio =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "width" => {
                            if v.is_null() {
                                continue;
                            }
                            width = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultDeviceResolution {
                    height,
                    pixel_ratio,
                    width,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultDeviceResolutionVisitor)
    }
}
