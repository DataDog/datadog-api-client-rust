// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Define computation for a log query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsQueryCompute {
    /// The aggregation method.
    #[serde(rename = "aggregation")]
    pub aggregation: String,
    /// Facet name.
    #[serde(rename = "facet")]
    pub facet: Option<String>,
    /// Define a time interval in seconds.
    #[serde(rename = "interval")]
    pub interval: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsQueryCompute {
    pub fn new(aggregation: String) -> LogsQueryCompute {
        LogsQueryCompute {
            aggregation,
            facet: None,
            interval: None,
            _unparsed: false,
        }
    }

    pub fn facet(mut self, value: String) -> Self {
        self.facet = Some(value);
        self
    }

    pub fn interval(mut self, value: i64) -> Self {
        self.interval = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for LogsQueryCompute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsQueryComputeVisitor;
        impl<'a> Visitor<'a> for LogsQueryComputeVisitor {
            type Value = LogsQueryCompute;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation: Option<String> = None;
                let mut facet: Option<String> = None;
                let mut interval: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation" => {
                            aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "facet" => {
                            if v.is_null() {
                                continue;
                            }
                            facet = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "interval" => {
                            if v.is_null() {
                                continue;
                            }
                            interval = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let aggregation =
                    aggregation.ok_or_else(|| M::Error::missing_field("aggregation"))?;

                let content = LogsQueryCompute {
                    aggregation,
                    facet,
                    interval,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsQueryComputeVisitor)
    }
}
