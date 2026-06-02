// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Aggregated slow interaction with long task detection grouped by action and selector.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AggregatedSlowInteractionLongTask {
    /// Type of user interaction that triggered the slow response.
    #[serde(rename = "action_type")]
    pub action_type: String,
    /// Average long task blocking duration in nanoseconds.
    #[serde(rename = "avg_blocking_duration")]
    pub avg_blocking_duration: i64,
    /// Average total interaction duration in nanoseconds.
    #[serde(rename = "avg_duration")]
    pub avg_duration: i64,
    /// Unique fingerprint identifying this detection group.
    #[serde(rename = "fingerprint")]
    pub fingerprint: String,
    /// Impact score combining view frequency and blocking severity.
    #[serde(rename = "impact_score")]
    pub impact_score: f64,
    /// Total number of detection instances across sampled views.
    #[serde(rename = "instance_count")]
    pub instance_count: i32,
    /// CSS selector of the element that was interacted with.
    #[serialize_always]
    #[serde(rename = "selector")]
    pub selector: Option<String>,
    /// Normalized CSS selector with dynamic parts replaced.
    #[serialize_always]
    #[serde(rename = "selector_normalized")]
    pub selector_normalized: Option<String>,
    /// Number of sampled views where this detection occurred.
    #[serde(rename = "view_occurrences")]
    pub view_occurrences: i32,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AggregatedSlowInteractionLongTask {
    pub fn new(
        action_type: String,
        avg_blocking_duration: i64,
        avg_duration: i64,
        fingerprint: String,
        impact_score: f64,
        instance_count: i32,
        selector: Option<String>,
        selector_normalized: Option<String>,
        view_occurrences: i32,
    ) -> AggregatedSlowInteractionLongTask {
        AggregatedSlowInteractionLongTask {
            action_type,
            avg_blocking_duration,
            avg_duration,
            fingerprint,
            impact_score,
            instance_count,
            selector,
            selector_normalized,
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

impl<'de> Deserialize<'de> for AggregatedSlowInteractionLongTask {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AggregatedSlowInteractionLongTaskVisitor;
        impl<'a> Visitor<'a> for AggregatedSlowInteractionLongTaskVisitor {
            type Value = AggregatedSlowInteractionLongTask;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action_type: Option<String> = None;
                let mut avg_blocking_duration: Option<i64> = None;
                let mut avg_duration: Option<i64> = None;
                let mut fingerprint: Option<String> = None;
                let mut impact_score: Option<f64> = None;
                let mut instance_count: Option<i32> = None;
                let mut selector: Option<Option<String>> = None;
                let mut selector_normalized: Option<Option<String>> = None;
                let mut view_occurrences: Option<i32> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action_type" => {
                            action_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "avg_blocking_duration" => {
                            avg_blocking_duration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "avg_duration" => {
                            avg_duration =
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
                        "instance_count" => {
                            instance_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "selector" => {
                            selector = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "selector_normalized" => {
                            selector_normalized =
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
                let action_type =
                    action_type.ok_or_else(|| M::Error::missing_field("action_type"))?;
                let avg_blocking_duration = avg_blocking_duration
                    .ok_or_else(|| M::Error::missing_field("avg_blocking_duration"))?;
                let avg_duration =
                    avg_duration.ok_or_else(|| M::Error::missing_field("avg_duration"))?;
                let fingerprint =
                    fingerprint.ok_or_else(|| M::Error::missing_field("fingerprint"))?;
                let impact_score =
                    impact_score.ok_or_else(|| M::Error::missing_field("impact_score"))?;
                let instance_count =
                    instance_count.ok_or_else(|| M::Error::missing_field("instance_count"))?;
                let selector = selector.ok_or_else(|| M::Error::missing_field("selector"))?;
                let selector_normalized = selector_normalized
                    .ok_or_else(|| M::Error::missing_field("selector_normalized"))?;
                let view_occurrences =
                    view_occurrences.ok_or_else(|| M::Error::missing_field("view_occurrences"))?;

                let content = AggregatedSlowInteractionLongTask {
                    action_type,
                    avg_blocking_duration,
                    avg_duration,
                    fingerprint,
                    impact_score,
                    instance_count,
                    selector,
                    selector_normalized,
                    view_occurrences,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AggregatedSlowInteractionLongTaskVisitor)
    }
}
