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
pub struct MonitorConfigPolicyAttributeResponse {
    /// Configuration for the policy.
    #[serde(rename = "policy")]
    pub policy: Option<crate::datadogV2::model::MonitorConfigPolicyPolicy>,
    /// The monitor configuration policy type.
    #[serde(rename = "policy_type")]
    pub policy_type: Option<crate::datadogV2::model::MonitorConfigPolicyType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorConfigPolicyAttributeResponse {
    pub fn new() -> MonitorConfigPolicyAttributeResponse {
        MonitorConfigPolicyAttributeResponse {
            policy: None,
            policy_type: None,
            _unparsed: false,
        }
    }

    pub fn policy(mut self, value: crate::datadogV2::model::MonitorConfigPolicyPolicy) -> Self {
        self.policy = Some(value);
        self
    }

    pub fn policy_type(mut self, value: crate::datadogV2::model::MonitorConfigPolicyType) -> Self {
        self.policy_type = Some(value);
        self
    }
}

impl Default for MonitorConfigPolicyAttributeResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorConfigPolicyAttributeResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorConfigPolicyAttributeResponseVisitor;
        impl<'a> Visitor<'a> for MonitorConfigPolicyAttributeResponseVisitor {
            type Value = MonitorConfigPolicyAttributeResponse;

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
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "policy" => {
                            if v.is_null() {
                                continue;
                            }
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
                            if v.is_null() {
                                continue;
                            }
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
                        &_ => {}
                    }
                }

                let content = MonitorConfigPolicyAttributeResponse {
                    policy,
                    policy_type,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorConfigPolicyAttributeResponseVisitor)
    }
}
