// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A rule restricting a Slack scope to specific private channel names.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExecutionPolicySlackScopeRule {
    /// The private channel name selectors this rule applies to. Selectors are lowercase private channel names
    /// without the leading `#`, and can contain `*` wildcards. Use `*` alone to match any private channel name.
    #[serde(rename = "channel_name_selectors")]
    pub channel_name_selectors: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExecutionPolicySlackScopeRule {
    pub fn new(channel_name_selectors: Vec<String>) -> ExecutionPolicySlackScopeRule {
        ExecutionPolicySlackScopeRule {
            channel_name_selectors,
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

impl<'de> Deserialize<'de> for ExecutionPolicySlackScopeRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExecutionPolicySlackScopeRuleVisitor;
        impl<'a> Visitor<'a> for ExecutionPolicySlackScopeRuleVisitor {
            type Value = ExecutionPolicySlackScopeRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut channel_name_selectors: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "channel_name_selectors" => {
                            channel_name_selectors =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let channel_name_selectors = channel_name_selectors
                    .ok_or_else(|| M::Error::missing_field("channel_name_selectors"))?;

                let content = ExecutionPolicySlackScopeRule {
                    channel_name_selectors,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExecutionPolicySlackScopeRuleVisitor)
    }
}
