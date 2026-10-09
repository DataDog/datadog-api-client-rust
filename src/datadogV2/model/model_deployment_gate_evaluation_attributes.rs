// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a deployment gate evaluation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeploymentGateEvaluationAttributes {
    /// Whether this evaluation used gate-level dry run.
    #[serde(rename = "dry_run")]
    pub dry_run: bool,
    /// Evaluation duration in seconds. Null while it is in progress.
    #[serialize_always]
    #[serde(rename = "duration_seconds")]
    pub duration_seconds: Option<i64>,
    /// Deployment environment evaluated by the gate.
    #[serde(rename = "env")]
    pub env: String,
    /// Gate evaluation UUID. Matches the resource `id`.
    #[serde(rename = "evaluation_id")]
    pub evaluation_id: uuid::Uuid,
    /// Time the evaluation finished. Null while it is in progress.
    #[serialize_always]
    #[serde(rename = "finished_at")]
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Configured deployment gate UUID. Null for just-in-time evaluations.
    #[serialize_always]
    #[serde(rename = "gate_id")]
    pub gate_id: Option<uuid::Uuid>,
    /// Deployment gate identifier.
    #[serde(rename = "identifier")]
    pub identifier: String,
    /// Service evaluated by the deployment gate.
    #[serde(rename = "service")]
    pub service: String,
    /// Time the evaluation started.
    #[serde(rename = "started_at")]
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// The recorded result of a gate or rule evaluation.
    /// - `in_progress`: The evaluation is still running.
    /// - `pass`: All rules passed successfully.
    /// - `fail`: One or more rules did not pass.
    #[serde(rename = "status")]
    pub status:
        crate::datadogV2::model::DeploymentGatesEvaluationResultResponseAttributesGateStatus,
    /// Evaluated deployment version. Empty when no version was provided.
    #[serde(rename = "version")]
    pub version: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DeploymentGateEvaluationAttributes {
    pub fn new(
        dry_run: bool,
        duration_seconds: Option<i64>,
        env: String,
        evaluation_id: uuid::Uuid,
        finished_at: Option<chrono::DateTime<chrono::Utc>>,
        gate_id: Option<uuid::Uuid>,
        identifier: String,
        service: String,
        started_at: chrono::DateTime<chrono::Utc>,
        status: crate::datadogV2::model::DeploymentGatesEvaluationResultResponseAttributesGateStatus,
        version: String,
    ) -> DeploymentGateEvaluationAttributes {
        DeploymentGateEvaluationAttributes {
            dry_run,
            duration_seconds,
            env,
            evaluation_id,
            finished_at,
            gate_id,
            identifier,
            service,
            started_at,
            status,
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

impl<'de> Deserialize<'de> for DeploymentGateEvaluationAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeploymentGateEvaluationAttributesVisitor;
        impl<'a> Visitor<'a> for DeploymentGateEvaluationAttributesVisitor {
            type Value = DeploymentGateEvaluationAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dry_run: Option<bool> = None;
                let mut duration_seconds: Option<Option<i64>> = None;
                let mut env: Option<String> = None;
                let mut evaluation_id: Option<uuid::Uuid> = None;
                let mut finished_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut gate_id: Option<Option<uuid::Uuid>> = None;
                let mut identifier: Option<String> = None;
                let mut service: Option<String> = None;
                let mut started_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut status: Option<crate::datadogV2::model::DeploymentGatesEvaluationResultResponseAttributesGateStatus> = None;
                let mut version: Option<String> = None;
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
                        "finished_at" => {
                            finished_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gate_id" => {
                            gate_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "identifier" => {
                            identifier = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let dry_run = dry_run.ok_or_else(|| M::Error::missing_field("dry_run"))?;
                let duration_seconds =
                    duration_seconds.ok_or_else(|| M::Error::missing_field("duration_seconds"))?;
                let env = env.ok_or_else(|| M::Error::missing_field("env"))?;
                let evaluation_id =
                    evaluation_id.ok_or_else(|| M::Error::missing_field("evaluation_id"))?;
                let finished_at =
                    finished_at.ok_or_else(|| M::Error::missing_field("finished_at"))?;
                let gate_id = gate_id.ok_or_else(|| M::Error::missing_field("gate_id"))?;
                let identifier = identifier.ok_or_else(|| M::Error::missing_field("identifier"))?;
                let service = service.ok_or_else(|| M::Error::missing_field("service"))?;
                let started_at = started_at.ok_or_else(|| M::Error::missing_field("started_at"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = DeploymentGateEvaluationAttributes {
                    dry_run,
                    duration_seconds,
                    env,
                    evaluation_id,
                    finished_at,
                    gate_id,
                    identifier,
                    service,
                    started_at,
                    status,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DeploymentGateEvaluationAttributesVisitor)
    }
}
