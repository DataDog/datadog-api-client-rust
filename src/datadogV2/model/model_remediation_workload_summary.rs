// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Aggregated health across all tasks in a workload (service or daemon).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RemediationWorkloadSummary {
    /// ECS deployment state, populated only for deployment failures.
    #[serde(rename = "deployment_rollout_state")]
    pub deployment_rollout_state:
        Option<crate::datadogV2::model::RemediationDeploymentRolloutState>,
    /// Expected task count (64-bit integer encoded as a string).
    #[serde(rename = "desired_count")]
    pub desired_count: Option<String>,
    /// Tasks in a problematic state (64-bit integer encoded as a string).
    #[serde(rename = "failed_count")]
    pub failed_count: Option<String>,
    /// Percentage of desired count that is failed (64-bit integer encoded as a string).
    #[serde(rename = "failed_percent")]
    pub failed_percent: Option<String>,
    /// Tasks currently pending (64-bit integer encoded as a string).
    #[serde(rename = "pending_count")]
    pub pending_count: Option<String>,
    /// Previous deployment's task definition, as family:revision or a full task definition ARN. Empty when no rollback target is visible.
    #[serde(rename = "previous_task_definition")]
    pub previous_task_definition: Option<String>,
    /// Tasks currently running (64-bit integer encoded as a string).
    #[serde(rename = "running_count")]
    pub running_count: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RemediationWorkloadSummary {
    pub fn new() -> RemediationWorkloadSummary {
        RemediationWorkloadSummary {
            deployment_rollout_state: None,
            desired_count: None,
            failed_count: None,
            failed_percent: None,
            pending_count: None,
            previous_task_definition: None,
            running_count: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn deployment_rollout_state(
        mut self,
        value: crate::datadogV2::model::RemediationDeploymentRolloutState,
    ) -> Self {
        self.deployment_rollout_state = Some(value);
        self
    }

    pub fn desired_count(mut self, value: String) -> Self {
        self.desired_count = Some(value);
        self
    }

    pub fn failed_count(mut self, value: String) -> Self {
        self.failed_count = Some(value);
        self
    }

    pub fn failed_percent(mut self, value: String) -> Self {
        self.failed_percent = Some(value);
        self
    }

    pub fn pending_count(mut self, value: String) -> Self {
        self.pending_count = Some(value);
        self
    }

    pub fn previous_task_definition(mut self, value: String) -> Self {
        self.previous_task_definition = Some(value);
        self
    }

    pub fn running_count(mut self, value: String) -> Self {
        self.running_count = Some(value);
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

impl Default for RemediationWorkloadSummary {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RemediationWorkloadSummary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RemediationWorkloadSummaryVisitor;
        impl<'a> Visitor<'a> for RemediationWorkloadSummaryVisitor {
            type Value = RemediationWorkloadSummary;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut deployment_rollout_state: Option<
                    crate::datadogV2::model::RemediationDeploymentRolloutState,
                > = None;
                let mut desired_count: Option<String> = None;
                let mut failed_count: Option<String> = None;
                let mut failed_percent: Option<String> = None;
                let mut pending_count: Option<String> = None;
                let mut previous_task_definition: Option<String> = None;
                let mut running_count: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "deployment_rollout_state" => {
                            if v.is_null() {
                                continue;
                            }
                            deployment_rollout_state =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _deployment_rollout_state) = deployment_rollout_state {
                                match _deployment_rollout_state {
                                    crate::datadogV2::model::RemediationDeploymentRolloutState::UnparsedObject(_deployment_rollout_state) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "desired_count" => {
                            if v.is_null() {
                                continue;
                            }
                            desired_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "failed_count" => {
                            if v.is_null() {
                                continue;
                            }
                            failed_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "failed_percent" => {
                            if v.is_null() {
                                continue;
                            }
                            failed_percent =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pending_count" => {
                            if v.is_null() {
                                continue;
                            }
                            pending_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "previous_task_definition" => {
                            if v.is_null() {
                                continue;
                            }
                            previous_task_definition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "running_count" => {
                            if v.is_null() {
                                continue;
                            }
                            running_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RemediationWorkloadSummary {
                    deployment_rollout_state,
                    desired_count,
                    failed_count,
                    failed_percent,
                    pending_count,
                    previous_task_definition,
                    running_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RemediationWorkloadSummaryVisitor)
    }
}
