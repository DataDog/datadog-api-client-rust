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
pub struct MSLWidgetLayout {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "height")]
    pub height: f64,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "parent", default, with = "::serde_with::rust::double_option")]
    pub parent: Option<Option<f64>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "width")]
    pub width: f64,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "x")]
    pub x: f64,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "y")]
    pub y: f64,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl MSLWidgetLayout {
    pub fn new(height: f64, width: f64, x: f64, y: f64) -> MSLWidgetLayout {
        MSLWidgetLayout {
            height,
            parent: None,
            width,
            x,
            y,
            _unparsed: false,
        }
    }
    
    pub fn parent(mut self, value: Option<f64>) -> Self {
        self.parent = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for MSLWidgetLayout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MSLWidgetLayoutVisitor;
        impl<'a> Visitor<'a> for MSLWidgetLayoutVisitor {
            type Value = MSLWidgetLayout;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut height: Option<f64> = None;
                let mut parent: Option<Option<f64>> = None;
                let mut width: Option<f64> = None;
                let mut x: Option<f64> = None;
                let mut y: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "height" => {
                            height = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "parent" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            parent = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "width" => {
                            width = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "x" => {
                            x = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "y" => {
                            y = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let height = height.ok_or_else(|| M::Error::missing_field("height"))?;
                let width = width.ok_or_else(|| M::Error::missing_field("width"))?;
                let x = x.ok_or_else(|| M::Error::missing_field("x"))?;
                let y = y.ok_or_else(|| M::Error::missing_field("y"))?;

                let content = MSLWidgetLayout {
                    height,
                    parent,
                    width,
                    x,
                    y,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MSLWidgetLayoutVisitor)
    }
}
