// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A metric to submit to Datadog.
/// See [Datadog metrics](<https://docs.datadoghq.com/developers/metrics/#custom-metrics-properties>).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Series {
    /// The name of the host that produced the metric.
    #[serde(rename = "host")]
    pub host: Option<String>,
    /// If the type of the metric is rate or count, define the corresponding interval.
    #[serde(
        rename = "interval",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub interval: Option<Option<i64>>,
    /// The name of the timeseries.
    #[serde(rename = "metric")]
    pub metric: String,
    /// Points relating to a metric. All points must be tuples with timestamp and a scalar value (cannot be a string). Timestamps should be in POSIX time in seconds, and cannot be more than ten minutes in the future or more than one hour in the past.
    #[serde(rename = "points")]
    pub points: Vec<Vec<Option<f64>>>,
    /// A list of tags associated with the metric.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The type of the metric. Valid types are "",`count`, `gauge`, and `rate`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl Series {
    pub fn new(metric: String, points: Vec<Vec<Option<f64>>>) -> Series {
        Series {
            host: None,
            interval: None,
            metric,
            points,
            tags: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn host(mut self, value: String) -> Self {
        self.host = Some(value);
        self
    }

    pub fn interval(mut self, value: Option<i64>) -> Self {
        self.interval = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for Series {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SeriesVisitor;
        impl<'a> Visitor<'a> for SeriesVisitor {
            type Value = Series;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut host: Option<String> = None;
                let mut interval: Option<Option<i64>> = None;
                let mut metric: Option<String> = None;
                let mut points: Option<Vec<Vec<Option<f64>>>> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut type_: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "host" => {
                            if v.is_null() {
                                continue;
                            }
                            host = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "interval" => {
                            interval = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric" => {
                            metric = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "points" => {
                            points = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let metric = metric.ok_or_else(|| M::Error::missing_field("metric"))?;
                let points = points.ok_or_else(|| M::Error::missing_field("points"))?;

                let content = Series {
                    host,
                    interval,
                    metric,
                    points,
                    tags,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SeriesVisitor)
    }
}
