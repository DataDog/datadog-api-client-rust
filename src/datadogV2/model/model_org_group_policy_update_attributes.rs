// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating an org group policy. `policy_name`, `content`, and `enforcement_tier` may be omitted individually to leave them unchanged.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrgGroupPolicyUpdateAttributes {
    /// The policy content as key-value pairs. For `org_config` policies, an arbitrary key-value map (for example, `{"value": "UTC"}`). For `role` policies, a `permissions` key containing an array of permission UUIDs (for example, `{"permissions": ["<uuid>", ...]}`).
    #[serde(rename = "content")]
    pub content: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The enforcement tier of the policy. `OVERRIDE_ALLOWED` means the policy is set but member orgs may mutate it. `GROUP_MANAGED` means the policy is strictly controlled and mutations are blocked for affected orgs. `DELEGATE` means each member org controls its own value. `role` policies only support `GROUP_MANAGED` and `DELEGATE` — `OVERRIDE_ALLOWED` is rejected for this policy type. Transitioning a `role` policy to `DELEGATE` (disabling it) is one-way — the policy cannot be transitioned back to `GROUP_MANAGED` afterward.
    #[serde(rename = "enforcement_tier")]
    pub enforcement_tier: Option<crate::datadogV2::model::OrgGroupPolicyEnforcementTier>,
    /// The name of the policy. This becomes the name of the resource created across orgs in the group (for example, for `role` policies, the name of the created role). Omit to leave unchanged.
    #[serde(rename = "policy_name")]
    pub policy_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OrgGroupPolicyUpdateAttributes {
    pub fn new() -> OrgGroupPolicyUpdateAttributes {
        OrgGroupPolicyUpdateAttributes {
            content: None,
            enforcement_tier: None,
            policy_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn content(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn enforcement_tier(
        mut self,
        value: crate::datadogV2::model::OrgGroupPolicyEnforcementTier,
    ) -> Self {
        self.enforcement_tier = Some(value);
        self
    }

    pub fn policy_name(mut self, value: String) -> Self {
        self.policy_name = Some(value);
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

impl Default for OrgGroupPolicyUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OrgGroupPolicyUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrgGroupPolicyUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for OrgGroupPolicyUpdateAttributesVisitor {
            type Value = OrgGroupPolicyUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut content: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut enforcement_tier: Option<
                    crate::datadogV2::model::OrgGroupPolicyEnforcementTier,
                > = None;
                let mut policy_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "content" => {
                            if v.is_null() {
                                continue;
                            }
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enforcement_tier" => {
                            if v.is_null() {
                                continue;
                            }
                            enforcement_tier =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _enforcement_tier) = enforcement_tier {
                                match _enforcement_tier {
                                    crate::datadogV2::model::OrgGroupPolicyEnforcementTier::UnparsedObject(_enforcement_tier) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "policy_name" => {
                            if v.is_null() {
                                continue;
                            }
                            policy_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = OrgGroupPolicyUpdateAttributes {
                    content,
                    enforcement_tier,
                    policy_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrgGroupPolicyUpdateAttributesVisitor)
    }
}
