// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Exclusion filter is defined by a query, a sampling rule, and a active/inactive toggle.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsExclusionFilter {
    /// Default query is `*`, meaning all logs flowing in the index would be excluded.
    /// Scope down exclusion filter to only a subset of logs with a log query.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Sample rate to apply to logs going through this exclusion filter,
    /// a value of 1.0 excludes all logs matching the query.
    #[serde(rename = "sample_rate")]
    pub sample_rate: f64,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsExclusionFilter {
    pub fn new(sample_rate: f64) -> LogsExclusionFilter {
        LogsExclusionFilter {
            query: None,
            sample_rate,
            _unparsed: false,
        }
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for LogsExclusionFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsExclusionFilterVisitor;
        impl<'a> Visitor<'a> for LogsExclusionFilterVisitor {
            type Value = LogsExclusionFilter;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut query: Option<String> = None;
                let mut sample_rate: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sample_rate" => {
                            sample_rate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let sample_rate =
                    sample_rate.ok_or_else(|| M::Error::missing_field("sample_rate"))?;

                let content = LogsExclusionFilter {
                    query,
                    sample_rate,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsExclusionFilterVisitor)
    }
}
