// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A timeseries metric containing timestamps, series values, and optional unit metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CommitmentsTimeseriesMetric {
    /// Timeseries data as a map of series names to their corresponding value arrays.
    #[serde(rename = "series")]
    pub series: std::collections::BTreeMap<String, Vec<f64>>,
    /// Unix timestamps in seconds for the timeseries data points.
    #[serde(rename = "times")]
    pub times: Vec<i64>,
    /// Unit metadata for a numeric metric.
    #[serde(rename = "unit")]
    pub unit: Option<crate::datadogV2::model::CommitmentsUnit>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CommitmentsTimeseriesMetric {
    pub fn new(
        series: std::collections::BTreeMap<String, Vec<f64>>,
        times: Vec<i64>,
    ) -> CommitmentsTimeseriesMetric {
        CommitmentsTimeseriesMetric {
            series,
            times,
            unit: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn unit(mut self, value: crate::datadogV2::model::CommitmentsUnit) -> Self {
        self.unit = Some(value);
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

impl<'de> Deserialize<'de> for CommitmentsTimeseriesMetric {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommitmentsTimeseriesMetricVisitor;
        impl<'a> Visitor<'a> for CommitmentsTimeseriesMetricVisitor {
            type Value = CommitmentsTimeseriesMetric;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut series: Option<std::collections::BTreeMap<String, Vec<f64>>> = None;
                let mut times: Option<Vec<i64>> = None;
                let mut unit: Option<crate::datadogV2::model::CommitmentsUnit> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "series" => {
                            series = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "times" => {
                            times = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "unit" => {
                            if v.is_null() {
                                continue;
                            }
                            unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let series = series.ok_or_else(|| M::Error::missing_field("series"))?;
                let times = times.ok_or_else(|| M::Error::missing_field("times"))?;

                let content = CommitmentsTimeseriesMetric {
                    series,
                    times,
                    unit,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CommitmentsTimeseriesMetricVisitor)
    }
}
