// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// ECS-specific context for the investigation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RemediationEcsMetadata {
    /// AWS account ID.
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// Associated capacity providers.
    #[serde(rename = "capacity_providers")]
    pub capacity_providers: Option<Vec<String>>,
    /// ECS cluster ARN.
    #[serde(rename = "cluster_arn")]
    pub cluster_arn: Option<String>,
    /// ECS cluster name.
    #[serde(rename = "cluster_name")]
    pub cluster_name: Option<String>,
    /// Computed impact score for ranking (64-bit integer encoded as a string).
    #[serde(rename = "impact_score")]
    pub impact_score: Option<String>,
    /// When the issue was first detected, epoch milliseconds (64-bit integer encoded as a string).
    #[serde(rename = "issue_start_time")]
    pub issue_start_time: Option<String>,
    /// ECS launch type.
    #[serde(rename = "launch_type")]
    pub launch_type: Option<crate::datadogV2::model::RemediationLaunchType>,
    /// AWS region.
    #[serde(rename = "region")]
    pub region: Option<String>,
    /// All affected task ARNs, for filtering.
    #[serde(rename = "task_arns")]
    pub task_arns: Option<Vec<String>>,
    /// Task definition family name.
    #[serde(rename = "task_definition_family")]
    pub task_definition_family: Option<String>,
    /// Current task definition revision.
    #[serde(rename = "task_definition_revision")]
    pub task_definition_revision: Option<i64>,
    /// Individual tasks exhibiting issues. Capped at 50 most recent.
    #[serde(rename = "tasks")]
    pub tasks: Option<Vec<crate::datadogV2::model::RemediationProblemTask>>,
    /// Sum of CPU units across affected tasks (64-bit integer encoded as a string).
    #[serde(rename = "total_cpu_units")]
    pub total_cpu_units: Option<String>,
    /// Sum of memory (MiB) across affected tasks (64-bit integer encoded as a string).
    #[serde(rename = "total_memory_mib")]
    pub total_memory_mib: Option<String>,
    /// When this metadata was last updated, epoch milliseconds (64-bit integer encoded as a string).
    #[serde(rename = "update_timestamp")]
    pub update_timestamp: Option<String>,
    /// Aggregated health across all tasks in a workload (service or daemon).
    #[serde(rename = "workload_summary")]
    pub workload_summary: Option<crate::datadogV2::model::RemediationWorkloadSummary>,
    /// The kind of ECS workload that owns the problematic tasks.
    #[serde(rename = "workload_type")]
    pub workload_type: Option<crate::datadogV2::model::RemediationWorkloadType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RemediationEcsMetadata {
    pub fn new() -> RemediationEcsMetadata {
        RemediationEcsMetadata {
            account_id: None,
            capacity_providers: None,
            cluster_arn: None,
            cluster_name: None,
            impact_score: None,
            issue_start_time: None,
            launch_type: None,
            region: None,
            task_arns: None,
            task_definition_family: None,
            task_definition_revision: None,
            tasks: None,
            total_cpu_units: None,
            total_memory_mib: None,
            update_timestamp: None,
            workload_summary: None,
            workload_type: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account_id(mut self, value: String) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn capacity_providers(mut self, value: Vec<String>) -> Self {
        self.capacity_providers = Some(value);
        self
    }

    pub fn cluster_arn(mut self, value: String) -> Self {
        self.cluster_arn = Some(value);
        self
    }

    pub fn cluster_name(mut self, value: String) -> Self {
        self.cluster_name = Some(value);
        self
    }

    pub fn impact_score(mut self, value: String) -> Self {
        self.impact_score = Some(value);
        self
    }

    pub fn issue_start_time(mut self, value: String) -> Self {
        self.issue_start_time = Some(value);
        self
    }

    pub fn launch_type(mut self, value: crate::datadogV2::model::RemediationLaunchType) -> Self {
        self.launch_type = Some(value);
        self
    }

    pub fn region(mut self, value: String) -> Self {
        self.region = Some(value);
        self
    }

    pub fn task_arns(mut self, value: Vec<String>) -> Self {
        self.task_arns = Some(value);
        self
    }

    pub fn task_definition_family(mut self, value: String) -> Self {
        self.task_definition_family = Some(value);
        self
    }

    pub fn task_definition_revision(mut self, value: i64) -> Self {
        self.task_definition_revision = Some(value);
        self
    }

    pub fn tasks(mut self, value: Vec<crate::datadogV2::model::RemediationProblemTask>) -> Self {
        self.tasks = Some(value);
        self
    }

    pub fn total_cpu_units(mut self, value: String) -> Self {
        self.total_cpu_units = Some(value);
        self
    }

    pub fn total_memory_mib(mut self, value: String) -> Self {
        self.total_memory_mib = Some(value);
        self
    }

    pub fn update_timestamp(mut self, value: String) -> Self {
        self.update_timestamp = Some(value);
        self
    }

    pub fn workload_summary(
        mut self,
        value: crate::datadogV2::model::RemediationWorkloadSummary,
    ) -> Self {
        self.workload_summary = Some(value);
        self
    }

    pub fn workload_type(
        mut self,
        value: crate::datadogV2::model::RemediationWorkloadType,
    ) -> Self {
        self.workload_type = Some(value);
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

impl Default for RemediationEcsMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RemediationEcsMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RemediationEcsMetadataVisitor;
        impl<'a> Visitor<'a> for RemediationEcsMetadataVisitor {
            type Value = RemediationEcsMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut capacity_providers: Option<Vec<String>> = None;
                let mut cluster_arn: Option<String> = None;
                let mut cluster_name: Option<String> = None;
                let mut impact_score: Option<String> = None;
                let mut issue_start_time: Option<String> = None;
                let mut launch_type: Option<crate::datadogV2::model::RemediationLaunchType> = None;
                let mut region: Option<String> = None;
                let mut task_arns: Option<Vec<String>> = None;
                let mut task_definition_family: Option<String> = None;
                let mut task_definition_revision: Option<i64> = None;
                let mut tasks: Option<Vec<crate::datadogV2::model::RemediationProblemTask>> = None;
                let mut total_cpu_units: Option<String> = None;
                let mut total_memory_mib: Option<String> = None;
                let mut update_timestamp: Option<String> = None;
                let mut workload_summary: Option<
                    crate::datadogV2::model::RemediationWorkloadSummary,
                > = None;
                let mut workload_type: Option<crate::datadogV2::model::RemediationWorkloadType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            if v.is_null() {
                                continue;
                            }
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "capacity_providers" => {
                            if v.is_null() {
                                continue;
                            }
                            capacity_providers =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cluster_arn" => {
                            if v.is_null() {
                                continue;
                            }
                            cluster_arn =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cluster_name" => {
                            if v.is_null() {
                                continue;
                            }
                            cluster_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "impact_score" => {
                            if v.is_null() {
                                continue;
                            }
                            impact_score =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issue_start_time" => {
                            if v.is_null() {
                                continue;
                            }
                            issue_start_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "launch_type" => {
                            if v.is_null() {
                                continue;
                            }
                            launch_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _launch_type) = launch_type {
                                match _launch_type {
                                    crate::datadogV2::model::RemediationLaunchType::UnparsedObject(_launch_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "region" => {
                            if v.is_null() {
                                continue;
                            }
                            region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "task_arns" => {
                            if v.is_null() {
                                continue;
                            }
                            task_arns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "task_definition_family" => {
                            if v.is_null() {
                                continue;
                            }
                            task_definition_family =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "task_definition_revision" => {
                            if v.is_null() {
                                continue;
                            }
                            task_definition_revision =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tasks" => {
                            if v.is_null() {
                                continue;
                            }
                            tasks = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_cpu_units" => {
                            if v.is_null() {
                                continue;
                            }
                            total_cpu_units =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_memory_mib" => {
                            if v.is_null() {
                                continue;
                            }
                            total_memory_mib =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "update_timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            update_timestamp =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "workload_summary" => {
                            if v.is_null() {
                                continue;
                            }
                            workload_summary =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "workload_type" => {
                            if v.is_null() {
                                continue;
                            }
                            workload_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _workload_type) = workload_type {
                                match _workload_type {
                                    crate::datadogV2::model::RemediationWorkloadType::UnparsedObject(_workload_type) => {
                                        _unparsed = true;
                                    },
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

                let content = RemediationEcsMetadata {
                    account_id,
                    capacity_providers,
                    cluster_arn,
                    cluster_name,
                    impact_score,
                    issue_start_time,
                    launch_type,
                    region,
                    task_arns,
                    task_definition_family,
                    task_definition_revision,
                    tasks,
                    total_cpu_units,
                    total_memory_mib,
                    update_timestamp,
                    workload_summary,
                    workload_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RemediationEcsMetadataVisitor)
    }
}
