// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Most anomalous point within the detected interval.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TimeseriesAnomalyInvestigationMaximumDeviation {
    /// Absolute distance between the observed value and the nearest anomaly boundary.
    #[serde(rename = "delta_from_boundary")]
    pub delta_from_boundary: f64,
    /// Point timestamp in milliseconds since the Unix epoch.
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    /// Observed value at the point.
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TimeseriesAnomalyInvestigationMaximumDeviation {
    pub fn new(
        delta_from_boundary: f64,
        timestamp: i64,
        value: f64,
    ) -> TimeseriesAnomalyInvestigationMaximumDeviation {
        TimeseriesAnomalyInvestigationMaximumDeviation {
            delta_from_boundary,
            timestamp,
            value,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for TimeseriesAnomalyInvestigationMaximumDeviation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeseriesAnomalyInvestigationMaximumDeviationVisitor;
        impl<'a> Visitor<'a> for TimeseriesAnomalyInvestigationMaximumDeviationVisitor {
            type Value = TimeseriesAnomalyInvestigationMaximumDeviation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut delta_from_boundary: Option<f64> = None;
                let mut timestamp: Option<i64> = None;
                let mut value: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "delta_from_boundary" => {
                            delta_from_boundary =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp" => {
                            timestamp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let delta_from_boundary = delta_from_boundary
                    .ok_or_else(|| M::Error::missing_field("delta_from_boundary"))?;
                let timestamp = timestamp.ok_or_else(|| M::Error::missing_field("timestamp"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = TimeseriesAnomalyInvestigationMaximumDeviation {
                    delta_from_boundary,
                    timestamp,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeseriesAnomalyInvestigationMaximumDeviationVisitor)
    }
}
