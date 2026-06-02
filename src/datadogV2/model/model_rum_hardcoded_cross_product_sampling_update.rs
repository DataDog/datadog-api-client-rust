// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Partial update for cross-product retention of a hardcoded retention filter.
/// Only fields whose matching flag in `cross_product_sampling_editability` is `true` can be updated.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumHardcodedCrossProductSamplingUpdate {
    /// Controls whether Session Replay cross-product retention is active. Omit to leave unchanged.
    #[serde(rename = "session_replay_enabled")]
    pub session_replay_enabled: Option<bool>,
    /// Percentage (0–100) of retained sessions with an ingested replay whose replay data is kept.
    /// Omit to leave unchanged.
    #[serde(rename = "session_replay_sample_rate")]
    pub session_replay_sample_rate: Option<f64>,
    /// Controls whether Trace cross-product retention is active. Omit to leave unchanged.
    #[serde(rename = "trace_enabled")]
    pub trace_enabled: Option<bool>,
    /// Percentage (0–100) of retained sessions with ingested traces whose traces are indexed.
    /// Omit to leave unchanged.
    #[serde(rename = "trace_sample_rate")]
    pub trace_sample_rate: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumHardcodedCrossProductSamplingUpdate {
    pub fn new() -> RumHardcodedCrossProductSamplingUpdate {
        RumHardcodedCrossProductSamplingUpdate {
            session_replay_enabled: None,
            session_replay_sample_rate: None,
            trace_enabled: None,
            trace_sample_rate: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn session_replay_enabled(mut self, value: bool) -> Self {
        self.session_replay_enabled = Some(value);
        self
    }

    pub fn session_replay_sample_rate(mut self, value: f64) -> Self {
        self.session_replay_sample_rate = Some(value);
        self
    }

    pub fn trace_enabled(mut self, value: bool) -> Self {
        self.trace_enabled = Some(value);
        self
    }

    pub fn trace_sample_rate(mut self, value: f64) -> Self {
        self.trace_sample_rate = Some(value);
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

impl Default for RumHardcodedCrossProductSamplingUpdate {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RumHardcodedCrossProductSamplingUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumHardcodedCrossProductSamplingUpdateVisitor;
        impl<'a> Visitor<'a> for RumHardcodedCrossProductSamplingUpdateVisitor {
            type Value = RumHardcodedCrossProductSamplingUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut session_replay_enabled: Option<bool> = None;
                let mut session_replay_sample_rate: Option<f64> = None;
                let mut trace_enabled: Option<bool> = None;
                let mut trace_sample_rate: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "session_replay_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            session_replay_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_replay_sample_rate" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            session_replay_sample_rate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trace_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            trace_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trace_sample_rate" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            trace_sample_rate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RumHardcodedCrossProductSamplingUpdate {
                    session_replay_enabled,
                    session_replay_sample_rate,
                    trace_enabled,
                    trace_sample_rate,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumHardcodedCrossProductSamplingUpdateVisitor)
    }
}
