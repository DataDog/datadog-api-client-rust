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
pub struct GeomapStyle {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "palette")]
    pub palette: crate::datadogV1_20270101::model::GeomapStylePalette,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "palette_flip")]
    pub palette_flip: bool,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl GeomapStyle {
    pub fn new(palette: crate::datadogV1_20270101::model::GeomapStylePalette, palette_flip: bool) -> GeomapStyle {
        GeomapStyle {
            palette,
            palette_flip,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for GeomapStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GeomapStyleVisitor;
        impl<'a> Visitor<'a> for GeomapStyleVisitor {
            type Value = GeomapStyle;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut palette: Option<crate::datadogV1_20270101::model::GeomapStylePalette> = None;
                let mut palette_flip: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "palette" => {
                            palette = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _palette) = palette {
                                match _palette {
                                    crate::datadogV1_20270101::model::GeomapStylePalette::UnparsedObject(_palette) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "palette_flip" => {
                            palette_flip = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let palette = palette.ok_or_else(|| M::Error::missing_field("palette"))?;
                let palette_flip = palette_flip.ok_or_else(|| M::Error::missing_field("palette_flip"))?;

                let content = GeomapStyle {
                    palette,
                    palette_flip,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GeomapStyleVisitor)
    }
}
