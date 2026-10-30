// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Request-level confidence threshold override. `flagging_global` is the global
/// threshold applied to every threat category. Any other property is a
/// per-category override keyed by an AI Guard threat tag; per-category values
/// take precedence over `flagging_global`, and categories without an entry fall
/// back to it. When this object omits flagging_global, the fallback is `0.5`.
/// Recognized category keys include policy-evasion, indirect-prompt-injection,
/// system-and-capability-discovery, credential-access, sensitive-data-collection,
/// privilege-escalation, malicious-execution, private-network-target,
/// destructive-or-resource-abuse, tool-abuse, poisoning-or-persistence, and
/// `data-exfiltration`. Unknown category keys are ignored.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AIGuardConfidenceThreshold {
    /// The global confidence threshold applied to categories without a specific override.
    #[serde(rename = "flagging_global")]
    pub flagging_global: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, f64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AIGuardConfidenceThreshold {
    pub fn new() -> AIGuardConfidenceThreshold {
        AIGuardConfidenceThreshold {
            flagging_global: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn flagging_global(mut self, value: f64) -> Self {
        self.flagging_global = Some(value);
        self
    }

    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, f64>) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for AIGuardConfidenceThreshold {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AIGuardConfidenceThreshold {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AIGuardConfidenceThresholdVisitor;
        impl<'a> Visitor<'a> for AIGuardConfidenceThresholdVisitor {
            type Value = AIGuardConfidenceThreshold;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut flagging_global: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<String, f64> =
                    std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "flagging_global" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            flagging_global =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AIGuardConfidenceThreshold {
                    flagging_global,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AIGuardConfidenceThresholdVisitor)
    }
}
