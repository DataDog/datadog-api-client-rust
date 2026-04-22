// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an org group policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrgGroupPolicyAttributes {
    /// The policy content as key-value pairs.
    #[serde(rename = "content")]
    pub content: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Timestamp when the policy was enforced.
    #[serde(rename = "enforced_at")]
    pub enforced_at: chrono::DateTime<chrono::Utc>,
    /// Timestamp when the policy was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// The name of the policy.
    #[serde(rename = "policy_name")]
    pub policy_name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OrgGroupPolicyAttributes {
    pub fn new(
        enforced_at: chrono::DateTime<chrono::Utc>,
        modified_at: chrono::DateTime<chrono::Utc>,
        policy_name: String,
    ) -> OrgGroupPolicyAttributes {
        OrgGroupPolicyAttributes {
            content: None,
            enforced_at,
            modified_at,
            policy_name,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn content(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.content = Some(value);
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

impl<'de> Deserialize<'de> for OrgGroupPolicyAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrgGroupPolicyAttributesVisitor;
        impl<'a> Visitor<'a> for OrgGroupPolicyAttributesVisitor {
            type Value = OrgGroupPolicyAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut content: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut enforced_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
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
                        "enforced_at" => {
                            enforced_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "policy_name" => {
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
                let enforced_at =
                    enforced_at.ok_or_else(|| M::Error::missing_field("enforced_at"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let policy_name =
                    policy_name.ok_or_else(|| M::Error::missing_field("policy_name"))?;

                let content = OrgGroupPolicyAttributes {
                    content,
                    enforced_at,
                    modified_at,
                    policy_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrgGroupPolicyAttributesVisitor)
    }
}
