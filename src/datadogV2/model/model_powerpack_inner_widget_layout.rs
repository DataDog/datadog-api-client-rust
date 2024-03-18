// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Powerpack inner widget layout.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PowerpackInnerWidgetLayout {
    /// The height of the widget. Should be a non-negative integer.
    #[serde(rename = "height")]
    pub height: i64,
    /// The width of the widget. Should be a non-negative integer.
    #[serde(rename = "width")]
    pub width: i64,
    /// The position of the widget on the x (horizontal) axis. Should be a non-negative integer.
    #[serde(rename = "x")]
    pub x: i64,
    /// The position of the widget on the y (vertical) axis. Should be a non-negative integer.
    #[serde(rename = "y")]
    pub y: i64,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PowerpackInnerWidgetLayout {
    pub fn new(height: i64, width: i64, x: i64, y: i64) -> PowerpackInnerWidgetLayout {
        PowerpackInnerWidgetLayout {
            height,
            width,
            x,
            y,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for PowerpackInnerWidgetLayout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PowerpackInnerWidgetLayoutVisitor;
        impl<'a> Visitor<'a> for PowerpackInnerWidgetLayoutVisitor {
            type Value = PowerpackInnerWidgetLayout;

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
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "height" => {
                            height = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "width" => {
                            width = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "x" => {
                            x = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "y" => {
                            y = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let height = height.ok_or_else(|| M::Error::missing_field("height"))?;
                let width = width.ok_or_else(|| M::Error::missing_field("width"))?;
                let x = x.ok_or_else(|| M::Error::missing_field("x"))?;
                let y = y.ok_or_else(|| M::Error::missing_field("y"))?;

                let content = PowerpackInnerWidgetLayout {
                    height,
                    width,
                    x,
                    y,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PowerpackInnerWidgetLayoutVisitor)
    }
}
