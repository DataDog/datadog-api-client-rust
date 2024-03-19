// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A timeseries point.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RUMAggregateBucketValueTimeseriesPoint {
    /// The time value for this point.
    #[serde(rename = "time")]
    pub time: Option<chrono::DateTime<chrono::Utc>>,
    /// The value for this point.
    #[serde(rename = "value")]
    pub value: Option<f64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RUMAggregateBucketValueTimeseriesPoint {
    pub fn new() -> RUMAggregateBucketValueTimeseriesPoint {
        RUMAggregateBucketValueTimeseriesPoint {
            time: None,
            value: None,
            _unparsed: false,
        }
    }

    pub fn time(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.time = Some(value);
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }
}

impl Default for RUMAggregateBucketValueTimeseriesPoint {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RUMAggregateBucketValueTimeseriesPoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RUMAggregateBucketValueTimeseriesPointVisitor;
        impl<'a> Visitor<'a> for RUMAggregateBucketValueTimeseriesPointVisitor {
            type Value = RUMAggregateBucketValueTimeseriesPoint;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut time: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut value: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "time" => {
                            if v.is_null() {
                                continue;
                            }
                            time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = RUMAggregateBucketValueTimeseriesPoint {
                    time,
                    value,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RUMAggregateBucketValueTimeseriesPointVisitor)
    }
}
