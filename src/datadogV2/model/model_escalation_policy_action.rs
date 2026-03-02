// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Triggers an escalation policy when the routing rule matches.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EscalationPolicyAction {
    /// The ID of the escalation policy to trigger.
    #[serde(rename = "policy_id")]
    pub policy_id: String,
    /// Holds time zone information and a list of time restrictions for a routing rule.
    #[serde(rename = "support_hours")]
    pub support_hours: Option<crate::datadogV2::model::TimeRestrictions>,
    /// Indicates that the action is an escalation policy action.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::EscalationPolicyActionType,
    /// Specifies the level of urgency for a routing rule (low, high, or dynamic).
    #[serde(rename = "urgency")]
    pub urgency: Option<crate::datadogV2::model::Urgency>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EscalationPolicyAction {
    pub fn new(
        policy_id: String,
        type_: crate::datadogV2::model::EscalationPolicyActionType,
    ) -> EscalationPolicyAction {
        EscalationPolicyAction {
            policy_id,
            support_hours: None,
            type_,
            urgency: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn support_hours(mut self, value: crate::datadogV2::model::TimeRestrictions) -> Self {
        self.support_hours = Some(value);
        self
    }

    pub fn urgency(mut self, value: crate::datadogV2::model::Urgency) -> Self {
        self.urgency = Some(value);
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

impl<'de> Deserialize<'de> for EscalationPolicyAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EscalationPolicyActionVisitor;
        impl<'a> Visitor<'a> for EscalationPolicyActionVisitor {
            type Value = EscalationPolicyAction;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut policy_id: Option<String> = None;
                let mut support_hours: Option<crate::datadogV2::model::TimeRestrictions> = None;
                let mut type_: Option<crate::datadogV2::model::EscalationPolicyActionType> = None;
                let mut urgency: Option<crate::datadogV2::model::Urgency> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "policy_id" => {
                            policy_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "support_hours" => {
                            if v.is_null() {
                                continue;
                            }
                            support_hours =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::EscalationPolicyActionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "urgency" => {
                            if v.is_null() {
                                continue;
                            }
                            urgency = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _urgency) = urgency {
                                match _urgency {
                                    crate::datadogV2::model::Urgency::UnparsedObject(_urgency) => {
                                        _unparsed = true;
                                    }
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
                let policy_id = policy_id.ok_or_else(|| M::Error::missing_field("policy_id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = EscalationPolicyAction {
                    policy_id,
                    support_hours,
                    type_,
                    urgency,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EscalationPolicyActionVisitor)
    }
}
