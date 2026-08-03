// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a deployment in the v2 API response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetDeploymentV2Attributes {
    /// Handle of the user who triggered the deployment.
    #[serde(rename = "author")]
    pub author: Option<String>,
    /// Ordered list of configuration file operations applied by this deployment.
    /// Absent for package deployments, which have no configuration file operations.
    #[serde(rename = "config_operations")]
    pub config_operations: Option<Vec<crate::datadogV2::model::FleetDeploymentOperation>>,
    /// Duration of the deployment in seconds, computed as `finished_at - started_at`.
    /// Zero if the deployment has not finished.
    #[serde(rename = "duration_seconds")]
    pub duration_seconds: Option<i64>,
    /// Top-level error message for the deployment. Populated only when the deployment has failed.
    #[serde(rename = "error_summary")]
    pub error_summary: Option<String>,
    /// Estimated completion time of the deployment as a Unix timestamp. Zero if not available.
    #[serde(rename = "estimated_finished_at")]
    pub estimated_finished_at: Option<i64>,
    /// Time the deployment finished as a Unix timestamp. Zero if not yet finished.
    #[serde(rename = "finished_at")]
    pub finished_at: Option<i64>,
    /// Whether this deployment was triggered by a schedule (`schedule_id` is non-empty).
    #[serde(rename = "is_scheduled")]
    pub is_scheduled: Option<bool>,
    /// Query used to filter and select target hosts for the deployment.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Identifier of the schedule that triggered this deployment. Empty if triggered manually.
    #[serde(rename = "schedule_id")]
    pub schedule_id: Option<String>,
    /// Time the deployment started as a Unix timestamp. Zero if not yet started.
    #[serde(rename = "started_at")]
    pub started_at: Option<i64>,
    /// Current high-level status of the deployment (for example, "pending", "running",
    /// "completed", "failed").
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// Package versions targeted by this deployment.
    #[serde(rename = "target_versions")]
    pub target_versions: Option<Vec<String>>,
    /// Total number of hosts targeted by this deployment.
    #[serde(rename = "total_hosts")]
    pub total_hosts: Option<i64>,
    /// Type of update operation performed by this deployment
    /// (for example, "update_config_operations", "update_package").
    #[serde(rename = "update_type")]
    pub update_type: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetDeploymentV2Attributes {
    pub fn new() -> FleetDeploymentV2Attributes {
        FleetDeploymentV2Attributes {
            author: None,
            config_operations: None,
            duration_seconds: None,
            error_summary: None,
            estimated_finished_at: None,
            finished_at: None,
            is_scheduled: None,
            query: None,
            schedule_id: None,
            started_at: None,
            status: None,
            target_versions: None,
            total_hosts: None,
            update_type: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn author(mut self, value: String) -> Self {
        self.author = Some(value);
        self
    }

    pub fn config_operations(
        mut self,
        value: Vec<crate::datadogV2::model::FleetDeploymentOperation>,
    ) -> Self {
        self.config_operations = Some(value);
        self
    }

    pub fn duration_seconds(mut self, value: i64) -> Self {
        self.duration_seconds = Some(value);
        self
    }

    pub fn error_summary(mut self, value: String) -> Self {
        self.error_summary = Some(value);
        self
    }

    pub fn estimated_finished_at(mut self, value: i64) -> Self {
        self.estimated_finished_at = Some(value);
        self
    }

    pub fn finished_at(mut self, value: i64) -> Self {
        self.finished_at = Some(value);
        self
    }

    pub fn is_scheduled(mut self, value: bool) -> Self {
        self.is_scheduled = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn schedule_id(mut self, value: String) -> Self {
        self.schedule_id = Some(value);
        self
    }

    pub fn started_at(mut self, value: i64) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
        self
    }

    pub fn target_versions(mut self, value: Vec<String>) -> Self {
        self.target_versions = Some(value);
        self
    }

    pub fn total_hosts(mut self, value: i64) -> Self {
        self.total_hosts = Some(value);
        self
    }

    pub fn update_type(mut self, value: String) -> Self {
        self.update_type = Some(value);
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

impl Default for FleetDeploymentV2Attributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetDeploymentV2Attributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetDeploymentV2AttributesVisitor;
        impl<'a> Visitor<'a> for FleetDeploymentV2AttributesVisitor {
            type Value = FleetDeploymentV2Attributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author: Option<String> = None;
                let mut config_operations: Option<
                    Vec<crate::datadogV2::model::FleetDeploymentOperation>,
                > = None;
                let mut duration_seconds: Option<i64> = None;
                let mut error_summary: Option<String> = None;
                let mut estimated_finished_at: Option<i64> = None;
                let mut finished_at: Option<i64> = None;
                let mut is_scheduled: Option<bool> = None;
                let mut query: Option<String> = None;
                let mut schedule_id: Option<String> = None;
                let mut started_at: Option<i64> = None;
                let mut status: Option<String> = None;
                let mut target_versions: Option<Vec<String>> = None;
                let mut total_hosts: Option<i64> = None;
                let mut update_type: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "author" => {
                            if v.is_null() {
                                continue;
                            }
                            author = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "config_operations" => {
                            if v.is_null() {
                                continue;
                            }
                            config_operations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "duration_seconds" => {
                            if v.is_null() {
                                continue;
                            }
                            duration_seconds =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_summary" => {
                            if v.is_null() {
                                continue;
                            }
                            error_summary =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "estimated_finished_at" => {
                            if v.is_null() {
                                continue;
                            }
                            estimated_finished_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "finished_at" => {
                            if v.is_null() {
                                continue;
                            }
                            finished_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_scheduled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_scheduled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "schedule_id" => {
                            if v.is_null() {
                                continue;
                            }
                            schedule_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "started_at" => {
                            if v.is_null() {
                                continue;
                            }
                            started_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target_versions" => {
                            if v.is_null() {
                                continue;
                            }
                            target_versions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_hosts" => {
                            if v.is_null() {
                                continue;
                            }
                            total_hosts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "update_type" => {
                            if v.is_null() {
                                continue;
                            }
                            update_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FleetDeploymentV2Attributes {
                    author,
                    config_operations,
                    duration_seconds,
                    error_summary,
                    estimated_finished_at,
                    finished_at,
                    is_scheduled,
                    query,
                    schedule_id,
                    started_at,
                    status,
                    target_versions,
                    total_hosts,
                    update_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetDeploymentV2AttributesVisitor)
    }
}
