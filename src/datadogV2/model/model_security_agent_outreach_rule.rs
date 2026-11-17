// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The outreach automation state of one detection rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityAgentOutreachRule {
    /// Whether the rule auto-sends automated interviews in `automated` mode.
    #[serde(rename = "is_enabled")]
    pub is_enabled: bool,
    /// ID of the detection rule.
    #[serde(rename = "rule_id")]
    pub rule_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityAgentOutreachRule {
    pub fn new(is_enabled: bool, rule_id: String) -> SecurityAgentOutreachRule {
        SecurityAgentOutreachRule {
            is_enabled,
            rule_id,
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

impl<'de> Deserialize<'de> for SecurityAgentOutreachRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityAgentOutreachRuleVisitor;
        impl<'a> Visitor<'a> for SecurityAgentOutreachRuleVisitor {
            type Value = SecurityAgentOutreachRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_enabled: Option<bool> = None;
                let mut rule_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "is_enabled" => {
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_id" => {
                            rule_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let is_enabled = is_enabled.ok_or_else(|| M::Error::missing_field("is_enabled"))?;
                let rule_id = rule_id.ok_or_else(|| M::Error::missing_field("rule_id"))?;

                let content = SecurityAgentOutreachRule {
                    is_enabled,
                    rule_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityAgentOutreachRuleVisitor)
    }
}
