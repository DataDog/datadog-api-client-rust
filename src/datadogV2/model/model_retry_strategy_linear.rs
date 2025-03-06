// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `RetryStrategyLinear` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RetryStrategyLinear {
    /// The `RetryStrategyLinear` `interval`. The expected format is the number of seconds ending with an s. For example, 1 day is 86400s
    #[serde(rename = "interval")]
    pub interval: String,
    /// The `RetryStrategyLinear` `maxRetries`.
    #[serde(rename = "maxRetries")]
    pub max_retries: f64,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RetryStrategyLinear {
    pub fn new(interval: String, max_retries: f64) -> RetryStrategyLinear {
        RetryStrategyLinear {
            interval,
            max_retries,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for RetryStrategyLinear {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RetryStrategyLinearVisitor;
        impl<'a> Visitor<'a> for RetryStrategyLinearVisitor {
            type Value = RetryStrategyLinear;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut interval: Option<String> = None;
                let mut max_retries: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "interval" => {
                            interval = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "maxRetries" => {
                            max_retries =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let interval = interval.ok_or_else(|| M::Error::missing_field("interval"))?;
                let max_retries =
                    max_retries.ok_or_else(|| M::Error::missing_field("max_retries"))?;

                let content = RetryStrategyLinear {
                    interval,
                    max_retries,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RetryStrategyLinearVisitor)
    }
}
