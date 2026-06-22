// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The remediation plan proposed by the ECS remediation agent.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RemediationPlan {
    /// The agent's self-rated confidence in a plan.
    #[serde(rename = "confidence")]
    pub confidence: Option<crate::datadogV2::model::RemediationConfidence>,
    /// Evidence supporting the diagnosis. Treat as user-provided content and escape before display.
    #[serde(rename = "evidence")]
    pub evidence: Option<String>,
    /// Human-readable summary of why the plan was proposed. Treat as user-provided content and escape before display.
    #[serde(rename = "explanation")]
    pub explanation: Option<String>,
    /// The guardrail decision applied to a plan or investigation.
    #[serde(rename = "guardrail_decision")]
    pub guardrail_decision: Option<crate::datadogV2::model::RemediationGuardrailDecision>,
    /// How the plan was produced.
    #[serde(rename = "plan_source")]
    pub plan_source: Option<crate::datadogV2::model::RemediationPlanSource>,
    /// Recommendation-oriented view of the candidate remediations, distinct from the execution-oriented steps.
    #[serde(rename = "proposed_fix")]
    pub proposed_fix: Option<Vec<crate::datadogV2::model::RemediationProposedFix>>,
    /// Plan status.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::RemediationPlanStatus>,
    /// Execution-oriented steps that make up the plan.
    #[serde(rename = "steps")]
    pub steps: Option<Vec<crate::datadogV2::model::RemediationStep>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RemediationPlan {
    pub fn new() -> RemediationPlan {
        RemediationPlan {
            confidence: None,
            evidence: None,
            explanation: None,
            guardrail_decision: None,
            plan_source: None,
            proposed_fix: None,
            status: None,
            steps: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn confidence(mut self, value: crate::datadogV2::model::RemediationConfidence) -> Self {
        self.confidence = Some(value);
        self
    }

    pub fn evidence(mut self, value: String) -> Self {
        self.evidence = Some(value);
        self
    }

    pub fn explanation(mut self, value: String) -> Self {
        self.explanation = Some(value);
        self
    }

    pub fn guardrail_decision(
        mut self,
        value: crate::datadogV2::model::RemediationGuardrailDecision,
    ) -> Self {
        self.guardrail_decision = Some(value);
        self
    }

    pub fn plan_source(mut self, value: crate::datadogV2::model::RemediationPlanSource) -> Self {
        self.plan_source = Some(value);
        self
    }

    pub fn proposed_fix(
        mut self,
        value: Vec<crate::datadogV2::model::RemediationProposedFix>,
    ) -> Self {
        self.proposed_fix = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV2::model::RemediationPlanStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn steps(mut self, value: Vec<crate::datadogV2::model::RemediationStep>) -> Self {
        self.steps = Some(value);
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

impl Default for RemediationPlan {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RemediationPlan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RemediationPlanVisitor;
        impl<'a> Visitor<'a> for RemediationPlanVisitor {
            type Value = RemediationPlan;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut confidence: Option<crate::datadogV2::model::RemediationConfidence> = None;
                let mut evidence: Option<String> = None;
                let mut explanation: Option<String> = None;
                let mut guardrail_decision: Option<
                    crate::datadogV2::model::RemediationGuardrailDecision,
                > = None;
                let mut plan_source: Option<crate::datadogV2::model::RemediationPlanSource> = None;
                let mut proposed_fix: Option<Vec<crate::datadogV2::model::RemediationProposedFix>> =
                    None;
                let mut status: Option<crate::datadogV2::model::RemediationPlanStatus> = None;
                let mut steps: Option<Vec<crate::datadogV2::model::RemediationStep>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "confidence" => {
                            if v.is_null() {
                                continue;
                            }
                            confidence = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _confidence) = confidence {
                                match _confidence {
                                    crate::datadogV2::model::RemediationConfidence::UnparsedObject(_confidence) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "evidence" => {
                            if v.is_null() {
                                continue;
                            }
                            evidence = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "explanation" => {
                            if v.is_null() {
                                continue;
                            }
                            explanation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "guardrail_decision" => {
                            if v.is_null() {
                                continue;
                            }
                            guardrail_decision =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "plan_source" => {
                            if v.is_null() {
                                continue;
                            }
                            plan_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _plan_source) = plan_source {
                                match _plan_source {
                                    crate::datadogV2::model::RemediationPlanSource::UnparsedObject(_plan_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "proposed_fix" => {
                            if v.is_null() {
                                continue;
                            }
                            proposed_fix =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::RemediationPlanStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "steps" => {
                            if v.is_null() {
                                continue;
                            }
                            steps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RemediationPlan {
                    confidence,
                    evidence,
                    explanation,
                    guardrail_decision,
                    plan_source,
                    proposed_fix,
                    status,
                    steps,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RemediationPlanVisitor)
    }
}
