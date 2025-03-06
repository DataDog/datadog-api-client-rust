// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines a rate limit for a trigger.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TriggerRateLimit {
    /// The `TriggerRateLimit` `count`.
    #[serde(rename = "count")]
    pub count: Option<i64>,
    /// The `TriggerRateLimit` `interval`. The expected format is the number of seconds ending with an s. For example, 1 day is 86400s
    #[serde(rename = "interval")]
    pub interval: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TriggerRateLimit {
    pub fn new() -> TriggerRateLimit {
        TriggerRateLimit {
            count: None,
            interval: None,
            _unparsed: false,
        }
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn interval(mut self, value: String) -> Self {
        self.interval = Some(value);
        self
    }
}

impl Default for TriggerRateLimit {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TriggerRateLimit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TriggerRateLimitVisitor;
        impl<'a> Visitor<'a> for TriggerRateLimitVisitor {
            type Value = TriggerRateLimit;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut count: Option<i64> = None;
                let mut interval: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "count" => {
                            if v.is_null() {
                                continue;
                            }
                            count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "interval" => {
                            if v.is_null() {
                                continue;
                            }
                            interval = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = TriggerRateLimit {
                    count,
                    interval,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TriggerRateLimitVisitor)
    }
}
