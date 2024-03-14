// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The style to apply to the widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GeomapWidgetDefinitionStyle {
    /// The color palette to apply to the widget.
    #[serde(rename = "palette")]
    pub palette: String,
    /// Whether to flip the palette tones.
    #[serde(rename = "palette_flip")]
    pub palette_flip: bool,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GeomapWidgetDefinitionStyle {
    pub fn new(palette: String, palette_flip: bool) -> GeomapWidgetDefinitionStyle {
        GeomapWidgetDefinitionStyle {
            palette,
            palette_flip,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for GeomapWidgetDefinitionStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GeomapWidgetDefinitionStyleVisitor;
        impl<'a> Visitor<'a> for GeomapWidgetDefinitionStyleVisitor {
            type Value = GeomapWidgetDefinitionStyle;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut palette: Option<String> = None;
                let mut palette_flip: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "palette" => {
                            palette = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "palette_flip" => {
                            palette_flip =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let palette = palette.ok_or_else(|| M::Error::missing_field("palette"))?;
                let palette_flip =
                    palette_flip.ok_or_else(|| M::Error::missing_field("palette_flip"))?;

                let content = GeomapWidgetDefinitionStyle {
                    palette,
                    palette_flip,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GeomapWidgetDefinitionStyleVisitor)
    }
}
