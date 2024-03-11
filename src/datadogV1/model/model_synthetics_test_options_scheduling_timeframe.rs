// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing a timeframe.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestOptionsSchedulingTimeframe {
    /// Number representing the day of the week.
    #[serde(rename = "day")]
    pub day: Option<i32>,
    /// The hour of the day on which scheduling starts.
    #[serde(rename = "from")]
    pub from: Option<String>,
    /// The hour of the day on which scheduling ends.
    #[serde(rename = "to")]
    pub to: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestOptionsSchedulingTimeframe {
    pub fn new() -> SyntheticsTestOptionsSchedulingTimeframe {
        SyntheticsTestOptionsSchedulingTimeframe {
            day: None,
            from: None,
            to: None,
            _unparsed: false,
        }
    }

    pub fn day(&mut self, value: i32) -> &mut Self {
        self.day = Some(value);
        self
    }

    pub fn from(&mut self, value: String) -> &mut Self {
        self.from = Some(value);
        self
    }

    pub fn to(&mut self, value: String) -> &mut Self {
        self.to = Some(value);
        self
    }
}

impl Default for SyntheticsTestOptionsSchedulingTimeframe {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestOptionsSchedulingTimeframe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestOptionsSchedulingTimeframeVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestOptionsSchedulingTimeframeVisitor {
            type Value = SyntheticsTestOptionsSchedulingTimeframe;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut day: Option<i32> = None;
                let mut from: Option<String> = None;
                let mut to: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "day" => {
                            if v.is_null() {
                                continue;
                            }
                            day = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "from" => {
                            if v.is_null() {
                                continue;
                            }
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to" => {
                            if v.is_null() {
                                continue;
                            }
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsTestOptionsSchedulingTimeframe {
                    day,
                    from,
                    to,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestOptionsSchedulingTimeframeVisitor)
    }
}
