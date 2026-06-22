// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single ECS remediation investigation: a detected issue together with its proposed plan, history, and ECS workload metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RemediationInvestigation {
    /// A linked code session (for example, a pull request) for the investigation.
    #[serde(rename = "code_session")]
    pub code_session: Option<crate::datadogV2::model::RemediationCodeSession>,
    /// Creation time in epoch milliseconds (64-bit integer encoded as a string).
    #[serde(rename = "created_at_ms")]
    pub created_at_ms: Option<String>,
    /// The guardrail decision applied to a plan or investigation.
    #[serde(rename = "guardrail_decision")]
    pub guardrail_decision: Option<crate::datadogV2::model::RemediationGuardrailDecision>,
    /// Ordered list of history events for the investigation.
    #[serde(rename = "history")]
    pub history: Option<Vec<crate::datadogV2::model::RemediationHistoryEvent>>,
    /// Unique investigation ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The detected issue type.
    #[serde(rename = "issue_type")]
    pub issue_type: Option<String>,
    /// Time of the last action in epoch milliseconds (64-bit integer encoded as a string).
    #[serde(rename = "last_action_at_ms")]
    pub last_action_at_ms: Option<String>,
    /// ECS-specific context for the investigation.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV2::model::RemediationEcsMetadata>,
    /// Datadog organization ID that owns the investigation (64-bit integer encoded as a string).
    #[serde(rename = "org_id")]
    pub org_id: Option<String>,
    /// The remediation plan proposed by the ECS remediation agent.
    #[serde(rename = "plan")]
    pub plan: Option<crate::datadogV2::model::RemediationPlan>,
    /// ARN of the ECS resource the investigation is about.
    #[serde(rename = "resource_arn")]
    pub resource_arn: Option<String>,
    /// Investigation status.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::RemediationInvestigationStatus>,
    /// Last update time in epoch milliseconds (64-bit integer encoded as a string).
    #[serde(rename = "updated_at_ms")]
    pub updated_at_ms: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RemediationInvestigation {
    pub fn new() -> RemediationInvestigation {
        RemediationInvestigation {
            code_session: None,
            created_at_ms: None,
            guardrail_decision: None,
            history: None,
            id: None,
            issue_type: None,
            last_action_at_ms: None,
            metadata: None,
            org_id: None,
            plan: None,
            resource_arn: None,
            status: None,
            updated_at_ms: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn code_session(mut self, value: crate::datadogV2::model::RemediationCodeSession) -> Self {
        self.code_session = Some(value);
        self
    }

    pub fn created_at_ms(mut self, value: String) -> Self {
        self.created_at_ms = Some(value);
        self
    }

    pub fn guardrail_decision(
        mut self,
        value: crate::datadogV2::model::RemediationGuardrailDecision,
    ) -> Self {
        self.guardrail_decision = Some(value);
        self
    }

    pub fn history(mut self, value: Vec<crate::datadogV2::model::RemediationHistoryEvent>) -> Self {
        self.history = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn issue_type(mut self, value: String) -> Self {
        self.issue_type = Some(value);
        self
    }

    pub fn last_action_at_ms(mut self, value: String) -> Self {
        self.last_action_at_ms = Some(value);
        self
    }

    pub fn metadata(mut self, value: crate::datadogV2::model::RemediationEcsMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn org_id(mut self, value: String) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn plan(mut self, value: crate::datadogV2::model::RemediationPlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn resource_arn(mut self, value: String) -> Self {
        self.resource_arn = Some(value);
        self
    }

    pub fn status(
        mut self,
        value: crate::datadogV2::model::RemediationInvestigationStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at_ms(mut self, value: String) -> Self {
        self.updated_at_ms = Some(value);
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

impl Default for RemediationInvestigation {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RemediationInvestigation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RemediationInvestigationVisitor;
        impl<'a> Visitor<'a> for RemediationInvestigationVisitor {
            type Value = RemediationInvestigation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut code_session: Option<crate::datadogV2::model::RemediationCodeSession> =
                    None;
                let mut created_at_ms: Option<String> = None;
                let mut guardrail_decision: Option<
                    crate::datadogV2::model::RemediationGuardrailDecision,
                > = None;
                let mut history: Option<Vec<crate::datadogV2::model::RemediationHistoryEvent>> =
                    None;
                let mut id: Option<String> = None;
                let mut issue_type: Option<String> = None;
                let mut last_action_at_ms: Option<String> = None;
                let mut metadata: Option<crate::datadogV2::model::RemediationEcsMetadata> = None;
                let mut org_id: Option<String> = None;
                let mut plan: Option<crate::datadogV2::model::RemediationPlan> = None;
                let mut resource_arn: Option<String> = None;
                let mut status: Option<crate::datadogV2::model::RemediationInvestigationStatus> =
                    None;
                let mut updated_at_ms: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "code_session" => {
                            if v.is_null() {
                                continue;
                            }
                            code_session =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at_ms" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "guardrail_decision" => {
                            if v.is_null() {
                                continue;
                            }
                            guardrail_decision =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "history" => {
                            if v.is_null() {
                                continue;
                            }
                            history = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issue_type" => {
                            if v.is_null() {
                                continue;
                            }
                            issue_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_action_at_ms" => {
                            if v.is_null() {
                                continue;
                            }
                            last_action_at_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            if v.is_null() {
                                continue;
                            }
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "plan" => {
                            if v.is_null() {
                                continue;
                            }
                            plan = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_arn" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_arn =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::RemediationInvestigationStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "updated_at_ms" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RemediationInvestigation {
                    code_session,
                    created_at_ms,
                    guardrail_decision,
                    history,
                    id,
                    issue_type,
                    last_action_at_ms,
                    metadata,
                    org_id,
                    plan,
                    resource_arn,
                    status,
                    updated_at_ms,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RemediationInvestigationVisitor)
    }
}
