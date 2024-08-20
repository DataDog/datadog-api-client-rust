// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Rule details.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudConfigurationRegoRule {
    /// The policy written in `rego`, see: <https://www.openpolicyagent.org/docs/latest/policy-language/>
    #[serde(rename = "policy")]
    pub policy: String,
    /// List of resource types that will be evaluated upon. Must have at least one element.
    #[serde(rename = "resourceTypes")]
    pub resource_types: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudConfigurationRegoRule {
    pub fn new(policy: String, resource_types: Vec<String>) -> CloudConfigurationRegoRule {
        CloudConfigurationRegoRule {
            policy,
            resource_types,
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

impl<'de> Deserialize<'de> for CloudConfigurationRegoRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudConfigurationRegoRuleVisitor;
        impl<'a> Visitor<'a> for CloudConfigurationRegoRuleVisitor {
            type Value = CloudConfigurationRegoRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut policy: Option<String> = None;
                let mut resource_types: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "policy" => {
                            policy = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resourceTypes" => {
                            resource_types =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let policy = policy.ok_or_else(|| M::Error::missing_field("policy"))?;
                let resource_types =
                    resource_types.ok_or_else(|| M::Error::missing_field("resource_types"))?;

                let content = CloudConfigurationRegoRule {
                    policy,
                    resource_types,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudConfigurationRegoRuleVisitor)
    }
}
