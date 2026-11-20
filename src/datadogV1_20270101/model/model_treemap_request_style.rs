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
pub struct TreemapRequestStyle {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "palette")]
    pub palette: Option<crate::datadogV1_20270101::model::TreemapRequestStylePalette>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TreemapRequestStyle {
    pub fn new() -> TreemapRequestStyle {
        TreemapRequestStyle {
            palette: None,
            _unparsed: false,
        }
    }
    
    pub fn palette(mut self, value: crate::datadogV1_20270101::model::TreemapRequestStylePalette) -> Self {
        self.palette = Some(value);
        self
    }
    
}


impl Default for TreemapRequestStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TreemapRequestStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TreemapRequestStyleVisitor;
        impl<'a> Visitor<'a> for TreemapRequestStyleVisitor {
            type Value = TreemapRequestStyle;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut palette: Option<crate::datadogV1_20270101::model::TreemapRequestStylePalette> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "palette" => {
                            if v.is_null() {
                                continue;
                            }
                            palette = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _palette) = palette {
                                match _palette {
                                    crate::datadogV1_20270101::model::TreemapRequestStylePalette::UnparsedObject(_palette) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }

                let content = TreemapRequestStyle {
                    palette,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TreemapRequestStyleVisitor)
    }
}
