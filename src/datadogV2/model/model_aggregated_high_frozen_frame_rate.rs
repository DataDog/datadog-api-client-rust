// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Aggregated high frozen frame rate detection at view level.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AggregatedHighFrozenFrameRate {
    /// Average frozen frame rate as a fraction of total frames.
    #[serde(rename = "avg_frozen_frame_rate")]
    pub avg_frozen_frame_rate: f64,
    /// Average segment duration in nanoseconds.
    #[serde(rename = "avg_segment_duration")]
    pub avg_segment_duration: i64,
    /// Average total frozen duration in nanoseconds.
    #[serde(rename = "avg_total_frozen_duration")]
    pub avg_total_frozen_duration: i64,
    /// Unique fingerprint identifying this detection group.
    #[serde(rename = "fingerprint")]
    pub fingerprint: String,
    /// Impact score for this detection.
    #[serde(rename = "impact_score")]
    pub impact_score: f64,
    /// Number of sampled views where this detection occurred.
    #[serde(rename = "view_occurrences")]
    pub view_occurrences: i32,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AggregatedHighFrozenFrameRate {
    pub fn new(
        avg_frozen_frame_rate: f64,
        avg_segment_duration: i64,
        avg_total_frozen_duration: i64,
        fingerprint: String,
        impact_score: f64,
        view_occurrences: i32,
    ) -> AggregatedHighFrozenFrameRate {
        AggregatedHighFrozenFrameRate {
            avg_frozen_frame_rate,
            avg_segment_duration,
            avg_total_frozen_duration,
            fingerprint,
            impact_score,
            view_occurrences,
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

impl<'de> Deserialize<'de> for AggregatedHighFrozenFrameRate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AggregatedHighFrozenFrameRateVisitor;
        impl<'a> Visitor<'a> for AggregatedHighFrozenFrameRateVisitor {
            type Value = AggregatedHighFrozenFrameRate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut avg_frozen_frame_rate: Option<f64> = None;
                let mut avg_segment_duration: Option<i64> = None;
                let mut avg_total_frozen_duration: Option<i64> = None;
                let mut fingerprint: Option<String> = None;
                let mut impact_score: Option<f64> = None;
                let mut view_occurrences: Option<i32> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "avg_frozen_frame_rate" => {
                            avg_frozen_frame_rate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "avg_segment_duration" => {
                            avg_segment_duration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "avg_total_frozen_duration" => {
                            avg_total_frozen_duration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fingerprint" => {
                            fingerprint =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "impact_score" => {
                            impact_score =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "view_occurrences" => {
                            view_occurrences =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let avg_frozen_frame_rate = avg_frozen_frame_rate
                    .ok_or_else(|| M::Error::missing_field("avg_frozen_frame_rate"))?;
                let avg_segment_duration = avg_segment_duration
                    .ok_or_else(|| M::Error::missing_field("avg_segment_duration"))?;
                let avg_total_frozen_duration = avg_total_frozen_duration
                    .ok_or_else(|| M::Error::missing_field("avg_total_frozen_duration"))?;
                let fingerprint =
                    fingerprint.ok_or_else(|| M::Error::missing_field("fingerprint"))?;
                let impact_score =
                    impact_score.ok_or_else(|| M::Error::missing_field("impact_score"))?;
                let view_occurrences =
                    view_occurrences.ok_or_else(|| M::Error::missing_field("view_occurrences"))?;

                let content = AggregatedHighFrozenFrameRate {
                    avg_frozen_frame_rate,
                    avg_segment_duration,
                    avg_total_frozen_duration,
                    fingerprint,
                    impact_score,
                    view_occurrences,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AggregatedHighFrozenFrameRateVisitor)
    }
}
