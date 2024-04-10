// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Usage timeseries data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageTimeSeriesObject {
    /// Datetime in ISO-8601 format, UTC. The hour for the usage.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /// Contains the number measured for the given usage_type during the hour.
    #[serde(rename = "value", default, with = "::serde_with::rust::double_option")]
    pub value: Option<Option<i64>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageTimeSeriesObject {
    pub fn new() -> UsageTimeSeriesObject {
        UsageTimeSeriesObject {
            timestamp: None,
            value: None,
            _unparsed: false,
        }
    }

    pub fn timestamp(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn value(mut self, value: Option<i64>) -> Self {
        self.value = Some(value);
        self
    }
}

impl Default for UsageTimeSeriesObject {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageTimeSeriesObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageTimeSeriesObjectVisitor;
        impl<'a> Visitor<'a> for UsageTimeSeriesObjectVisitor {
            type Value = UsageTimeSeriesObject;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut timestamp: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut value: Option<Option<i64>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            timestamp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UsageTimeSeriesObject {
                    timestamp,
                    value,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageTimeSeriesObjectVisitor)
    }
}
