// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a Datadog Agent in the v2 list response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetAgentV2Attributes {
    /// The Datadog Agent version.
    #[serde(rename = "agent_version")]
    pub agent_version: Option<String>,
    /// The name of the API key used by the agent, if available and not redacted.
    #[serde(rename = "api_key_name")]
    pub api_key_name: Option<String>,
    /// The UUID of the API key used by the agent.
    #[serde(rename = "api_key_uuid")]
    pub api_key_uuid: Option<String>,
    /// The cloud provider where the agent is running.
    #[serde(rename = "cloud_provider")]
    pub cloud_provider: Option<String>,
    /// The Kubernetes cluster name, if the agent runs in a cluster.
    #[serde(rename = "cluster_name")]
    pub cluster_name: Option<String>,
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
    /// Unix timestamp when the agent was first seen.
    #[serde(rename = "first_seen_at")]
    pub first_seen_at: Option<i64>,
    /// Identifiers of fleet policies applied to the agent.
    #[serde(rename = "fleet_policies")]
    pub fleet_policies: Option<Vec<String>>,
    /// The hostname of the agent.
    #[serde(rename = "hostname")]
    pub hostname: Option<String>,
    /// Number of instrumentation errors on the agent. Absent from the response when the count is zero.
    #[serde(rename = "instrumentation_error_counts")]
    pub instrumentation_error_counts: Option<i64>,
    /// The single-step instrumentation status of the Agent.
    #[serde(rename = "instrumentation_status")]
    pub instrumentation_status:
        Option<crate::datadogV2::model::FleetAgentV2AttributesInstrumentationStatus>,
    /// Names of integrations configured on the agent.
    #[serde(rename = "integrations")]
    pub integrations: Option<Vec<String>>,
    /// IP addresses of the agent host.
    #[serde(rename = "ip_addresses")]
    pub ip_addresses: Option<Vec<String>>,
    /// Whether single-step instrumentation is enabled on the agent.
    #[serde(rename = "is_single_step_instrumentation_enabled")]
    pub is_single_step_instrumentation_enabled: Option<bool>,
    /// Unix timestamp of the last agent restart.
    #[serde(rename = "last_restart_at")]
    pub last_restart_at: Option<i64>,
    /// The operating system of the host.
    #[serde(rename = "os")]
    pub os: Option<String>,
    /// OpenTelemetry collector deployment types associated with the agent.
    #[serde(rename = "otel_collector_deployment_types")]
    pub otel_collector_deployment_types: Option<Vec<String>>,
    /// OpenTelemetry collector distributions associated with the agent.
    #[serde(rename = "otel_collector_distributions")]
    pub otel_collector_distributions: Option<Vec<String>>,
    /// The primary OpenTelemetry collector version, if applicable.
    #[serde(rename = "otel_collector_version")]
    pub otel_collector_version: Option<String>,
    /// All OpenTelemetry collector versions associated with the agent.
    #[serde(rename = "otel_collector_versions")]
    pub otel_collector_versions: Option<Vec<String>>,
    /// OpenTelemetry resource attributes reported by the agent.
    #[serde(rename = "otel_resource_attributes")]
    pub otel_resource_attributes: Option<Vec<String>>,
    /// The Kubernetes pod name, if the agent runs as a pod.
    #[serde(rename = "pod_name")]
    pub pod_name: Option<String>,
    /// The remote agent management status.
    #[serde(rename = "remote_agent_management")]
    pub remote_agent_management: Option<String>,
    /// The remote configuration connection status of the agent.
    #[serde(rename = "remote_config_status")]
    pub remote_config_status: Option<String>,
    /// Services running on the agent.
    #[serde(rename = "services")]
    pub services: Option<Vec<String>>,
    /// Tags associated with the agent. Returned as an empty array when the agent has no tags.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<crate::datadogV2::model::FleetAgentAttributesTagsItems>>,
    /// The team associated with the agent.
    #[serde(rename = "team")]
    pub team: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetAgentV2Attributes {
    pub fn new() -> FleetAgentV2Attributes {
        FleetAgentV2Attributes {
            agent_version: None,
            api_key_name: None,
            api_key_uuid: None,
            cloud_provider: None,
            cluster_name: None,
            datadog_data_center: None,
            ecs_fargate_cluster_name: None,
            ecs_fargate_task_arn: None,
            enabled_products: None,
            env: None,
            first_seen_at: None,
            fleet_policies: None,
            hostname: None,
            instrumentation_error_counts: None,
            instrumentation_status: None,
            integrations: None,
            ip_addresses: None,
            is_single_step_instrumentation_enabled: None,
            last_restart_at: None,
            os: None,
            otel_collector_deployment_types: None,
            otel_collector_distributions: None,
            otel_collector_version: None,
            otel_collector_versions: None,
            otel_resource_attributes: None,
            pod_name: None,
            remote_agent_management: None,
            remote_config_status: None,
            services: None,
            tags: None,
            team: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
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

    pub fn fleet_policies(mut self, value: Vec<String>) -> Self {
        self.fleet_policies = Some(value);
        self
    }

    pub fn hostname(mut self, value: String) -> Self {
        self.hostname = Some(value);
        self
    }

    pub fn instrumentation_error_counts(mut self, value: i64) -> Self {
        self.instrumentation_error_counts = Some(value);
        self
    }

    pub fn instrumentation_status(
        mut self,
        value: crate::datadogV2::model::FleetAgentV2AttributesInstrumentationStatus,
    ) -> Self {
        self.instrumentation_status = Some(value);
        self
    }

    pub fn integrations(mut self, value: Vec<String>) -> Self {
        self.integrations = Some(value);
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

    pub fn otel_collector_deployment_types(mut self, value: Vec<String>) -> Self {
        self.otel_collector_deployment_types = Some(value);
        self
    }

    pub fn otel_collector_distributions(mut self, value: Vec<String>) -> Self {
        self.otel_collector_distributions = Some(value);
        self
    }

    pub fn otel_collector_version(mut self, value: String) -> Self {
        self.otel_collector_version = Some(value);
        self
    }

    pub fn otel_collector_versions(mut self, value: Vec<String>) -> Self {
        self.otel_collector_versions = Some(value);
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

    pub fn tags(
        mut self,
        value: Vec<crate::datadogV2::model::FleetAgentAttributesTagsItems>,
    ) -> Self {
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

impl Default for FleetAgentV2Attributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetAgentV2Attributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetAgentV2AttributesVisitor;
        impl<'a> Visitor<'a> for FleetAgentV2AttributesVisitor {
            type Value = FleetAgentV2Attributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut agent_version: Option<String> = None;
                let mut api_key_name: Option<String> = None;
                let mut api_key_uuid: Option<String> = None;
                let mut cloud_provider: Option<String> = None;
                let mut cluster_name: Option<String> = None;
                let mut datadog_data_center: Option<String> = None;
                let mut ecs_fargate_cluster_name: Option<String> = None;
                let mut ecs_fargate_task_arn: Option<String> = None;
                let mut enabled_products: Option<Vec<String>> = None;
                let mut env: Option<Vec<String>> = None;
                let mut first_seen_at: Option<i64> = None;
                let mut fleet_policies: Option<Vec<String>> = None;
                let mut hostname: Option<String> = None;
                let mut instrumentation_error_counts: Option<i64> = None;
                let mut instrumentation_status: Option<
                    crate::datadogV2::model::FleetAgentV2AttributesInstrumentationStatus,
                > = None;
                let mut integrations: Option<Vec<String>> = None;
                let mut ip_addresses: Option<Vec<String>> = None;
                let mut is_single_step_instrumentation_enabled: Option<bool> = None;
                let mut last_restart_at: Option<i64> = None;
                let mut os: Option<String> = None;
                let mut otel_collector_deployment_types: Option<Vec<String>> = None;
                let mut otel_collector_distributions: Option<Vec<String>> = None;
                let mut otel_collector_version: Option<String> = None;
                let mut otel_collector_versions: Option<Vec<String>> = None;
                let mut otel_resource_attributes: Option<Vec<String>> = None;
                let mut pod_name: Option<String> = None;
                let mut remote_agent_management: Option<String> = None;
                let mut remote_config_status: Option<String> = None;
                let mut services: Option<Vec<String>> = None;
                let mut tags: Option<Vec<crate::datadogV2::model::FleetAgentAttributesTagsItems>> =
                    None;
                let mut team: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                        "fleet_policies" => {
                            if v.is_null() {
                                continue;
                            }
                            fleet_policies =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hostname" => {
                            if v.is_null() {
                                continue;
                            }
                            hostname = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "instrumentation_error_counts" => {
                            if v.is_null() {
                                continue;
                            }
                            instrumentation_error_counts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "instrumentation_status" => {
                            if v.is_null() {
                                continue;
                            }
                            instrumentation_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _instrumentation_status) = instrumentation_status {
                                match _instrumentation_status {
                                    crate::datadogV2::model::FleetAgentV2AttributesInstrumentationStatus::UnparsedObject(_instrumentation_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "integrations" => {
                            if v.is_null() {
                                continue;
                            }
                            integrations =
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
                        "otel_collector_version" => {
                            if v.is_null() {
                                continue;
                            }
                            otel_collector_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "otel_collector_versions" => {
                            if v.is_null() {
                                continue;
                            }
                            otel_collector_versions =
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

                let content = FleetAgentV2Attributes {
                    agent_version,
                    api_key_name,
                    api_key_uuid,
                    cloud_provider,
                    cluster_name,
                    datadog_data_center,
                    ecs_fargate_cluster_name,
                    ecs_fargate_task_arn,
                    enabled_products,
                    env,
                    first_seen_at,
                    fleet_policies,
                    hostname,
                    instrumentation_error_counts,
                    instrumentation_status,
                    integrations,
                    ip_addresses,
                    is_single_step_instrumentation_enabled,
                    last_restart_at,
                    os,
                    otel_collector_deployment_types,
                    otel_collector_distributions,
                    otel_collector_version,
                    otel_collector_versions,
                    otel_resource_attributes,
                    pod_name,
                    remote_agent_management,
                    remote_config_status,
                    services,
                    tags,
                    team,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetAgentV2AttributesVisitor)
    }
}
