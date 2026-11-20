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
pub struct GeomapRequestStyle {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "color_by")]
    pub color_by: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "palette")]
    pub palette: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl GeomapRequestStyle {
    pub fn new() -> GeomapRequestStyle {
        GeomapRequestStyle {
            color_by: None,
            palette: None,
            _unparsed: false,
        }
    }
    
    pub fn color_by(mut self, value: String) -> Self {
        self.color_by = Some(value);
        self
    }
    
    pub fn palette(mut self, value: String) -> Self {
        self.palette = Some(value);
        self
    }
    
}


impl Default for GeomapRequestStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GeomapRequestStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GeomapRequestStyleVisitor;
        impl<'a> Visitor<'a> for GeomapRequestStyleVisitor {
            type Value = GeomapRequestStyle;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut color_by: Option<String> = None;
                let mut palette: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "color_by" => {
                            if v.is_null() {
                                continue;
                            }
                            color_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "palette" => {
                            if v.is_null() {
                                continue;
                            }
                            palette = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }

                let content = GeomapRequestStyle {
                    color_by,
                    palette,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GeomapRequestStyleVisitor)
    }
}
