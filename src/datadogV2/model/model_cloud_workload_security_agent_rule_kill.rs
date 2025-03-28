// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Kill system call applied on the container matching the rule
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudWorkloadSecurityAgentRuleKill {
    /// Supported signals for the kill system call
    #[serde(rename = "signal")]
    pub signal: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudWorkloadSecurityAgentRuleKill {
    pub fn new() -> CloudWorkloadSecurityAgentRuleKill {
        CloudWorkloadSecurityAgentRuleKill {
            signal: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn signal(mut self, value: String) -> Self {
        self.signal = Some(value);
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

impl Default for CloudWorkloadSecurityAgentRuleKill {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CloudWorkloadSecurityAgentRuleKill {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudWorkloadSecurityAgentRuleKillVisitor;
        impl<'a> Visitor<'a> for CloudWorkloadSecurityAgentRuleKillVisitor {
            type Value = CloudWorkloadSecurityAgentRuleKill;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut signal: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "signal" => {
                            if v.is_null() {
                                continue;
                            }
                            signal = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CloudWorkloadSecurityAgentRuleKill {
                    signal,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudWorkloadSecurityAgentRuleKillVisitor)
    }
}
