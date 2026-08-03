// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a deployment detail response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetDeploymentV2DetailAttributes {
    /// Handle of the user who triggered the deployment.
    #[serde(rename = "author")]
    pub author: Option<String>,
    /// Number of hosts on which the deployment was canceled.
    #[serde(rename = "canceled_hosts")]
    pub canceled_hosts: Option<i64>,
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
    /// Number of hosts on which the deployment failed.
    #[serde(rename = "failed_hosts")]
    pub failed_hosts: Option<i64>,
    /// Current high-level status of the deployment (for example, "pending", "running",
    /// "completed", "failed").
    #[serde(rename = "high_level_status")]
    pub high_level_status: Option<String>,
    /// Per-host status list for this deployment.
    #[serde(rename = "hosts")]
    pub hosts: Option<Vec<crate::datadogV2::model::FleetDeploymentV2DetailAgent>>,
    /// Whether this deployment was triggered by a schedule (`schedule_id` is non-empty).
    #[serde(rename = "is_scheduled")]
    pub is_scheduled: Option<bool>,
    /// Query used to filter and select target hosts for the deployment.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Number of hosts on which the deployment is currently running.
    #[serde(rename = "running_hosts")]
    pub running_hosts: Option<i64>,
    /// Identifier of the schedule that triggered this deployment. Empty if triggered manually.
    #[serde(rename = "schedule_id")]
    pub schedule_id: Option<String>,
    /// Number of hosts that were skipped during the deployment.
    #[serde(rename = "skipped_hosts")]
    pub skipped_hosts: Option<i64>,
    /// Number of hosts on which the deployment succeeded.
    #[serde(rename = "succeeded_hosts")]
    pub succeeded_hosts: Option<i64>,
    /// Distinct package versions targeted by this deployment, in first-seen order.
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

impl FleetDeploymentV2DetailAttributes {
    pub fn new() -> FleetDeploymentV2DetailAttributes {
        FleetDeploymentV2DetailAttributes {
            author: None,
            canceled_hosts: None,
            config_operations: None,
            duration_seconds: None,
            error_summary: None,
            estimated_finished_at: None,
            failed_hosts: None,
            high_level_status: None,
            hosts: None,
            is_scheduled: None,
            query: None,
            running_hosts: None,
            schedule_id: None,
            skipped_hosts: None,
            succeeded_hosts: None,
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

    pub fn canceled_hosts(mut self, value: i64) -> Self {
        self.canceled_hosts = Some(value);
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

    pub fn failed_hosts(mut self, value: i64) -> Self {
        self.failed_hosts = Some(value);
        self
    }

    pub fn high_level_status(mut self, value: String) -> Self {
        self.high_level_status = Some(value);
        self
    }

    pub fn hosts(
        mut self,
        value: Vec<crate::datadogV2::model::FleetDeploymentV2DetailAgent>,
    ) -> Self {
        self.hosts = Some(value);
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

    pub fn running_hosts(mut self, value: i64) -> Self {
        self.running_hosts = Some(value);
        self
    }

    pub fn schedule_id(mut self, value: String) -> Self {
        self.schedule_id = Some(value);
        self
    }

    pub fn skipped_hosts(mut self, value: i64) -> Self {
        self.skipped_hosts = Some(value);
        self
    }

    pub fn succeeded_hosts(mut self, value: i64) -> Self {
        self.succeeded_hosts = Some(value);
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

impl Default for FleetDeploymentV2DetailAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetDeploymentV2DetailAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetDeploymentV2DetailAttributesVisitor;
        impl<'a> Visitor<'a> for FleetDeploymentV2DetailAttributesVisitor {
            type Value = FleetDeploymentV2DetailAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author: Option<String> = None;
                let mut canceled_hosts: Option<i64> = None;
                let mut config_operations: Option<
                    Vec<crate::datadogV2::model::FleetDeploymentOperation>,
                > = None;
                let mut duration_seconds: Option<i64> = None;
                let mut error_summary: Option<String> = None;
                let mut estimated_finished_at: Option<i64> = None;
                let mut failed_hosts: Option<i64> = None;
                let mut high_level_status: Option<String> = None;
                let mut hosts: Option<Vec<crate::datadogV2::model::FleetDeploymentV2DetailAgent>> =
                    None;
                let mut is_scheduled: Option<bool> = None;
                let mut query: Option<String> = None;
                let mut running_hosts: Option<i64> = None;
                let mut schedule_id: Option<String> = None;
                let mut skipped_hosts: Option<i64> = None;
                let mut succeeded_hosts: Option<i64> = None;
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
                        "canceled_hosts" => {
                            if v.is_null() {
                                continue;
                            }
                            canceled_hosts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "failed_hosts" => {
                            if v.is_null() {
                                continue;
                            }
                            failed_hosts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "high_level_status" => {
                            if v.is_null() {
                                continue;
                            }
                            high_level_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hosts" => {
                            if v.is_null() {
                                continue;
                            }
                            hosts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "running_hosts" => {
                            if v.is_null() {
                                continue;
                            }
                            running_hosts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "schedule_id" => {
                            if v.is_null() {
                                continue;
                            }
                            schedule_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "skipped_hosts" => {
                            if v.is_null() {
                                continue;
                            }
                            skipped_hosts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "succeeded_hosts" => {
                            if v.is_null() {
                                continue;
                            }
                            succeeded_hosts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = FleetDeploymentV2DetailAttributes {
                    author,
                    canceled_hosts,
                    config_operations,
                    duration_seconds,
                    error_summary,
                    estimated_finished_at,
                    failed_hosts,
                    high_level_status,
                    hosts,
                    is_scheduled,
                    query,
                    running_hosts,
                    schedule_id,
                    skipped_hosts,
                    succeeded_hosts,
                    target_versions,
                    total_hosts,
                    update_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetDeploymentV2DetailAttributesVisitor)
    }
}
