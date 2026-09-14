// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Policy and policy type for a monitor configuration policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorConfigPolicyAttributeEditRequest {
    /// Configuration for the policy.
    #[serde(rename = "policy")]
    pub policy: crate::datadogV2::model::MonitorConfigPolicyPolicy,
    /// The monitor configuration policy type.
    #[serde(rename = "policy_type")]
    pub policy_type: crate::datadogV2::model::MonitorConfigPolicyType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorConfigPolicyAttributeEditRequest {
    pub fn new(
        policy: crate::datadogV2::model::MonitorConfigPolicyPolicy,
        policy_type: crate::datadogV2::model::MonitorConfigPolicyType,
    ) -> MonitorConfigPolicyAttributeEditRequest {
        MonitorConfigPolicyAttributeEditRequest {
            policy,
            policy_type,
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

impl<'de> Deserialize<'de> for MonitorConfigPolicyAttributeEditRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorConfigPolicyAttributeEditRequestVisitor;
        impl<'a> Visitor<'a> for MonitorConfigPolicyAttributeEditRequestVisitor {
            type Value = MonitorConfigPolicyAttributeEditRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut policy: Option<crate::datadogV2::model::MonitorConfigPolicyPolicy> = None;
                let mut policy_type: Option<crate::datadogV2::model::MonitorConfigPolicyType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "policy" => {
                            policy = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _policy) = policy {
                                match _policy {
                                    crate::datadogV2::model::MonitorConfigPolicyPolicy::UnparsedObject(_policy) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "policy_type" => {
                            policy_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _policy_type) = policy_type {
                                match _policy_type {
                                    crate::datadogV2::model::MonitorConfigPolicyType::UnparsedObject(_policy_type) => {
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
                let policy = policy.ok_or_else(|| M::Error::missing_field("policy"))?;
                let policy_type =
                    policy_type.ok_or_else(|| M::Error::missing_field("policy_type"))?;

                let content = MonitorConfigPolicyAttributeEditRequest {
                    policy,
                    policy_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorConfigPolicyAttributeEditRequestVisitor)
    }
}
