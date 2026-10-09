// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A position and color in a continuous gradient.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HeatgridColorStop {
    /// A color string, or two color strings for the light and dark themes, in that order.
    #[serde(rename = "color")]
    pub color: crate::datadogV1::model::HeatgridColor,
    /// Position in the gradient, from 0 to 100.
    #[serde(rename = "position")]
    pub position: i64,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HeatgridColorStop {
    pub fn new(color: crate::datadogV1::model::HeatgridColor, position: i64) -> HeatgridColorStop {
        HeatgridColorStop {
            color,
            position,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for HeatgridColorStop {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeatgridColorStopVisitor;
        impl<'a> Visitor<'a> for HeatgridColorStopVisitor {
            type Value = HeatgridColorStop;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut color: Option<crate::datadogV1::model::HeatgridColor> = None;
                let mut position: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "color" => {
                            color = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _color) = color {
                                match _color {
                                    crate::datadogV1::model::HeatgridColor::UnparsedObject(
                                        _color,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "position" => {
                            position = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let color = color.ok_or_else(|| M::Error::missing_field("color"))?;
                let position = position.ok_or_else(|| M::Error::missing_field("position"))?;

                let content = HeatgridColorStop {
                    color,
                    position,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HeatgridColorStopVisitor)
    }
}
