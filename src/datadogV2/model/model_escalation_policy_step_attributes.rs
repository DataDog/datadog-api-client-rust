// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines attributes for an escalation policy step, such as assignment strategy and escalation timeout.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EscalationPolicyStepAttributes {
    /// Specifies how this escalation step will assign targets (example `default` or `round-robin`).
    #[serde(rename = "assignment")]
    pub assignment: Option<crate::datadogV2::model::EscalationPolicyStepAttributesAssignment>,
    /// Specifies how many seconds to wait before escalating to the next step.
    #[serde(rename = "escalate_after_seconds")]
    pub escalate_after_seconds: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EscalationPolicyStepAttributes {
    pub fn new() -> EscalationPolicyStepAttributes {
        EscalationPolicyStepAttributes {
            assignment: None,
            escalate_after_seconds: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assignment(
        mut self,
        value: crate::datadogV2::model::EscalationPolicyStepAttributesAssignment,
    ) -> Self {
        self.assignment = Some(value);
        self
    }

    pub fn escalate_after_seconds(mut self, value: i64) -> Self {
        self.escalate_after_seconds = Some(value);
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

impl Default for EscalationPolicyStepAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EscalationPolicyStepAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EscalationPolicyStepAttributesVisitor;
        impl<'a> Visitor<'a> for EscalationPolicyStepAttributesVisitor {
            type Value = EscalationPolicyStepAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assignment: Option<
                    crate::datadogV2::model::EscalationPolicyStepAttributesAssignment,
                > = None;
                let mut escalate_after_seconds: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assignment" => {
                            if v.is_null() {
                                continue;
                            }
                            assignment = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _assignment) = assignment {
                                match _assignment {
                                    crate::datadogV2::model::EscalationPolicyStepAttributesAssignment::UnparsedObject(_assignment) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "escalate_after_seconds" => {
                            if v.is_null() {
                                continue;
                            }
                            escalate_after_seconds =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = EscalationPolicyStepAttributes {
                    assignment,
                    escalate_after_seconds,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EscalationPolicyStepAttributesVisitor)
    }
}
