// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Create a new Cloud Workload Security Agent policy
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudWorkloadSecurityAgentPolicyCreateAttributes {
    /// The description of the policy
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Whether the policy is enabled
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The host tags defining where this policy is deployed
    #[serde(rename = "hostTags")]
    pub host_tags: Option<Vec<String>>,
    /// The host tags defining where this policy is deployed, the inner values are linked with AND, the outer values are linked with OR
    #[serde(rename = "hostTagsLists")]
    pub host_tags_lists: Option<Vec<Vec<String>>>,
    /// The name of the policy
    #[serde(rename = "name")]
    pub name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudWorkloadSecurityAgentPolicyCreateAttributes {
    pub fn new(name: String) -> CloudWorkloadSecurityAgentPolicyCreateAttributes {
        CloudWorkloadSecurityAgentPolicyCreateAttributes {
            description: None,
            enabled: None,
            host_tags: None,
            host_tags_lists: None,
            name,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for CloudWorkloadSecurityAgentPolicyCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudWorkloadSecurityAgentPolicyCreateAttributesVisitor;
        impl<'a> Visitor<'a> for CloudWorkloadSecurityAgentPolicyCreateAttributesVisitor {
            type Value = CloudWorkloadSecurityAgentPolicyCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut enabled: Option<bool> = None;
                let mut host_tags: Option<Vec<String>> = None;
                let mut host_tags_lists: Option<Vec<Vec<String>>> = None;
                let mut name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
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
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = CloudWorkloadSecurityAgentPolicyCreateAttributes {
                    description,
                    enabled,
                    host_tags,
                    host_tags_lists,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudWorkloadSecurityAgentPolicyCreateAttributesVisitor)
    }
}
