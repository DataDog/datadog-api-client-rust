// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the investigation to start.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityAgentInvestigationCreateAttributes {
    /// Start a new investigation even if the signal already has one. If the signal's investigation is still
    /// running, the response has that investigation instead. A rerun counts toward the monthly investigation
    /// limit like any investigation.
    #[serde(rename = "rerun")]
    pub rerun: Option<bool>,
    /// ID of the signal to investigate, in its V1 form, which starts with `AQ`, or in its longer V3 form, which
    /// starts with `Aw`. Investigations carry the V1 form, so their `signal_id` can differ from the ID you pass.
    #[serde(rename = "signal_id")]
    pub signal_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityAgentInvestigationCreateAttributes {
    pub fn new(signal_id: String) -> SecurityAgentInvestigationCreateAttributes {
        SecurityAgentInvestigationCreateAttributes {
            rerun: None,
            signal_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn rerun(mut self, value: bool) -> Self {
        self.rerun = Some(value);
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

impl<'de> Deserialize<'de> for SecurityAgentInvestigationCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityAgentInvestigationCreateAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityAgentInvestigationCreateAttributesVisitor {
            type Value = SecurityAgentInvestigationCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut rerun: Option<bool> = None;
                let mut signal_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "rerun" => {
                            if v.is_null() {
                                continue;
                            }
                            rerun = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "signal_id" => {
                            signal_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let signal_id = signal_id.ok_or_else(|| M::Error::missing_field("signal_id"))?;

                let content = SecurityAgentInvestigationCreateAttributes {
                    rerun,
                    signal_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityAgentInvestigationCreateAttributesVisitor)
    }
}
