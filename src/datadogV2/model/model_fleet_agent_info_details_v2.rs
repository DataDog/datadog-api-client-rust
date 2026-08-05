// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Detailed information about a Datadog Agent.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetAgentInfoDetailsV2 {
    /// The currently active agent in the high-availability group.
    #[serde(rename = "active_ha_agent")]
    pub active_ha_agent: Option<String>,
    /// The Datadog Agent version.
    #[serde(rename = "agent_version")]
    pub agent_version: Option<String>,
    /// The API key name (if available and not redacted).
    #[serde(rename = "api_key_name")]
    pub api_key_name: Option<String>,
    /// The API key UUID.
    #[serde(rename = "api_key_uuid")]
    pub api_key_uuid: Option<String>,
    /// The cloud provider where the agent is running.
    #[serde(rename = "cloud_provider")]
    pub cloud_provider: Option<String>,
    /// Kubernetes cluster name (if applicable).
    #[serde(rename = "cluster_name")]
    pub cluster_name: Option<String>,
    /// The configuration identifier applied to the agent.
    #[serde(rename = "config_id")]
    pub config_id: Option<String>,
    /// The unique agent key identifier.
    #[serde(rename = "datadog_agent_key")]
    pub datadog_agent_key: Option<String>,
    /// The Datadog data center the agent reports to.
    #[serde(rename = "datadog_data_center")]
    pub datadog_data_center: Option<String>,
    /// The ECS Fargate cluster name, if the agent runs in an ECS Fargate environment.
    #[serde(rename = "ecs_fargate_cluster_name")]
    pub ecs_fargate_cluster_name: Option<String>,
    /// The ECS Fargate task ARN, if the agent runs in an ECS Fargate environment.
    #[serde(rename = "ecs_fargate_task_arn")]
    pub ecs_fargate_task_arn: Option<String>,
    /// Datadog products enabled on the agent.
    #[serde(rename = "enabled_products")]
    pub enabled_products: Option<Vec<String>>,
    /// Environments the agent is reporting from.
    #[serde(rename = "env")]
    pub env: Option<Vec<String>>,
    /// Timestamp when the agent was first seen.
    #[serde(rename = "first_seen_at")]
    pub first_seen_at: Option<i64>,
    /// Hosts participating in the agent's high-availability group.
    #[serde(rename = "ha_agent_hosts")]
    pub ha_agent_hosts: Option<Vec<String>>,
    /// The high-availability state of the agent.
    #[serde(rename = "ha_agent_state")]
    pub ha_agent_state: Option<String>,
    /// The hostname of the agent.
    #[serde(rename = "hostname")]
    pub hostname: Option<String>,
    /// Alternative hostname list for the agent.
    #[serde(rename = "hostname_aliases")]
    pub hostname_aliases: Option<Vec<String>>,
    /// The version of the installer used.
    #[serde(rename = "install_method_installer_version")]
    pub install_method_installer_version: Option<String>,
    /// The tool used to install the agent.
    #[serde(rename = "install_method_tool")]
    pub install_method_tool: Option<String>,
    /// IP addresses of the agent.
    #[serde(rename = "ip_addresses")]
    pub ip_addresses: Option<Vec<String>>,
    /// Whether single-step instrumentation is enabled.
    #[serde(rename = "is_single_step_instrumentation_enabled")]
    pub is_single_step_instrumentation_enabled: Option<bool>,
    /// Timestamp of the last agent restart.
    #[serde(rename = "last_restart_at")]
    pub last_restart_at: Option<i64>,
    /// The operating system.
    #[serde(rename = "os")]
    pub os: Option<String>,
    /// The operating system version.
    #[serde(rename = "os_version")]
    pub os_version: Option<String>,
    /// OpenTelemetry collector deployment types associated with the agent.
    #[serde(rename = "otel_collector_deployment_types")]
    pub otel_collector_deployment_types: Option<Vec<String>>,
    /// OpenTelemetry collector distributions associated with the agent.
    #[serde(rename = "otel_collector_distributions")]
    pub otel_collector_distributions: Option<Vec<String>>,
    /// List of OpenTelemetry collector versions (if applicable).
    #[serde(rename = "otel_collector_versions")]
    pub otel_collector_versions: Option<Vec<String>>,
    /// OpenTelemetry collectors associated with the agent (if applicable).
    #[serde(rename = "otel_collectors")]
    pub otel_collectors: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// OpenTelemetry resource attributes reported by the agent.
    #[serde(rename = "otel_resource_attributes")]
    pub otel_resource_attributes: Option<Vec<String>>,
    /// Kubernetes pod name (if applicable).
    #[serde(rename = "pod_name")]
    pub pod_name: Option<String>,
    /// The preferred active agent in the high-availability group.
    #[serde(rename = "preferred_ha_active_agent")]
    pub preferred_ha_active_agent: Option<String>,
    /// The Python version used by the agent.
    #[serde(rename = "python_version")]
    pub python_version: Option<String>,
    /// Regions where the agent is running.
    #[serde(rename = "region")]
    pub region: Option<Vec<String>>,
    /// Remote agent management status.
    #[serde(rename = "remote_agent_management")]
    pub remote_agent_management: Option<String>,
    /// Remote configuration status.
    #[serde(rename = "remote_config_status")]
    pub remote_config_status: Option<String>,
    /// Services running on the agent.
    #[serde(rename = "services")]
    pub services: Option<Vec<String>>,
    /// Whether the agent supports remote agent upgrade.
    #[serde(rename = "support_agent_upgrade")]
    pub support_agent_upgrade: Option<bool>,
    /// Tags associated with the agent.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Team associated with the agent.
    #[serde(rename = "team")]
    pub team: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetAgentInfoDetailsV2 {
    pub fn new() -> FleetAgentInfoDetailsV2 {
        FleetAgentInfoDetailsV2 {
            active_ha_agent: None,
            agent_version: None,
            api_key_name: None,
            api_key_uuid: None,
            cloud_provider: None,
            cluster_name: None,
            config_id: None,
            datadog_agent_key: None,
            datadog_data_center: None,
            ecs_fargate_cluster_name: None,
            ecs_fargate_task_arn: None,
            enabled_products: None,
            env: None,
            first_seen_at: None,
            ha_agent_hosts: None,
            ha_agent_state: None,
            hostname: None,
            hostname_aliases: None,
            install_method_installer_version: None,
            install_method_tool: None,
            ip_addresses: None,
            is_single_step_instrumentation_enabled: None,
            last_restart_at: None,
            os: None,
            os_version: None,
            otel_collector_deployment_types: None,
            otel_collector_distributions: None,
            otel_collector_versions: None,
            otel_collectors: None,
            otel_resource_attributes: None,
            pod_name: None,
            preferred_ha_active_agent: None,
            python_version: None,
            region: None,
            remote_agent_management: None,
            remote_config_status: None,
            services: None,
            support_agent_upgrade: None,
            tags: None,
            team: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn active_ha_agent(mut self, value: String) -> Self {
        self.active_ha_agent = Some(value);
        self
    }

    pub fn agent_version(mut self, value: String) -> Self {
        self.agent_version = Some(value);
        self
    }

    pub fn api_key_name(mut self, value: String) -> Self {
        self.api_key_name = Some(value);
        self
    }

    pub fn api_key_uuid(mut self, value: String) -> Self {
        self.api_key_uuid = Some(value);
        self
    }

    pub fn cloud_provider(mut self, value: String) -> Self {
        self.cloud_provider = Some(value);
        self
    }

    pub fn cluster_name(mut self, value: String) -> Self {
        self.cluster_name = Some(value);
        self
    }

    pub fn config_id(mut self, value: String) -> Self {
        self.config_id = Some(value);
        self
    }

    pub fn datadog_agent_key(mut self, value: String) -> Self {
        self.datadog_agent_key = Some(value);
        self
    }

    pub fn datadog_data_center(mut self, value: String) -> Self {
        self.datadog_data_center = Some(value);
        self
    }

    pub fn ecs_fargate_cluster_name(mut self, value: String) -> Self {
        self.ecs_fargate_cluster_name = Some(value);
        self
    }

    pub fn ecs_fargate_task_arn(mut self, value: String) -> Self {
        self.ecs_fargate_task_arn = Some(value);
        self
    }

    pub fn enabled_products(mut self, value: Vec<String>) -> Self {
        self.enabled_products = Some(value);
        self
    }

    pub fn env(mut self, value: Vec<String>) -> Self {
        self.env = Some(value);
        self
    }

    pub fn first_seen_at(mut self, value: i64) -> Self {
        self.first_seen_at = Some(value);
        self
    }

    pub fn ha_agent_hosts(mut self, value: Vec<String>) -> Self {
        self.ha_agent_hosts = Some(value);
        self
    }

    pub fn ha_agent_state(mut self, value: String) -> Self {
        self.ha_agent_state = Some(value);
        self
    }

    pub fn hostname(mut self, value: String) -> Self {
        self.hostname = Some(value);
        self
    }

    pub fn hostname_aliases(mut self, value: Vec<String>) -> Self {
        self.hostname_aliases = Some(value);
        self
    }

    pub fn install_method_installer_version(mut self, value: String) -> Self {
        self.install_method_installer_version = Some(value);
        self
    }

    pub fn install_method_tool(mut self, value: String) -> Self {
        self.install_method_tool = Some(value);
        self
    }

    pub fn ip_addresses(mut self, value: Vec<String>) -> Self {
        self.ip_addresses = Some(value);
        self
    }

    pub fn is_single_step_instrumentation_enabled(mut self, value: bool) -> Self {
        self.is_single_step_instrumentation_enabled = Some(value);
        self
    }

    pub fn last_restart_at(mut self, value: i64) -> Self {
        self.last_restart_at = Some(value);
        self
    }

    pub fn os(mut self, value: String) -> Self {
        self.os = Some(value);
        self
    }

    pub fn os_version(mut self, value: String) -> Self {
        self.os_version = Some(value);
        self
    }

    pub fn otel_collector_deployment_types(mut self, value: Vec<String>) -> Self {
        self.otel_collector_deployment_types = Some(value);
        self
    }

    pub fn otel_collector_distributions(mut self, value: Vec<String>) -> Self {
        self.otel_collector_distributions = Some(value);
        self
    }

    pub fn otel_collector_versions(mut self, value: Vec<String>) -> Self {
        self.otel_collector_versions = Some(value);
        self
    }

    pub fn otel_collectors(
        mut self,
        value: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.otel_collectors = Some(value);
        self
    }

    pub fn otel_resource_attributes(mut self, value: Vec<String>) -> Self {
        self.otel_resource_attributes = Some(value);
        self
    }

    pub fn pod_name(mut self, value: String) -> Self {
        self.pod_name = Some(value);
        self
    }

    pub fn preferred_ha_active_agent(mut self, value: String) -> Self {
        self.preferred_ha_active_agent = Some(value);
        self
    }

    pub fn python_version(mut self, value: String) -> Self {
        self.python_version = Some(value);
        self
    }

    pub fn region(mut self, value: Vec<String>) -> Self {
        self.region = Some(value);
        self
    }

    pub fn remote_agent_management(mut self, value: String) -> Self {
        self.remote_agent_management = Some(value);
        self
    }

    pub fn remote_config_status(mut self, value: String) -> Self {
        self.remote_config_status = Some(value);
        self
    }

    pub fn services(mut self, value: Vec<String>) -> Self {
        self.services = Some(value);
        self
    }

    pub fn support_agent_upgrade(mut self, value: bool) -> Self {
        self.support_agent_upgrade = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn team(mut self, value: String) -> Self {
        self.team = Some(value);
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

impl Default for FleetAgentInfoDetailsV2 {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetAgentInfoDetailsV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetAgentInfoDetailsV2Visitor;
        impl<'a> Visitor<'a> for FleetAgentInfoDetailsV2Visitor {
            type Value = FleetAgentInfoDetailsV2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut active_ha_agent: Option<String> = None;
                let mut agent_version: Option<String> = None;
                let mut api_key_name: Option<String> = None;
                let mut api_key_uuid: Option<String> = None;
                let mut cloud_provider: Option<String> = None;
                let mut cluster_name: Option<String> = None;
                let mut config_id: Option<String> = None;
                let mut datadog_agent_key: Option<String> = None;
                let mut datadog_data_center: Option<String> = None;
                let mut ecs_fargate_cluster_name: Option<String> = None;
                let mut ecs_fargate_task_arn: Option<String> = None;
                let mut enabled_products: Option<Vec<String>> = None;
                let mut env: Option<Vec<String>> = None;
                let mut first_seen_at: Option<i64> = None;
                let mut ha_agent_hosts: Option<Vec<String>> = None;
                let mut ha_agent_state: Option<String> = None;
                let mut hostname: Option<String> = None;
                let mut hostname_aliases: Option<Vec<String>> = None;
                let mut install_method_installer_version: Option<String> = None;
                let mut install_method_tool: Option<String> = None;
                let mut ip_addresses: Option<Vec<String>> = None;
                let mut is_single_step_instrumentation_enabled: Option<bool> = None;
                let mut last_restart_at: Option<i64> = None;
                let mut os: Option<String> = None;
                let mut os_version: Option<String> = None;
                let mut otel_collector_deployment_types: Option<Vec<String>> = None;
                let mut otel_collector_distributions: Option<Vec<String>> = None;
                let mut otel_collector_versions: Option<Vec<String>> = None;
                let mut otel_collectors: Option<
                    Vec<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut otel_resource_attributes: Option<Vec<String>> = None;
                let mut pod_name: Option<String> = None;
                let mut preferred_ha_active_agent: Option<String> = None;
                let mut python_version: Option<String> = None;
                let mut region: Option<Vec<String>> = None;
                let mut remote_agent_management: Option<String> = None;
                let mut remote_config_status: Option<String> = None;
                let mut services: Option<Vec<String>> = None;
                let mut support_agent_upgrade: Option<bool> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut team: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "active_ha_agent" => {
                            if v.is_null() {
                                continue;
                            }
                            active_ha_agent =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "agent_version" => {
                            if v.is_null() {
                                continue;
                            }
                            agent_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "api_key_name" => {
                            if v.is_null() {
                                continue;
                            }
                            api_key_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "api_key_uuid" => {
                            if v.is_null() {
                                continue;
                            }
                            api_key_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cloud_provider" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_provider =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cluster_name" => {
                            if v.is_null() {
                                continue;
                            }
                            cluster_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "config_id" => {
                            if v.is_null() {
                                continue;
                            }
                            config_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "datadog_agent_key" => {
                            if v.is_null() {
                                continue;
                            }
                            datadog_agent_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "datadog_data_center" => {
                            if v.is_null() {
                                continue;
                            }
                            datadog_data_center =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ecs_fargate_cluster_name" => {
                            if v.is_null() {
                                continue;
                            }
                            ecs_fargate_cluster_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ecs_fargate_task_arn" => {
                            if v.is_null() {
                                continue;
                            }
                            ecs_fargate_task_arn =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled_products" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled_products =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "env" => {
                            if v.is_null() {
                                continue;
                            }
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "first_seen_at" => {
                            if v.is_null() {
                                continue;
                            }
                            first_seen_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ha_agent_hosts" => {
                            if v.is_null() {
                                continue;
                            }
                            ha_agent_hosts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ha_agent_state" => {
                            if v.is_null() {
                                continue;
                            }
                            ha_agent_state =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hostname" => {
                            if v.is_null() {
                                continue;
                            }
                            hostname = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hostname_aliases" => {
                            if v.is_null() {
                                continue;
                            }
                            hostname_aliases =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "install_method_installer_version" => {
                            if v.is_null() {
                                continue;
                            }
                            install_method_installer_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "install_method_tool" => {
                            if v.is_null() {
                                continue;
                            }
                            install_method_tool =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ip_addresses" => {
                            if v.is_null() {
                                continue;
                            }
                            ip_addresses =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_single_step_instrumentation_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_single_step_instrumentation_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_restart_at" => {
                            if v.is_null() {
                                continue;
                            }
                            last_restart_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "os" => {
                            if v.is_null() {
                                continue;
                            }
                            os = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "os_version" => {
                            if v.is_null() {
                                continue;
                            }
                            os_version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "otel_collector_deployment_types" => {
                            if v.is_null() {
                                continue;
                            }
                            otel_collector_deployment_types =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "otel_collector_distributions" => {
                            if v.is_null() {
                                continue;
                            }
                            otel_collector_distributions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "otel_collector_versions" => {
                            if v.is_null() {
                                continue;
                            }
                            otel_collector_versions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "otel_collectors" => {
                            if v.is_null() {
                                continue;
                            }
                            otel_collectors =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "otel_resource_attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            otel_resource_attributes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pod_name" => {
                            if v.is_null() {
                                continue;
                            }
                            pod_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "preferred_ha_active_agent" => {
                            if v.is_null() {
                                continue;
                            }
                            preferred_ha_active_agent =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "python_version" => {
                            if v.is_null() {
                                continue;
                            }
                            python_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "region" => {
                            if v.is_null() {
                                continue;
                            }
                            region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "remote_agent_management" => {
                            if v.is_null() {
                                continue;
                            }
                            remote_agent_management =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "remote_config_status" => {
                            if v.is_null() {
                                continue;
                            }
                            remote_config_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "services" => {
                            if v.is_null() {
                                continue;
                            }
                            services = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "support_agent_upgrade" => {
                            if v.is_null() {
                                continue;
                            }
                            support_agent_upgrade =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team" => {
                            if v.is_null() {
                                continue;
                            }
                            team = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FleetAgentInfoDetailsV2 {
                    active_ha_agent,
                    agent_version,
                    api_key_name,
                    api_key_uuid,
                    cloud_provider,
                    cluster_name,
                    config_id,
                    datadog_agent_key,
                    datadog_data_center,
                    ecs_fargate_cluster_name,
                    ecs_fargate_task_arn,
                    enabled_products,
                    env,
                    first_seen_at,
                    ha_agent_hosts,
                    ha_agent_state,
                    hostname,
                    hostname_aliases,
                    install_method_installer_version,
                    install_method_tool,
                    ip_addresses,
                    is_single_step_instrumentation_enabled,
                    last_restart_at,
                    os,
                    os_version,
                    otel_collector_deployment_types,
                    otel_collector_distributions,
                    otel_collector_versions,
                    otel_collectors,
                    otel_resource_attributes,
                    pod_name,
                    preferred_ha_active_agent,
                    python_version,
                    region,
                    remote_agent_management,
                    remote_config_status,
                    services,
                    support_agent_upgrade,
                    tags,
                    team,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetAgentInfoDetailsV2Visitor)
    }
}
