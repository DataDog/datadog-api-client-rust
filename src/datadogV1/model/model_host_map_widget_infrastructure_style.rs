// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Style configuration for the infrastructure host map.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HostMapWidgetInfrastructureStyle {
    /// Maximum value for the fill color scale. Omit to use automatic scaling.
    #[serde(rename = "fill_max")]
    pub fill_max: Option<f64>,
    /// Minimum value for the fill color scale. Omit to use automatic scaling.
    #[serde(rename = "fill_min")]
    pub fill_min: Option<f64>,
    /// Color palette name or alias.
    #[serde(rename = "palette")]
    pub palette: Option<String>,
    /// Whether to invert the color palette.
    #[serde(rename = "palette_flip")]
    pub palette_flip: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HostMapWidgetInfrastructureStyle {
    pub fn new() -> HostMapWidgetInfrastructureStyle {
        HostMapWidgetInfrastructureStyle {
            fill_max: None,
            fill_min: None,
            palette: None,
            palette_flip: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn fill_max(mut self, value: f64) -> Self {
        self.fill_max = Some(value);
        self
    }

    pub fn fill_min(mut self, value: f64) -> Self {
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for HostMapWidgetInfrastructureStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HostMapWidgetInfrastructureStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostMapWidgetInfrastructureStyleVisitor;
        impl<'a> Visitor<'a> for HostMapWidgetInfrastructureStyleVisitor {
            type Value = HostMapWidgetInfrastructureStyle;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut fill_max: Option<f64> = None;
                let mut fill_min: Option<f64> = None;
                let mut palette: Option<String> = None;
                let mut palette_flip: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "fill_max" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            fill_max = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fill_min" => {
                            if v.is_null() || v.as_str() == Some("") {
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = HostMapWidgetInfrastructureStyle {
                    fill_max,
                    fill_min,
                    palette,
                    palette_flip,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HostMapWidgetInfrastructureStyleVisitor)
    }
}
