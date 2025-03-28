// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A Cloud Workload Security Agent rule returned by the API
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudWorkloadSecurityAgentPolicyAttributes {
    /// The number of rules with the blocking feature in this policy
    #[serde(rename = "blockingRulesCount")]
    pub blocking_rules_count: Option<i32>,
    /// The description of the policy
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The number of rules that are disabled in this policy
    #[serde(rename = "disabledRulesCount")]
    pub disabled_rules_count: Option<i32>,
    /// Whether the Agent policy is enabled
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The host tags defining where this policy is deployed
    #[serde(rename = "hostTags")]
    pub host_tags: Option<Vec<String>>,
    /// The number of rules in the monitoring state in this policy
    #[serde(rename = "monitoringRulesCount")]
    pub monitoring_rules_count: Option<i32>,
    /// The name of the policy
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The priority of the policy
    #[serde(rename = "priority")]
    pub priority: Option<i64>,
    /// The number of rules in this policy
    #[serde(rename = "ruleCount")]
    pub rule_count: Option<i32>,
    /// When the policy was last updated, timestamp in milliseconds
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
    /// The attributes of the user who last updated the policy
    #[serde(rename = "updater")]
    pub updater: Option<crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyUpdaterAttributes>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudWorkloadSecurityAgentPolicyAttributes {
    pub fn new() -> CloudWorkloadSecurityAgentPolicyAttributes {
        CloudWorkloadSecurityAgentPolicyAttributes {
            blocking_rules_count: None,
            description: None,
            disabled_rules_count: None,
            enabled: None,
            host_tags: None,
            monitoring_rules_count: None,
            name: None,
            priority: None,
            rule_count: None,
            updated_at: None,
            updater: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn blocking_rules_count(mut self, value: i32) -> Self {
        self.blocking_rules_count = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn disabled_rules_count(mut self, value: i32) -> Self {
        self.disabled_rules_count = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn host_tags(mut self, value: Vec<String>) -> Self {
        self.host_tags = Some(value);
        self
    }

    pub fn monitoring_rules_count(mut self, value: i32) -> Self {
        self.monitoring_rules_count = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn priority(mut self, value: i64) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn rule_count(mut self, value: i32) -> Self {
        self.rule_count = Some(value);
        self
    }

    pub fn updated_at(mut self, value: i64) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn updater(
        mut self,
        value: crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyUpdaterAttributes,
    ) -> Self {
        self.updater = Some(value);
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

impl Default for CloudWorkloadSecurityAgentPolicyAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CloudWorkloadSecurityAgentPolicyAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudWorkloadSecurityAgentPolicyAttributesVisitor;
        impl<'a> Visitor<'a> for CloudWorkloadSecurityAgentPolicyAttributesVisitor {
            type Value = CloudWorkloadSecurityAgentPolicyAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut blocking_rules_count: Option<i32> = None;
                let mut description: Option<String> = None;
                let mut disabled_rules_count: Option<i32> = None;
                let mut enabled: Option<bool> = None;
                let mut host_tags: Option<Vec<String>> = None;
                let mut monitoring_rules_count: Option<i32> = None;
                let mut name: Option<String> = None;
                let mut priority: Option<i64> = None;
                let mut rule_count: Option<i32> = None;
                let mut updated_at: Option<i64> = None;
                let mut updater: Option<
                    crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyUpdaterAttributes,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "blockingRulesCount" => {
                            if v.is_null() {
                                continue;
                            }
                            blocking_rules_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "disabledRulesCount" => {
                            if v.is_null() {
                                continue;
                            }
                            disabled_rules_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hostTags" => {
                            if v.is_null() {
                                continue;
                            }
                            host_tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitoringRulesCount" => {
                            if v.is_null() {
                                continue;
                            }
                            monitoring_rules_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "priority" => {
                            if v.is_null() {
                                continue;
                            }
                            priority = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ruleCount" => {
                            if v.is_null() {
                                continue;
                            }
                            rule_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updatedAt" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updater" => {
                            if v.is_null() {
                                continue;
                            }
                            updater = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CloudWorkloadSecurityAgentPolicyAttributes {
                    blocking_rules_count,
                    description,
                    disabled_rules_count,
                    enabled,
                    host_tags,
                    monitoring_rules_count,
                    name,
                    priority,
                    rule_count,
                    updated_at,
                    updater,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudWorkloadSecurityAgentPolicyAttributesVisitor)
    }
}
