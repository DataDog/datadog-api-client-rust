// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing options to override the default daily limit reset time.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsDailyLimitReset {
    /// String in `HH:00` format representing the time of day the daily limit should be reset. The hours must be between 00 and 23 (inclusive).
    #[serde(rename = "reset_time")]
    pub reset_time: Option<String>,
    /// String in `(-|+)HH:00` format representing the UTC offset to apply to the given reset time. The hours must be between -12 and +14 (inclusive).
    #[serde(rename = "reset_utc_offset")]
    pub reset_utc_offset: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsDailyLimitReset {
    pub fn new() -> LogsDailyLimitReset {
        LogsDailyLimitReset {
            reset_time: None,
            reset_utc_offset: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn reset_time(mut self, value: String) -> Self {
        self.reset_time = Some(value);
        self
    }

    pub fn reset_utc_offset(mut self, value: String) -> Self {
        self.reset_utc_offset = Some(value);
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

impl Default for LogsDailyLimitReset {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LogsDailyLimitReset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsDailyLimitResetVisitor;
        impl<'a> Visitor<'a> for LogsDailyLimitResetVisitor {
            type Value = LogsDailyLimitReset;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut reset_time: Option<String> = None;
                let mut reset_utc_offset: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "reset_time" => {
                            if v.is_null() {
                                continue;
                            }
                            reset_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reset_utc_offset" => {
                            if v.is_null() {
                                continue;
                            }
                            reset_utc_offset =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LogsDailyLimitReset {
                    reset_time,
                    reset_utc_offset,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsDailyLimitResetVisitor)
    }
}
