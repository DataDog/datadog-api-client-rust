// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Position of the action relative to the element.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsMobileStepParamsElementRelativePosition {
    /// The `relativePosition` on the `x` axis for the element.
    #[serde(rename = "x")]
    pub x: Option<f64>,
    /// The `relativePosition` on the `y` axis for the element.
    #[serde(rename = "y")]
    pub y: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsMobileStepParamsElementRelativePosition {
    pub fn new() -> SyntheticsMobileStepParamsElementRelativePosition {
        SyntheticsMobileStepParamsElementRelativePosition {
            x: None,
            y: None,
            additional_properties: std::collections::BTreeMap::new(),
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for SyntheticsMobileStepParamsElementRelativePosition {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsMobileStepParamsElementRelativePosition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsMobileStepParamsElementRelativePositionVisitor;
        impl<'a> Visitor<'a> for SyntheticsMobileStepParamsElementRelativePositionVisitor {
            type Value = SyntheticsMobileStepParamsElementRelativePosition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut x: Option<f64> = None;
                let mut y: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsMobileStepParamsElementRelativePosition {
                    x,
                    y,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsMobileStepParamsElementRelativePositionVisitor)
    }
}