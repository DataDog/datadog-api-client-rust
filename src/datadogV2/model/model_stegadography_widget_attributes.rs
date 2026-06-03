// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a watermarked widget recovered from an image.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct StegadographyWidgetAttributes {
    /// Horizontal pixel coordinate where the watermark was found in the image.
    #[serde(rename = "locationx")]
    pub locationx: i64,
    /// Vertical pixel coordinate where the watermark was found in the image.
    #[serde(rename = "locationy")]
    pub locationy: i64,
    /// JSON-encoded string representing the widget state.
    #[serde(rename = "rawData")]
    pub raw_data: String,
    /// Hex-encoded watermark string identifying the widget.
    #[serde(rename = "watermark")]
    pub watermark: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl StegadographyWidgetAttributes {
    pub fn new(
        locationx: i64,
        locationy: i64,
        raw_data: String,
        watermark: String,
    ) -> StegadographyWidgetAttributes {
        StegadographyWidgetAttributes {
            locationx,
            locationy,
            raw_data,
            watermark,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for StegadographyWidgetAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StegadographyWidgetAttributesVisitor;
        impl<'a> Visitor<'a> for StegadographyWidgetAttributesVisitor {
            type Value = StegadographyWidgetAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut locationx: Option<i64> = None;
                let mut locationy: Option<i64> = None;
                let mut raw_data: Option<String> = None;
                let mut watermark: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "locationx" => {
                            locationx = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "locationy" => {
                            locationy = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rawData" => {
                            raw_data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "watermark" => {
                            watermark = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let locationx = locationx.ok_or_else(|| M::Error::missing_field("locationx"))?;
                let locationy = locationy.ok_or_else(|| M::Error::missing_field("locationy"))?;
                let raw_data = raw_data.ok_or_else(|| M::Error::missing_field("raw_data"))?;
                let watermark = watermark.ok_or_else(|| M::Error::missing_field("watermark"))?;

                let content = StegadographyWidgetAttributes {
                    locationx,
                    locationy,
                    raw_data,
                    watermark,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(StegadographyWidgetAttributesVisitor)
    }
}
