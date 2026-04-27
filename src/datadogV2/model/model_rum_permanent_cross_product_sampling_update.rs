// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Partial update of the cross-product sample rates for a permanent retention filter.
/// Only rates with a matching `cross_product_sampling_editability` flag set to `true` can be updated.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumPermanentCrossProductSamplingUpdate {
    /// Percentage (0–100) of retained sessions (with an ingested replay) whose replay is kept.
    /// Omit to leave unchanged. Rejected if the filter's `cross_product_sampling_editability.session_replay_sample_rate` is `false`.
    #[serde(rename = "session_replay_sample_rate")]
    pub session_replay_sample_rate: Option<f64>,
    /// Percentage (0–100) of retained sessions (with ingested traces) whose traces are indexed.
    /// Omit to leave unchanged. Rejected if the filter's `cross_product_sampling_editability.trace_sample_rate` is `false`.
    #[serde(rename = "trace_sample_rate")]
    pub trace_sample_rate: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumPermanentCrossProductSamplingUpdate {
    pub fn new() -> RumPermanentCrossProductSamplingUpdate {
        RumPermanentCrossProductSamplingUpdate {
            session_replay_sample_rate: None,
            trace_sample_rate: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn session_replay_sample_rate(mut self, value: f64) -> Self {
        self.session_replay_sample_rate = Some(value);
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

impl Default for RumPermanentCrossProductSamplingUpdate {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RumPermanentCrossProductSamplingUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumPermanentCrossProductSamplingUpdateVisitor;
        impl<'a> Visitor<'a> for RumPermanentCrossProductSamplingUpdateVisitor {
            type Value = RumPermanentCrossProductSamplingUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut session_replay_sample_rate: Option<f64> = None;
                let mut trace_sample_rate: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "session_replay_sample_rate" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            session_replay_sample_rate =
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

                let content = RumPermanentCrossProductSamplingUpdate {
                    session_replay_sample_rate,
                    trace_sample_rate,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumPermanentCrossProductSamplingUpdateVisitor)
    }
}
