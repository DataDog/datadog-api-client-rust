// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The object describing a timeseries response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TimeseriesResponseAttributes {
    /// Array of response series. The index here corresponds to the index in the `formulas` or `queries` array from the request.
    #[serde(rename = "series")]
    pub series: Option<Vec<crate::datadogV2::model::TimeseriesResponseSeries>>,
    /// Array of times, 1-1 match with individual values arrays.
    #[serde(rename = "times")]
    pub times: Option<Vec<i64>>,
    /// Array of value-arrays. The index here corresponds to the index in the `formulas` or `queries` array from the request.
    #[serde(rename = "values")]
    pub values: Option<Vec<Vec<Option<f64>>>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TimeseriesResponseAttributes {
    pub fn new() -> TimeseriesResponseAttributes {
        TimeseriesResponseAttributes {
            series: None,
            times: None,
            values: None,
            _unparsed: false,
        }
    }

    pub fn series(mut self, value: Vec<crate::datadogV2::model::TimeseriesResponseSeries>) -> Self {
        self.series = Some(value);
        self
    }

    pub fn times(mut self, value: Vec<i64>) -> Self {
        self.times = Some(value);
        self
    }

    pub fn values(mut self, value: Vec<Vec<Option<f64>>>) -> Self {
        self.values = Some(value);
        self
    }
}

impl Default for TimeseriesResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TimeseriesResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeseriesResponseAttributesVisitor;
        impl<'a> Visitor<'a> for TimeseriesResponseAttributesVisitor {
            type Value = TimeseriesResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut series: Option<Vec<crate::datadogV2::model::TimeseriesResponseSeries>> =
                    None;
                let mut times: Option<Vec<i64>> = None;
                let mut values: Option<Vec<Vec<Option<f64>>>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "series" => {
                            if v.is_null() {
                                continue;
                            }
                            series = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "times" => {
                            if v.is_null() {
                                continue;
                            }
                            times = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "values" => {
                            if v.is_null() {
                                continue;
                            }
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = TimeseriesResponseAttributes {
                    series,
                    times,
                    values,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeseriesResponseAttributesVisitor)
    }
}
