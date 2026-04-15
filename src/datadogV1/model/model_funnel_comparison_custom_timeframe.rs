// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Custom timeframe for funnel comparison.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FunnelComparisonCustomTimeframe {
    /// Start of the custom timeframe.
    #[serde(rename = "from")]
    pub from: f64,
    /// End of the custom timeframe.
    #[serde(rename = "to")]
    pub to: f64,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FunnelComparisonCustomTimeframe {
    pub fn new(from: f64, to: f64) -> FunnelComparisonCustomTimeframe {
        FunnelComparisonCustomTimeframe {
            from,
            to,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for FunnelComparisonCustomTimeframe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FunnelComparisonCustomTimeframeVisitor;
        impl<'a> Visitor<'a> for FunnelComparisonCustomTimeframeVisitor {
            type Value = FunnelComparisonCustomTimeframe;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut from: Option<f64> = None;
                let mut to: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "from" => {
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to" => {
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let from = from.ok_or_else(|| M::Error::missing_field("from"))?;
                let to = to.ok_or_else(|| M::Error::missing_field("to"))?;

                let content = FunnelComparisonCustomTimeframe {
                    from,
                    to,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FunnelComparisonCustomTimeframeVisitor)
    }
}
