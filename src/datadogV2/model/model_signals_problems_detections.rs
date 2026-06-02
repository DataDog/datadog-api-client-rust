// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Grouped detection results by detection type.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SignalsProblemsDetections {
    /// Detected high frozen frame rate issues.
    #[serde(rename = "high_frozen_frame_rates")]
    pub high_frozen_frame_rates:
        Option<Vec<crate::datadogV2::model::AggregatedHighFrozenFrameRate>>,
    /// Detected high script evaluation issues.
    #[serde(rename = "high_script_evaluations")]
    pub high_script_evaluations: Option<Vec<crate::datadogV2::model::AggregatedHighScriptEval>>,
    /// Detected low cache hit rate issues.
    #[serde(rename = "low_cache_hit_rates")]
    pub low_cache_hit_rates: Option<Vec<crate::datadogV2::model::AggregatedLowCacheHitRate>>,
    /// Detected mobile scroll friction issues.
    #[serde(rename = "mobile_scroll_frictions")]
    pub mobile_scroll_frictions:
        Option<Vec<crate::datadogV2::model::AggregatedMobileScrollFriction>>,
    /// Detected slow first contentful paint with high byte count issues.
    #[serde(rename = "slow_fcp_high_bytes")]
    pub slow_fcp_high_bytes: Option<Vec<crate::datadogV2::model::AggregatedSlowFCPHighBytes>>,
    /// Detected slow interaction with long task issues.
    #[serde(rename = "slow_interaction_long_tasks")]
    pub slow_interaction_long_tasks:
        Option<Vec<crate::datadogV2::model::AggregatedSlowInteractionLongTask>>,
    /// Detected uncompressed resource issues.
    #[serde(rename = "uncompressed_resources")]
    pub uncompressed_resources:
        Option<Vec<crate::datadogV2::model::AggregatedUncompressedResource>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SignalsProblemsDetections {
    pub fn new() -> SignalsProblemsDetections {
        SignalsProblemsDetections {
            high_frozen_frame_rates: None,
            high_script_evaluations: None,
            low_cache_hit_rates: None,
            mobile_scroll_frictions: None,
            slow_fcp_high_bytes: None,
            slow_interaction_long_tasks: None,
            uncompressed_resources: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn high_frozen_frame_rates(
        mut self,
        value: Vec<crate::datadogV2::model::AggregatedHighFrozenFrameRate>,
    ) -> Self {
        self.high_frozen_frame_rates = Some(value);
        self
    }

    pub fn high_script_evaluations(
        mut self,
        value: Vec<crate::datadogV2::model::AggregatedHighScriptEval>,
    ) -> Self {
        self.high_script_evaluations = Some(value);
        self
    }

    pub fn low_cache_hit_rates(
        mut self,
        value: Vec<crate::datadogV2::model::AggregatedLowCacheHitRate>,
    ) -> Self {
        self.low_cache_hit_rates = Some(value);
        self
    }

    pub fn mobile_scroll_frictions(
        mut self,
        value: Vec<crate::datadogV2::model::AggregatedMobileScrollFriction>,
    ) -> Self {
        self.mobile_scroll_frictions = Some(value);
        self
    }

    pub fn slow_fcp_high_bytes(
        mut self,
        value: Vec<crate::datadogV2::model::AggregatedSlowFCPHighBytes>,
    ) -> Self {
        self.slow_fcp_high_bytes = Some(value);
        self
    }

    pub fn slow_interaction_long_tasks(
        mut self,
        value: Vec<crate::datadogV2::model::AggregatedSlowInteractionLongTask>,
    ) -> Self {
        self.slow_interaction_long_tasks = Some(value);
        self
    }

    pub fn uncompressed_resources(
        mut self,
        value: Vec<crate::datadogV2::model::AggregatedUncompressedResource>,
    ) -> Self {
        self.uncompressed_resources = Some(value);
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

impl Default for SignalsProblemsDetections {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SignalsProblemsDetections {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SignalsProblemsDetectionsVisitor;
        impl<'a> Visitor<'a> for SignalsProblemsDetectionsVisitor {
            type Value = SignalsProblemsDetections;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut high_frozen_frame_rates: Option<
                    Vec<crate::datadogV2::model::AggregatedHighFrozenFrameRate>,
                > = None;
                let mut high_script_evaluations: Option<
                    Vec<crate::datadogV2::model::AggregatedHighScriptEval>,
                > = None;
                let mut low_cache_hit_rates: Option<
                    Vec<crate::datadogV2::model::AggregatedLowCacheHitRate>,
                > = None;
                let mut mobile_scroll_frictions: Option<
                    Vec<crate::datadogV2::model::AggregatedMobileScrollFriction>,
                > = None;
                let mut slow_fcp_high_bytes: Option<
                    Vec<crate::datadogV2::model::AggregatedSlowFCPHighBytes>,
                > = None;
                let mut slow_interaction_long_tasks: Option<
                    Vec<crate::datadogV2::model::AggregatedSlowInteractionLongTask>,
                > = None;
                let mut uncompressed_resources: Option<
                    Vec<crate::datadogV2::model::AggregatedUncompressedResource>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "high_frozen_frame_rates" => {
                            if v.is_null() {
                                continue;
                            }
                            high_frozen_frame_rates =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "high_script_evaluations" => {
                            if v.is_null() {
                                continue;
                            }
                            high_script_evaluations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "low_cache_hit_rates" => {
                            if v.is_null() {
                                continue;
                            }
                            low_cache_hit_rates =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mobile_scroll_frictions" => {
                            if v.is_null() {
                                continue;
                            }
                            mobile_scroll_frictions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "slow_fcp_high_bytes" => {
                            if v.is_null() {
                                continue;
                            }
                            slow_fcp_high_bytes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "slow_interaction_long_tasks" => {
                            if v.is_null() {
                                continue;
                            }
                            slow_interaction_long_tasks =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "uncompressed_resources" => {
                            if v.is_null() {
                                continue;
                            }
                            uncompressed_resources =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SignalsProblemsDetections {
                    high_frozen_frame_rates,
                    high_script_evaluations,
                    low_cache_hit_rates,
                    mobile_scroll_frictions,
                    slow_fcp_high_bytes,
                    slow_interaction_long_tasks,
                    uncompressed_resources,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SignalsProblemsDetectionsVisitor)
    }
}
