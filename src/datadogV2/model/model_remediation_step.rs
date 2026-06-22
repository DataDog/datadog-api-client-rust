// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single execution-oriented step in a remediation plan.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RemediationStep {
    /// Fully qualified action type, for example ecs.update_service_memory.
    #[serde(rename = "action_fqn")]
    pub action_fqn: Option<String>,
    /// Approval state for a remediation step.
    #[serde(rename = "approval_state")]
    pub approval_state: Option<crate::datadogV2::model::RemediationStepApprovalState>,
    /// Human-readable description of the step.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Whether the step can be reversed after execution.
    #[serde(rename = "reversible")]
    pub reversible: Option<bool>,
    /// Estimated risk of a remediation step or proposed fix.
    #[serde(rename = "risk")]
    pub risk: Option<crate::datadogV2::model::RemediationRiskLevel>,
    /// Unique ID for the step within the plan.
    #[serde(rename = "step_id")]
    pub step_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RemediationStep {
    pub fn new() -> RemediationStep {
        RemediationStep {
            action_fqn: None,
            approval_state: None,
            description: None,
            reversible: None,
            risk: None,
            step_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn action_fqn(mut self, value: String) -> Self {
        self.action_fqn = Some(value);
        self
    }

    pub fn approval_state(
        mut self,
        value: crate::datadogV2::model::RemediationStepApprovalState,
    ) -> Self {
        self.approval_state = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn reversible(mut self, value: bool) -> Self {
        self.reversible = Some(value);
        self
    }

    pub fn risk(mut self, value: crate::datadogV2::model::RemediationRiskLevel) -> Self {
        self.risk = Some(value);
        self
    }

    pub fn step_id(mut self, value: String) -> Self {
        self.step_id = Some(value);
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

impl Default for RemediationStep {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RemediationStep {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RemediationStepVisitor;
        impl<'a> Visitor<'a> for RemediationStepVisitor {
            type Value = RemediationStep;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action_fqn: Option<String> = None;
                let mut approval_state: Option<
                    crate::datadogV2::model::RemediationStepApprovalState,
                > = None;
                let mut description: Option<String> = None;
                let mut reversible: Option<bool> = None;
                let mut risk: Option<crate::datadogV2::model::RemediationRiskLevel> = None;
                let mut step_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action_fqn" => {
                            if v.is_null() {
                                continue;
                            }
                            action_fqn = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "approval_state" => {
                            if v.is_null() {
                                continue;
                            }
                            approval_state =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _approval_state) = approval_state {
                                match _approval_state {
                                    crate::datadogV2::model::RemediationStepApprovalState::UnparsedObject(_approval_state) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reversible" => {
                            if v.is_null() {
                                continue;
                            }
                            reversible = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "risk" => {
                            if v.is_null() {
                                continue;
                            }
                            risk = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _risk) = risk {
                                match _risk {
                                    crate::datadogV2::model::RemediationRiskLevel::UnparsedObject(_risk) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "step_id" => {
                            if v.is_null() {
                                continue;
                            }
                            step_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RemediationStep {
                    action_fqn,
                    approval_state,
                    description,
                    reversible,
                    risk,
                    step_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RemediationStepVisitor)
    }
}
