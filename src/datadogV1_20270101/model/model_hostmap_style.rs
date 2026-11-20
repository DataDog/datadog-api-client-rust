// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HostmapStyle {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "fill_max")]
    pub fill_max: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "fill_min")]
    pub fill_min: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "palette")]
    pub palette: Option<crate::datadogV1_20270101::model::HostmapStylePalette>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "palette_flip")]
    pub palette_flip: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl HostmapStyle {
    pub fn new() -> HostmapStyle {
        HostmapStyle {
            fill_max: None,
            fill_min: None,
            palette: None,
            palette_flip: None,
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
    
    pub fn palette(mut self, value: crate::datadogV1_20270101::model::HostmapStylePalette) -> Self {
        self.palette = Some(value);
        self
    }
    
    pub fn palette_flip(mut self, value: bool) -> Self {
        self.palette_flip = Some(value);
        self
    }
    
}


impl Default for HostmapStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HostmapStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostmapStyleVisitor;
        impl<'a> Visitor<'a> for HostmapStyleVisitor {
            type Value = HostmapStyle;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut fill_max: Option<f64> = None;
                let mut fill_min: Option<f64> = None;
                let mut palette: Option<crate::datadogV1_20270101::model::HostmapStylePalette> = None;
                let mut palette_flip: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "fill_max" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            fill_max = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "fill_min" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            fill_min = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "palette" => {
                            if v.is_null() {
                                continue;
                            }
                            palette = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _palette) = palette {
                                match _palette {
                                    crate::datadogV1_20270101::model::HostmapStylePalette::UnparsedObject(_palette) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "palette_flip" => {
                            if v.is_null() {
                                continue;
                            }
                            palette_flip = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }

                let content = HostmapStyle {
                    fill_max,
                    fill_min,
                    palette,
                    palette_flip,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HostmapStyleVisitor)
    }
}
