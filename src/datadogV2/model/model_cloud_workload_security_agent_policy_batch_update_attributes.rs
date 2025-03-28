// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for the batch update operation
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudWorkloadSecurityAgentPolicyBatchUpdateAttributes {
    /// List of policies to update in the batch
    #[serde(rename = "policies")]
    pub policies: Option<Vec<crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyBatchUpdateAttributesPoliciesItems>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CloudWorkloadSecurityAgentPolicyBatchUpdateAttributes {
    pub fn new() -> CloudWorkloadSecurityAgentPolicyBatchUpdateAttributes {
        CloudWorkloadSecurityAgentPolicyBatchUpdateAttributes {
            policies: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn policies(
        mut self,
        value: Vec<crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyBatchUpdateAttributesPoliciesItems>,
    ) -> Self {
        self.policies = Some(value);
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

impl Default for CloudWorkloadSecurityAgentPolicyBatchUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CloudWorkloadSecurityAgentPolicyBatchUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudWorkloadSecurityAgentPolicyBatchUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for CloudWorkloadSecurityAgentPolicyBatchUpdateAttributesVisitor {
            type Value = CloudWorkloadSecurityAgentPolicyBatchUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut policies: Option<Vec<crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyBatchUpdateAttributesPoliciesItems>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "policies" => {
                            if v.is_null() {
                                continue;
                            }
                            policies = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CloudWorkloadSecurityAgentPolicyBatchUpdateAttributes {
                    policies,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudWorkloadSecurityAgentPolicyBatchUpdateAttributesVisitor)
    }
}
