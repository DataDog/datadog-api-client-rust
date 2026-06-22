// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An individual ECS task in a problematic state.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RemediationProblemTask {
    /// Availability zone where the task is running.
    #[serde(rename = "availability_zone")]
    pub availability_zone: Option<String>,
    /// Container instance ARN (EC2 launch type only).
    #[serde(rename = "container_instance_arn")]
    pub container_instance_arn: Option<String>,
    /// Problematic containers within the task.
    #[serde(rename = "containers")]
    pub containers: Option<Vec<crate::datadogV2::model::RemediationProblemContainer>>,
    /// CPU units allocated to the task (64-bit integer encoded as a string).
    #[serde(rename = "cpu_units")]
    pub cpu_units: Option<String>,
    /// Desired task status.
    #[serde(rename = "desired_status")]
    pub desired_status: Option<String>,
    /// IAM role used by ECS to pull images and fetch secrets.
    #[serde(rename = "execution_role_arn")]
    pub execution_role_arn: Option<String>,
    /// Task-level health status.
    #[serde(rename = "health_status")]
    pub health_status: Option<String>,
    /// When this task's issue started, epoch milliseconds (64-bit integer encoded as a string).
    #[serde(rename = "issue_start_time")]
    pub issue_start_time: Option<String>,
    /// Issue type for this specific task.
    #[serde(rename = "issue_type")]
    pub issue_type: Option<String>,
    /// Current task status.
    #[serde(rename = "last_status")]
    pub last_status: Option<String>,
    /// ECS launch type.
    #[serde(rename = "launch_type")]
    pub launch_type: Option<crate::datadogV2::model::RemediationLaunchType>,
    /// Memory in MiB allocated to the task (64-bit integer encoded as a string).
    #[serde(rename = "memory_mib")]
    pub memory_mib: Option<String>,
    /// Stop code if the task was stopped.
    #[serde(rename = "stop_code")]
    pub stop_code: Option<String>,
    /// Stop reason if the task was stopped.
    #[serde(rename = "stopped_reason")]
    pub stopped_reason: Option<String>,
    /// Task-level tags.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Full task ARN.
    #[serde(rename = "task_arn")]
    pub task_arn: Option<String>,
    /// Task definition ARN with revision.
    #[serde(rename = "task_definition_arn")]
    pub task_definition_arn: Option<String>,
    /// IAM role used by the task at runtime.
    #[serde(rename = "task_role_arn")]
    pub task_role_arn: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RemediationProblemTask {
    pub fn new() -> RemediationProblemTask {
        RemediationProblemTask {
            availability_zone: None,
            container_instance_arn: None,
            containers: None,
            cpu_units: None,
            desired_status: None,
            execution_role_arn: None,
            health_status: None,
            issue_start_time: None,
            issue_type: None,
            last_status: None,
            launch_type: None,
            memory_mib: None,
            stop_code: None,
            stopped_reason: None,
            tags: None,
            task_arn: None,
            task_definition_arn: None,
            task_role_arn: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn availability_zone(mut self, value: String) -> Self {
        self.availability_zone = Some(value);
        self
    }

    pub fn container_instance_arn(mut self, value: String) -> Self {
        self.container_instance_arn = Some(value);
        self
    }

    pub fn containers(
        mut self,
        value: Vec<crate::datadogV2::model::RemediationProblemContainer>,
    ) -> Self {
        self.containers = Some(value);
        self
    }

    pub fn cpu_units(mut self, value: String) -> Self {
        self.cpu_units = Some(value);
        self
    }

    pub fn desired_status(mut self, value: String) -> Self {
        self.desired_status = Some(value);
        self
    }

    pub fn execution_role_arn(mut self, value: String) -> Self {
        self.execution_role_arn = Some(value);
        self
    }

    pub fn health_status(mut self, value: String) -> Self {
        self.health_status = Some(value);
        self
    }

    pub fn issue_start_time(mut self, value: String) -> Self {
        self.issue_start_time = Some(value);
        self
    }

    pub fn issue_type(mut self, value: String) -> Self {
        self.issue_type = Some(value);
        self
    }

    pub fn last_status(mut self, value: String) -> Self {
        self.last_status = Some(value);
        self
    }

    pub fn launch_type(mut self, value: crate::datadogV2::model::RemediationLaunchType) -> Self {
        self.launch_type = Some(value);
        self
    }

    pub fn memory_mib(mut self, value: String) -> Self {
        self.memory_mib = Some(value);
        self
    }

    pub fn stop_code(mut self, value: String) -> Self {
        self.stop_code = Some(value);
        self
    }

    pub fn stopped_reason(mut self, value: String) -> Self {
        self.stopped_reason = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn task_arn(mut self, value: String) -> Self {
        self.task_arn = Some(value);
        self
    }

    pub fn task_definition_arn(mut self, value: String) -> Self {
        self.task_definition_arn = Some(value);
        self
    }

    pub fn task_role_arn(mut self, value: String) -> Self {
        self.task_role_arn = Some(value);
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

impl Default for RemediationProblemTask {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RemediationProblemTask {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RemediationProblemTaskVisitor;
        impl<'a> Visitor<'a> for RemediationProblemTaskVisitor {
            type Value = RemediationProblemTask;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut availability_zone: Option<String> = None;
                let mut container_instance_arn: Option<String> = None;
                let mut containers: Option<
                    Vec<crate::datadogV2::model::RemediationProblemContainer>,
                > = None;
                let mut cpu_units: Option<String> = None;
                let mut desired_status: Option<String> = None;
                let mut execution_role_arn: Option<String> = None;
                let mut health_status: Option<String> = None;
                let mut issue_start_time: Option<String> = None;
                let mut issue_type: Option<String> = None;
                let mut last_status: Option<String> = None;
                let mut launch_type: Option<crate::datadogV2::model::RemediationLaunchType> = None;
                let mut memory_mib: Option<String> = None;
                let mut stop_code: Option<String> = None;
                let mut stopped_reason: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut task_arn: Option<String> = None;
                let mut task_definition_arn: Option<String> = None;
                let mut task_role_arn: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "availability_zone" => {
                            if v.is_null() {
                                continue;
                            }
                            availability_zone =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "container_instance_arn" => {
                            if v.is_null() {
                                continue;
                            }
                            container_instance_arn =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "containers" => {
                            if v.is_null() {
                                continue;
                            }
                            containers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cpu_units" => {
                            if v.is_null() {
                                continue;
                            }
                            cpu_units = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "desired_status" => {
                            if v.is_null() {
                                continue;
                            }
                            desired_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "execution_role_arn" => {
                            if v.is_null() {
                                continue;
                            }
                            execution_role_arn =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "health_status" => {
                            if v.is_null() {
                                continue;
                            }
                            health_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issue_start_time" => {
                            if v.is_null() {
                                continue;
                            }
                            issue_start_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issue_type" => {
                            if v.is_null() {
                                continue;
                            }
                            issue_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_status" => {
                            if v.is_null() {
                                continue;
                            }
                            last_status =
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
                        "memory_mib" => {
                            if v.is_null() {
                                continue;
                            }
                            memory_mib = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "stop_code" => {
                            if v.is_null() {
                                continue;
                            }
                            stop_code = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "stopped_reason" => {
                            if v.is_null() {
                                continue;
                            }
                            stopped_reason =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "task_arn" => {
                            if v.is_null() {
                                continue;
                            }
                            task_arn = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "task_definition_arn" => {
                            if v.is_null() {
                                continue;
                            }
                            task_definition_arn =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "task_role_arn" => {
                            if v.is_null() {
                                continue;
                            }
                            task_role_arn =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RemediationProblemTask {
                    availability_zone,
                    container_instance_arn,
                    containers,
                    cpu_units,
                    desired_status,
                    execution_role_arn,
                    health_status,
                    issue_start_time,
                    issue_type,
                    last_status,
                    launch_type,
                    memory_mib,
                    stop_code,
                    stopped_reason,
                    tags,
                    task_arn,
                    task_definition_arn,
                    task_role_arn,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RemediationProblemTaskVisitor)
    }
}
