// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for a deployment gate evaluation result response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeploymentGatesEvaluationResultResponseAttributes {
    /// Whether the gate was evaluated in dry-run mode.
    #[serde(rename = "dry_run")]
    pub dry_run: bool,
    /// The unique identifier of the gate evaluation.
    #[serde(rename = "evaluation_id")]
    pub evaluation_id: String,
    /// A URL to view the evaluation details in the Datadog UI.
    #[serde(rename = "evaluation_url")]
    pub evaluation_url: String,
    /// The unique identifier of the deployment gate.
    #[serde(rename = "gate_id")]
    pub gate_id: uuid::Uuid,
    /// The overall status of the gate evaluation.
    /// - `in_progress`: The evaluation is still running.
    /// - `pass`: All rules passed successfully and the deployment is allowed to proceed.
    /// - `fail`: One or more rules did not pass; the deployment should not proceed.
    #[serde(rename = "gate_status")]
    pub gate_status:
        crate::datadogV2::model::DeploymentGatesEvaluationResultResponseAttributesGateStatus,
    /// The results of individual rule evaluations.
    #[serde(rename = "rules")]
    pub rules: Vec<crate::datadogV2::model::DeploymentGatesRuleResponse>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DeploymentGatesEvaluationResultResponseAttributes {
    pub fn new(
        dry_run: bool,
        evaluation_id: String,
        evaluation_url: String,
        gate_id: uuid::Uuid,
        gate_status: crate::datadogV2::model::DeploymentGatesEvaluationResultResponseAttributesGateStatus,
        rules: Vec<crate::datadogV2::model::DeploymentGatesRuleResponse>,
    ) -> DeploymentGatesEvaluationResultResponseAttributes {
        DeploymentGatesEvaluationResultResponseAttributes {
            dry_run,
            evaluation_id,
            evaluation_url,
            gate_id,
            gate_status,
            rules,
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

impl<'de> Deserialize<'de> for DeploymentGatesEvaluationResultResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeploymentGatesEvaluationResultResponseAttributesVisitor;
        impl<'a> Visitor<'a> for DeploymentGatesEvaluationResultResponseAttributesVisitor {
            type Value = DeploymentGatesEvaluationResultResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dry_run: Option<bool> = None;
                let mut evaluation_id: Option<String> = None;
                let mut evaluation_url: Option<String> = None;
                let mut gate_id: Option<uuid::Uuid> = None;
                let mut gate_status: Option<crate::datadogV2::model::DeploymentGatesEvaluationResultResponseAttributesGateStatus> = None;
                let mut rules: Option<Vec<crate::datadogV2::model::DeploymentGatesRuleResponse>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dry_run" => {
                            dry_run = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "evaluation_id" => {
                            evaluation_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "evaluation_url" => {
                            evaluation_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gate_id" => {
                            gate_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gate_status" => {
                            gate_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _gate_status) = gate_status {
                                match _gate_status {
                                    crate::datadogV2::model::DeploymentGatesEvaluationResultResponseAttributesGateStatus::UnparsedObject(_gate_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "rules" => {
                            rules = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let dry_run = dry_run.ok_or_else(|| M::Error::missing_field("dry_run"))?;
                let evaluation_id =
                    evaluation_id.ok_or_else(|| M::Error::missing_field("evaluation_id"))?;
                let evaluation_url =
                    evaluation_url.ok_or_else(|| M::Error::missing_field("evaluation_url"))?;
                let gate_id = gate_id.ok_or_else(|| M::Error::missing_field("gate_id"))?;
                let gate_status =
                    gate_status.ok_or_else(|| M::Error::missing_field("gate_status"))?;
                let rules = rules.ok_or_else(|| M::Error::missing_field("rules"))?;

                let content = DeploymentGatesEvaluationResultResponseAttributes {
                    dry_run,
                    evaluation_id,
                    evaluation_url,
                    gate_id,
                    gate_status,
                    rules,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DeploymentGatesEvaluationResultResponseAttributesVisitor)
    }
}
