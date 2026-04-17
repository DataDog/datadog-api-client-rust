// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating an org group policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrgGroupPolicyUpdateAttributes {
    /// The policy content as key-value pairs.
    #[serde(rename = "content")]
    pub content: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The enforcement tier of the policy. `DEFAULT` means the policy is set but member orgs may mutate it. `ENFORCE` means the policy is strictly controlled and mutations are blocked for affected orgs. `DELEGATE` means each member org controls its own value.
    #[serde(rename = "enforcement_tier")]
    pub enforcement_tier: Option<crate::datadogV2::model::OrgGroupPolicyEnforcementTier>,
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
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrgGroupPolicyUpdateAttributesVisitor)
    }
}
