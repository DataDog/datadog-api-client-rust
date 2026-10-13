// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a deployment gate rule evaluation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeploymentGateRuleEvaluationAttributes {
    /// Evaluated rule configuration. Fields depend on rule type and unset fields are omitted.
    /// Monitor rules can include `duration`, `query`, `monitor_ids`, `warmup`, `fail_on_no_groups_found`, and `fail_on_no_data`.
    /// Faulty deployment detection rules can include `duration`, `allowed_resources`, and `excluded_resources`.
    #[serde(rename = "configuration")]
    pub configuration: crate::datadogV2::model::DeploymentGateRuleEvaluationConfiguration,
    /// Whether this rule is non-enforcing. A failed dry-run rule is ignored when computing the gate outcome. Independent of `gate_dry_run`.
    #[serde(rename = "dry_run")]
    pub dry_run: bool,
    /// Rule evaluation duration in seconds. Null while it is in progress.
    #[serialize_always]
    #[serde(rename = "duration_seconds")]
    pub duration_seconds: Option<i64>,
    /// Evaluated environment.
    #[serde(rename = "env")]
    pub env: String,
    /// Rule evaluation UUID. Matches the resource `id`.
    #[serde(rename = "evaluation_id")]
    pub evaluation_id: uuid::Uuid,
    /// Rule failure details.
    #[serde(rename = "failures")]
    pub failures: crate::datadogV2::model::DeploymentGateRuleFailures,
    /// Time the rule evaluation finished. Null while it is in progress.
    #[serialize_always]
    #[serde(rename = "finished_at")]
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Whether the parent gate is dry-run. A failed dry-run gate blocks but does not stop deployment. Independent of rule-level `dry_run`.
    #[serde(rename = "gate_dry_run")]
    pub gate_dry_run: bool,
    /// Deployment gate evaluation UUID.
    #[serde(rename = "gate_evaluation_id")]
    pub gate_evaluation_id: uuid::Uuid,
    /// Configured deployment gate UUID. Null for just-in-time evaluations.
    #[serialize_always]
    #[serde(rename = "gate_id")]
    pub gate_id: Option<uuid::Uuid>,
    /// Deployment gate identifier.
    #[serde(rename = "identifier")]
    pub identifier: String,
    /// Rule name.
    #[serde(rename = "name")]
    pub name: String,
    /// Reason for the rule result.
    #[serde(rename = "reason")]
    pub reason: String,
    /// Configured deployment rule UUID. Null for just-in-time rules.
    #[serialize_always]
    #[serde(rename = "rule_id")]
    pub rule_id: Option<uuid::Uuid>,
    /// Evaluated service.
    #[serde(rename = "service")]
    pub service: String,
    /// Time the rule evaluation started.
    #[serde(rename = "started_at")]
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// The recorded result of a gate or rule evaluation.
    /// - `in_progress`: The evaluation is still running.
    /// - `pass`: All rules passed successfully.
    /// - `fail`: One or more rules did not pass.
    #[serde(rename = "status")]
    pub status:
        crate::datadogV2::model::DeploymentGatesEvaluationResultResponseAttributesGateStatus,
    /// Type of deployment gate rule.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::DeploymentGateRuleEvaluationType,
    /// Evaluated deployment version. Empty when no version was provided.
    #[serde(rename = "version")]
    pub version: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DeploymentGateRuleEvaluationAttributes {
    pub fn new(
        configuration: crate::datadogV2::model::DeploymentGateRuleEvaluationConfiguration,
        dry_run: bool,
        duration_seconds: Option<i64>,
        env: String,
        evaluation_id: uuid::Uuid,
        failures: crate::datadogV2::model::DeploymentGateRuleFailures,
        finished_at: Option<chrono::DateTime<chrono::Utc>>,
        gate_dry_run: bool,
        gate_evaluation_id: uuid::Uuid,
        gate_id: Option<uuid::Uuid>,
        identifier: String,
        name: String,
        reason: String,
        rule_id: Option<uuid::Uuid>,
        service: String,
        started_at: chrono::DateTime<chrono::Utc>,
        status: crate::datadogV2::model::DeploymentGatesEvaluationResultResponseAttributesGateStatus,
        type_: crate::datadogV2::model::DeploymentGateRuleEvaluationType,
        version: String,
    ) -> DeploymentGateRuleEvaluationAttributes {
        DeploymentGateRuleEvaluationAttributes {
            configuration,
            dry_run,
            duration_seconds,
            env,
            evaluation_id,
            failures,
            finished_at,
            gate_dry_run,
            gate_evaluation_id,
            gate_id,
            identifier,
            name,
            reason,
            rule_id,
            service,
            started_at,
            status,
            type_,
            version,
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

impl<'de> Deserialize<'de> for DeploymentGateRuleEvaluationAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeploymentGateRuleEvaluationAttributesVisitor;
        impl<'a> Visitor<'a> for DeploymentGateRuleEvaluationAttributesVisitor {
            type Value = DeploymentGateRuleEvaluationAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut configuration: Option<
                    crate::datadogV2::model::DeploymentGateRuleEvaluationConfiguration,
                > = None;
                let mut dry_run: Option<bool> = None;
                let mut duration_seconds: Option<Option<i64>> = None;
                let mut env: Option<String> = None;
                let mut evaluation_id: Option<uuid::Uuid> = None;
                let mut failures: Option<crate::datadogV2::model::DeploymentGateRuleFailures> =
                    None;
                let mut finished_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut gate_dry_run: Option<bool> = None;
                let mut gate_evaluation_id: Option<uuid::Uuid> = None;
                let mut gate_id: Option<Option<uuid::Uuid>> = None;
                let mut identifier: Option<String> = None;
                let mut name: Option<String> = None;
                let mut reason: Option<String> = None;
                let mut rule_id: Option<Option<uuid::Uuid>> = None;
                let mut service: Option<String> = None;
                let mut started_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut status: Option<crate::datadogV2::model::DeploymentGatesEvaluationResultResponseAttributesGateStatus> = None;
                let mut type_: Option<crate::datadogV2::model::DeploymentGateRuleEvaluationType> =
                    None;
                let mut version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "configuration" => {
                            configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dry_run" => {
                            dry_run = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "duration_seconds" => {
                            duration_seconds =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "env" => {
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "evaluation_id" => {
                            evaluation_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "failures" => {
                            failures = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "finished_at" => {
                            finished_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gate_dry_run" => {
                            gate_dry_run =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gate_evaluation_id" => {
                            gate_evaluation_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gate_id" => {
                            gate_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "identifier" => {
                            identifier = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reason" => {
                            reason = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_id" => {
                            rule_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "started_at" => {
                            started_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::DeploymentGatesEvaluationResultResponseAttributesGateStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::DeploymentGateRuleEvaluationType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let configuration =
                    configuration.ok_or_else(|| M::Error::missing_field("configuration"))?;
                let dry_run = dry_run.ok_or_else(|| M::Error::missing_field("dry_run"))?;
                let duration_seconds =
                    duration_seconds.ok_or_else(|| M::Error::missing_field("duration_seconds"))?;
                let env = env.ok_or_else(|| M::Error::missing_field("env"))?;
                let evaluation_id =
                    evaluation_id.ok_or_else(|| M::Error::missing_field("evaluation_id"))?;
                let failures = failures.ok_or_else(|| M::Error::missing_field("failures"))?;
                let finished_at =
                    finished_at.ok_or_else(|| M::Error::missing_field("finished_at"))?;
                let gate_dry_run =
                    gate_dry_run.ok_or_else(|| M::Error::missing_field("gate_dry_run"))?;
                let gate_evaluation_id = gate_evaluation_id
                    .ok_or_else(|| M::Error::missing_field("gate_evaluation_id"))?;
                let gate_id = gate_id.ok_or_else(|| M::Error::missing_field("gate_id"))?;
                let identifier = identifier.ok_or_else(|| M::Error::missing_field("identifier"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let reason = reason.ok_or_else(|| M::Error::missing_field("reason"))?;
                let rule_id = rule_id.ok_or_else(|| M::Error::missing_field("rule_id"))?;
                let service = service.ok_or_else(|| M::Error::missing_field("service"))?;
                let started_at = started_at.ok_or_else(|| M::Error::missing_field("started_at"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = DeploymentGateRuleEvaluationAttributes {
                    configuration,
                    dry_run,
                    duration_seconds,
                    env,
                    evaluation_id,
                    failures,
                    finished_at,
                    gate_dry_run,
                    gate_evaluation_id,
                    gate_id,
                    identifier,
                    name,
                    reason,
                    rule_id,
                    service,
                    started_at,
                    status,
                    type_,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DeploymentGateRuleEvaluationAttributesVisitor)
    }
}
