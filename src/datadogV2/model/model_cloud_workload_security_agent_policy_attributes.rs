// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A Cloud Workload Security Agent policy returned by the API
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudWorkloadSecurityAgentPolicyAttributes {
    /// The number of rules with the blocking feature in this policy
    #[serde(rename = "blockingRulesCount")]
    pub blocking_rules_count: Option<i32>,
    /// Whether the policy is managed by Datadog
    #[serde(rename = "datadogManaged")]
    pub datadog_managed: Option<bool>,
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
    /// The host tags defining where this policy is deployed, the inner values are linked with AND, the outer values are linked with OR
    #[serde(rename = "hostTagsLists")]
    pub host_tags_lists: Option<Vec<Vec<String>>>,
    /// The number of rules in the monitoring state in this policy
    #[serde(rename = "monitoringRulesCount")]
    pub monitoring_rules_count: Option<i32>,
    /// The name of the policy
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Whether the policy is pinned
    #[serde(rename = "pinned")]
    pub pinned: Option<bool>,
    /// The type of the policy
    #[serde(rename = "policyType")]
    pub policy_type: Option<String>,
    /// The version of the policy
    #[serde(rename = "policyVersion")]
    pub policy_version: Option<String>,
    /// The priority of the policy
    #[serde(rename = "priority")]
    pub priority: Option<i64>,
    /// The number of rules in this policy
    #[serde(rename = "ruleCount")]
    pub rule_count: Option<i32>,
    /// Timestamp in milliseconds when the policy was last updated
    #[serde(rename = "updateDate")]
    pub update_date: Option<i64>,
    /// When the policy was last updated, timestamp in milliseconds
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
    /// The attributes of the user who last updated the policy
    #[serde(rename = "updater")]
    pub updater: Option<crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyUpdaterAttributes>,
    /// The versions of the policy
    #[serde(rename = "versions")]
    pub versions: Option<Vec<crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyVersion>>,
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
            datadog_managed: None,
            description: None,
            disabled_rules_count: None,
            enabled: None,
            host_tags: None,
            host_tags_lists: None,
            monitoring_rules_count: None,
            name: None,
            pinned: None,
            policy_type: None,
            policy_version: None,
            priority: None,
            rule_count: None,
            update_date: None,
            updated_at: None,
            updater: None,
            versions: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn blocking_rules_count(mut self, value: i32) -> Self {
        self.blocking_rules_count = Some(value);
        self
    }

    pub fn datadog_managed(mut self, value: bool) -> Self {
        self.datadog_managed = Some(value);
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

    pub fn host_tags_lists(mut self, value: Vec<Vec<String>>) -> Self {
        self.host_tags_lists = Some(value);
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

    pub fn pinned(mut self, value: bool) -> Self {
        self.pinned = Some(value);
        self
    }

    pub fn policy_type(mut self, value: String) -> Self {
        self.policy_type = Some(value);
        self
    }

    pub fn policy_version(mut self, value: String) -> Self {
        self.policy_version = Some(value);
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

    pub fn update_date(mut self, value: i64) -> Self {
        self.update_date = Some(value);
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

    pub fn versions(
        mut self,
        value: Vec<crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyVersion>,
    ) -> Self {
        self.versions = Some(value);
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
                let mut datadog_managed: Option<bool> = None;
                let mut description: Option<String> = None;
                let mut disabled_rules_count: Option<i32> = None;
                let mut enabled: Option<bool> = None;
                let mut host_tags: Option<Vec<String>> = None;
                let mut host_tags_lists: Option<Vec<Vec<String>>> = None;
                let mut monitoring_rules_count: Option<i32> = None;
                let mut name: Option<String> = None;
                let mut pinned: Option<bool> = None;
                let mut policy_type: Option<String> = None;
                let mut policy_version: Option<String> = None;
                let mut priority: Option<i64> = None;
                let mut rule_count: Option<i32> = None;
                let mut update_date: Option<i64> = None;
                let mut updated_at: Option<i64> = None;
                let mut updater: Option<
                    crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyUpdaterAttributes,
                > = None;
                let mut versions: Option<
                    Vec<crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyVersion>,
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
                        "datadogManaged" => {
                            if v.is_null() {
                                continue;
                            }
                            datadog_managed =
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
                        "hostTagsLists" => {
                            if v.is_null() {
                                continue;
                            }
                            host_tags_lists =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "pinned" => {
                            if v.is_null() {
                                continue;
                            }
                            pinned = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "policyType" => {
                            if v.is_null() {
                                continue;
                            }
                            policy_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "policyVersion" => {
                            if v.is_null() {
                                continue;
                            }
                            policy_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "updateDate" => {
                            if v.is_null() {
                                continue;
                            }
                            update_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "versions" => {
                            if v.is_null() {
                                continue;
                            }
                            versions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                    datadog_managed,
                    description,
                    disabled_rules_count,
                    enabled,
                    host_tags,
                    host_tags_lists,
                    monitoring_rules_count,
                    name,
                    pinned,
                    policy_type,
                    policy_version,
                    priority,
                    rule_count,
                    update_date,
                    updated_at,
                    updater,
                    versions,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudWorkloadSecurityAgentPolicyAttributesVisitor)
    }
}
