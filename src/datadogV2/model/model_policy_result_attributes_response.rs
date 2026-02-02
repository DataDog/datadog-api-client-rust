// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a policy evaluation result.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PolicyResultAttributesResponse {
    /// Whether the policy is active.
    #[serde(rename = "active")]
    pub active: bool,
    /// The policy configuration payload.
    #[serde(rename = "payload")]
    pub payload: std::collections::BTreeMap<String, serde_json::Value>,
    /// The type of policy being evaluated.
    #[serde(rename = "policy_type")]
    pub policy_type: String,
    /// The organization UUID reference.
    #[serde(rename = "reference_org_uuid")]
    pub reference_org_uuid: uuid::Uuid,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PolicyResultAttributesResponse {
    pub fn new(
        active: bool,
        payload: std::collections::BTreeMap<String, serde_json::Value>,
        policy_type: String,
        reference_org_uuid: uuid::Uuid,
    ) -> PolicyResultAttributesResponse {
        PolicyResultAttributesResponse {
            active,
            payload,
            policy_type,
            reference_org_uuid,
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

impl<'de> Deserialize<'de> for PolicyResultAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PolicyResultAttributesResponseVisitor;
        impl<'a> Visitor<'a> for PolicyResultAttributesResponseVisitor {
            type Value = PolicyResultAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut active: Option<bool> = None;
                let mut payload: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut policy_type: Option<String> = None;
                let mut reference_org_uuid: Option<uuid::Uuid> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "active" => {
                            active = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "payload" => {
                            payload = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "policy_type" => {
                            policy_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reference_org_uuid" => {
                            reference_org_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let active = active.ok_or_else(|| M::Error::missing_field("active"))?;
                let payload = payload.ok_or_else(|| M::Error::missing_field("payload"))?;
                let policy_type =
                    policy_type.ok_or_else(|| M::Error::missing_field("policy_type"))?;
                let reference_org_uuid = reference_org_uuid
                    .ok_or_else(|| M::Error::missing_field("reference_org_uuid"))?;

                let content = PolicyResultAttributesResponse {
                    active,
                    payload,
                    policy_type,
                    reference_org_uuid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PolicyResultAttributesResponseVisitor)
    }
}
