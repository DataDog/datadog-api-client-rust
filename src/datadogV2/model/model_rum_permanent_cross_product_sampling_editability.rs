// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Flags indicating which `cross_product_sampling` rates can be updated for this filter. Read-only.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumPermanentCrossProductSamplingEditability {
    /// If `true`, `cross_product_sampling.session_replay_sample_rate` can be updated on this filter.
    #[serde(rename = "session_replay_sample_rate")]
    pub session_replay_sample_rate: Option<bool>,
    /// If `true`, `cross_product_sampling.trace_sample_rate` can be updated on this filter.
    #[serde(rename = "trace_sample_rate")]
    pub trace_sample_rate: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumPermanentCrossProductSamplingEditability {
    pub fn new() -> RumPermanentCrossProductSamplingEditability {
        RumPermanentCrossProductSamplingEditability {
            session_replay_sample_rate: None,
            trace_sample_rate: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn session_replay_sample_rate(mut self, value: bool) -> Self {
        self.session_replay_sample_rate = Some(value);
        self
    }

    pub fn trace_sample_rate(mut self, value: bool) -> Self {
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

impl Default for RumPermanentCrossProductSamplingEditability {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RumPermanentCrossProductSamplingEditability {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumPermanentCrossProductSamplingEditabilityVisitor;
        impl<'a> Visitor<'a> for RumPermanentCrossProductSamplingEditabilityVisitor {
            type Value = RumPermanentCrossProductSamplingEditability;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut session_replay_sample_rate: Option<bool> = None;
                let mut trace_sample_rate: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "session_replay_sample_rate" => {
                            if v.is_null() {
                                continue;
                            }
                            session_replay_sample_rate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trace_sample_rate" => {
                            if v.is_null() {
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

                let content = RumPermanentCrossProductSamplingEditability {
                    session_replay_sample_rate,
                    trace_sample_rate,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumPermanentCrossProductSamplingEditabilityVisitor)
    }
}
