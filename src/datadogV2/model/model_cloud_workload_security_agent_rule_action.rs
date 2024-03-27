// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The action the rule can perform if triggered.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudWorkloadSecurityAgentRuleAction {
    /// SECL expression used to target the container to apply the action on
    #[serde(rename = "filter")]
    pub filter: Option<String>,
    /// Kill system call applied on the container matching the rule
    #[serde(rename = "kill")]
    pub kill: Option<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleKill>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudWorkloadSecurityAgentRuleAction {
    pub fn new() -> CloudWorkloadSecurityAgentRuleAction {
        CloudWorkloadSecurityAgentRuleAction {
            filter: None,
            kill: None,
            _unparsed: false,
        }
    }

    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn kill(
        mut self,
        value: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleKill,
    ) -> Self {
        self.kill = Some(value);
        self
    }
}

impl Default for CloudWorkloadSecurityAgentRuleAction {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CloudWorkloadSecurityAgentRuleAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudWorkloadSecurityAgentRuleActionVisitor;
        impl<'a> Visitor<'a> for CloudWorkloadSecurityAgentRuleActionVisitor {
            type Value = CloudWorkloadSecurityAgentRuleAction;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filter: Option<String> = None;
                let mut kill: Option<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleKill> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "kill" => {
                            if v.is_null() {
                                continue;
                            }
                            kill = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = CloudWorkloadSecurityAgentRuleAction {
                    filter,
                    kill,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudWorkloadSecurityAgentRuleActionVisitor)
    }
}
