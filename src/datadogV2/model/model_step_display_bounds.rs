// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `StepDisplayBounds` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct StepDisplayBounds {
    /// The `bounds` `x`.
    #[serde(rename = "x")]
    pub x: Option<f64>,
    /// The `bounds` `y`.
    #[serde(rename = "y")]
    pub y: Option<f64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl StepDisplayBounds {
    pub fn new() -> StepDisplayBounds {
        StepDisplayBounds {
            x: None,
            y: None,
            _unparsed: false,
        }
    }

    pub fn x(mut self, value: f64) -> Self {
        self.x = Some(value);
        self
    }

    pub fn y(mut self, value: f64) -> Self {
        self.y = Some(value);
        self
    }
}

impl Default for StepDisplayBounds {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for StepDisplayBounds {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StepDisplayBoundsVisitor;
        impl<'a> Visitor<'a> for StepDisplayBoundsVisitor {
            type Value = StepDisplayBounds;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut x: Option<f64> = None;
                let mut y: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "x" => {
                            if v.is_null() {
                                continue;
                            }
                            x = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "y" => {
                            if v.is_null() {
                                continue;
                            }
                            y = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = StepDisplayBounds { x, y, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(StepDisplayBoundsVisitor)
    }
}
