// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Aggregated slow first contentful paint with high byte count detection.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AggregatedSlowFCPHighBytes {
    /// Average total bytes loaded before first contentful paint.
    #[serde(rename = "avg_bytes_before_fcp_bytes")]
    pub avg_bytes_before_fcp_bytes: i64,
    /// Average first contentful paint time in milliseconds.
    #[serde(rename = "avg_first_contentful_paint_ms")]
    pub avg_first_contentful_paint_ms: i64,
    /// Average number of resources loaded before first contentful paint.
    #[serde(rename = "avg_resource_count_before_fcp")]
    pub avg_resource_count_before_fcp: i64,
    /// Unique fingerprint identifying this detection group.
    #[serde(rename = "fingerprint")]
    pub fingerprint: String,
    /// Impact score for this detection.
    #[serde(rename = "impact_score")]
    pub impact_score: f64,
    /// Platform identifier for the affected views.
    #[serde(rename = "platform")]
    pub platform: String,
    /// Number of sampled views where this detection occurred.
    #[serde(rename = "view_occurrences")]
    pub view_occurrences: i32,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AggregatedSlowFCPHighBytes {
    pub fn new(
        avg_bytes_before_fcp_bytes: i64,
        avg_first_contentful_paint_ms: i64,
        avg_resource_count_before_fcp: i64,
        fingerprint: String,
        impact_score: f64,
        platform: String,
        view_occurrences: i32,
    ) -> AggregatedSlowFCPHighBytes {
        AggregatedSlowFCPHighBytes {
            avg_bytes_before_fcp_bytes,
            avg_first_contentful_paint_ms,
            avg_resource_count_before_fcp,
            fingerprint,
            impact_score,
            platform,
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

impl<'de> Deserialize<'de> for AggregatedSlowFCPHighBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AggregatedSlowFCPHighBytesVisitor;
        impl<'a> Visitor<'a> for AggregatedSlowFCPHighBytesVisitor {
            type Value = AggregatedSlowFCPHighBytes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut avg_bytes_before_fcp_bytes: Option<i64> = None;
                let mut avg_first_contentful_paint_ms: Option<i64> = None;
                let mut avg_resource_count_before_fcp: Option<i64> = None;
                let mut fingerprint: Option<String> = None;
                let mut impact_score: Option<f64> = None;
                let mut platform: Option<String> = None;
                let mut view_occurrences: Option<i32> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "avg_bytes_before_fcp_bytes" => {
                            avg_bytes_before_fcp_bytes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "avg_first_contentful_paint_ms" => {
                            avg_first_contentful_paint_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "avg_resource_count_before_fcp" => {
                            avg_resource_count_before_fcp =
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
                        "platform" => {
                            platform = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let avg_bytes_before_fcp_bytes = avg_bytes_before_fcp_bytes
                    .ok_or_else(|| M::Error::missing_field("avg_bytes_before_fcp_bytes"))?;
                let avg_first_contentful_paint_ms = avg_first_contentful_paint_ms
                    .ok_or_else(|| M::Error::missing_field("avg_first_contentful_paint_ms"))?;
                let avg_resource_count_before_fcp = avg_resource_count_before_fcp
                    .ok_or_else(|| M::Error::missing_field("avg_resource_count_before_fcp"))?;
                let fingerprint =
                    fingerprint.ok_or_else(|| M::Error::missing_field("fingerprint"))?;
                let impact_score =
                    impact_score.ok_or_else(|| M::Error::missing_field("impact_score"))?;
                let platform = platform.ok_or_else(|| M::Error::missing_field("platform"))?;
                let view_occurrences =
                    view_occurrences.ok_or_else(|| M::Error::missing_field("view_occurrences"))?;

                let content = AggregatedSlowFCPHighBytes {
                    avg_bytes_before_fcp_bytes,
                    avg_first_contentful_paint_ms,
                    avg_resource_count_before_fcp,
                    fingerprint,
                    impact_score,
                    platform,
                    view_occurrences,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AggregatedSlowFCPHighBytesVisitor)
    }
}
