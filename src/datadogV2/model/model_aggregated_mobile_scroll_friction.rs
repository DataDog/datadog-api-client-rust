// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Aggregated mobile scroll friction detection at view level.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AggregatedMobileScrollFriction {
    /// Average number of frozen frames during scroll interactions.
    #[serde(rename = "avg_scroll_frozen_frame_count")]
    pub avg_scroll_frozen_frame_count: i32,
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

impl AggregatedMobileScrollFriction {
    pub fn new(
        avg_scroll_frozen_frame_count: i32,
        fingerprint: String,
        impact_score: f64,
        view_occurrences: i32,
    ) -> AggregatedMobileScrollFriction {
        AggregatedMobileScrollFriction {
            avg_scroll_frozen_frame_count,
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

impl<'de> Deserialize<'de> for AggregatedMobileScrollFriction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AggregatedMobileScrollFrictionVisitor;
        impl<'a> Visitor<'a> for AggregatedMobileScrollFrictionVisitor {
            type Value = AggregatedMobileScrollFriction;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut avg_scroll_frozen_frame_count: Option<i32> = None;
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
                        "avg_scroll_frozen_frame_count" => {
                            avg_scroll_frozen_frame_count =
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
                let avg_scroll_frozen_frame_count = avg_scroll_frozen_frame_count
                    .ok_or_else(|| M::Error::missing_field("avg_scroll_frozen_frame_count"))?;
                let fingerprint =
                    fingerprint.ok_or_else(|| M::Error::missing_field("fingerprint"))?;
                let impact_score =
                    impact_score.ok_or_else(|| M::Error::missing_field("impact_score"))?;
                let view_occurrences =
                    view_occurrences.ok_or_else(|| M::Error::missing_field("view_occurrences"))?;

                let content = AggregatedMobileScrollFriction {
                    avg_scroll_frozen_frame_count,
                    fingerprint,
                    impact_score,
                    view_occurrences,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AggregatedMobileScrollFrictionVisitor)
    }
}
