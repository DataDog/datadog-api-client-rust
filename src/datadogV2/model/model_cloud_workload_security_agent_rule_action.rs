// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The action the rule can perform if triggered
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudWorkloadSecurityAgentRuleAction {
    /// The core dump action applied on the process matching the rule.
    #[serde(rename = "coredump")]
    pub coredump: Option<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleActionCoreDump>,
    /// Whether the action is disabled
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
    /// SECL expression used to target the container to apply the action on
    #[serde(rename = "filter")]
    pub filter: Option<String>,
    /// Hash file specified by the field attribute
    #[serde(rename = "hash")]
    pub hash: Option<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleActionHash>,
    /// Kill system call applied on the container matching the rule
    #[serde(rename = "kill")]
    pub kill: Option<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleKill>,
    /// The log action applied when the rule is triggered.
    #[serde(rename = "log")]
    pub log: Option<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleActionLog>,
    /// The metadata action applied on the scope matching the rule
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleActionMetadata>,
    /// The network filter action applied on the network traffic matching the rule.
    #[serde(rename = "network_filter")]
    pub network_filter:
        Option<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleActionNetworkFilter>,
    /// The set action applied on the scope matching the rule
    #[serde(rename = "set")]
    pub set: Option<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleActionSet>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudWorkloadSecurityAgentRuleAction {
    pub fn new() -> CloudWorkloadSecurityAgentRuleAction {
        CloudWorkloadSecurityAgentRuleAction {
            coredump: None,
            disabled: None,
            filter: None,
            hash: None,
            kill: None,
            log: None,
            metadata: None,
            network_filter: None,
            set: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn coredump(
        mut self,
        value: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleActionCoreDump,
    ) -> Self {
        self.coredump = Some(value);
        self
    }

    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = Some(value);
        self
    }

    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn hash(
        mut self,
        value: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleActionHash,
    ) -> Self {
        self.hash = Some(value);
        self
    }

    pub fn kill(
        mut self,
        value: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleKill,
    ) -> Self {
        self.kill = Some(value);
        self
    }

    pub fn log(
        mut self,
        value: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleActionLog,
    ) -> Self {
        self.log = Some(value);
        self
    }

    pub fn metadata(
        mut self,
        value: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleActionMetadata,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn network_filter(
        mut self,
        value: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleActionNetworkFilter,
    ) -> Self {
        self.network_filter = Some(value);
        self
    }

    pub fn set(
        mut self,
        value: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleActionSet,
    ) -> Self {
        self.set = Some(value);
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
                let mut coredump: Option<
                    crate::datadogV2::model::CloudWorkloadSecurityAgentRuleActionCoreDump,
                > = None;
                let mut disabled: Option<bool> = None;
                let mut filter: Option<String> = None;
                let mut hash: Option<
                    crate::datadogV2::model::CloudWorkloadSecurityAgentRuleActionHash,
                > = None;
                let mut kill: Option<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleKill> =
                    None;
                let mut log: Option<
                    crate::datadogV2::model::CloudWorkloadSecurityAgentRuleActionLog,
                > = None;
                let mut metadata: Option<
                    crate::datadogV2::model::CloudWorkloadSecurityAgentRuleActionMetadata,
                > = None;
                let mut network_filter: Option<
                    crate::datadogV2::model::CloudWorkloadSecurityAgentRuleActionNetworkFilter,
                > = None;
                let mut set: Option<
                    crate::datadogV2::model::CloudWorkloadSecurityAgentRuleActionSet,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "coredump" => {
                            if v.is_null() {
                                continue;
                            }
                            coredump = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "disabled" => {
                            if v.is_null() {
                                continue;
                            }
                            disabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hash" => {
                            if v.is_null() {
                                continue;
                            }
                            hash = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "kill" => {
                            if v.is_null() {
                                continue;
                            }
                            kill = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "log" => {
                            if v.is_null() {
                                continue;
                            }
                            log = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "network_filter" => {
                            if v.is_null() {
                                continue;
                            }
                            network_filter =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "set" => {
                            if v.is_null() {
                                continue;
                            }
                            set = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CloudWorkloadSecurityAgentRuleAction {
                    coredump,
                    disabled,
                    filter,
                    hash,
                    kill,
                    log,
                    metadata,
                    network_filter,
                    set,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudWorkloadSecurityAgentRuleActionVisitor)
    }
}
