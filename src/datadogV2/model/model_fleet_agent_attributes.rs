// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a Datadog Agent in the list view.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetAgentAttributes {
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
    /// The unique agent key identifier.
    #[serde(rename = "datadog_agent_key")]
    pub datadog_agent_key: Option<String>,
    /// Datadog products enabled on the agent.
    #[serde(rename = "enabled_products")]
    pub enabled_products: Option<Vec<String>>,
    /// Environments the agent is reporting from.
    #[serde(rename = "envs")]
    pub envs: Option<Vec<String>>,
    /// Timestamp when the agent was first seen.
    #[serde(rename = "first_seen_at")]
    pub first_seen_at: Option<i64>,
    /// The hostname of the agent.
    #[serde(rename = "hostname")]
    pub hostname: Option<String>,
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
    /// OpenTelemetry collector version (if applicable).
    #[serde(rename = "otel_collector_version")]
    pub otel_collector_version: Option<String>,
    /// List of OpenTelemetry collector versions (if applicable).
    #[serde(rename = "otel_collector_versions")]
    pub otel_collector_versions: Option<Vec<String>>,
    /// Kubernetes pod name (if applicable).
    #[serde(rename = "pod_name")]
    pub pod_name: Option<String>,
    /// Remote agent management status.
    #[serde(rename = "remote_agent_management")]
    pub remote_agent_management: Option<String>,
    /// Remote configuration status.
    #[serde(rename = "remote_config_status")]
    pub remote_config_status: Option<String>,
    /// Services running on the agent.
    #[serde(rename = "services")]
    pub services: Option<Vec<String>>,
    /// Tags associated with the agent.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<crate::datadogV2::model::FleetAgentAttributesTagsItems>>,
    /// Team associated with the agent.
    #[serde(rename = "team")]
    pub team: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetAgentAttributes {
    pub fn new() -> FleetAgentAttributes {
        FleetAgentAttributes {
            agent_version: None,
            api_key_name: None,
            api_key_uuid: None,
            cloud_provider: None,
            cluster_name: None,
            datadog_agent_key: None,
            enabled_products: None,
            envs: None,
            first_seen_at: None,
            hostname: None,
            ip_addresses: None,
            is_single_step_instrumentation_enabled: None,
            last_restart_at: None,
            os: None,
            otel_collector_version: None,
            otel_collector_versions: None,
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

    pub fn datadog_agent_key(mut self, value: String) -> Self {
        self.datadog_agent_key = Some(value);
        self
    }

    pub fn enabled_products(mut self, value: Vec<String>) -> Self {
        self.enabled_products = Some(value);
        self
    }

    pub fn envs(mut self, value: Vec<String>) -> Self {
        self.envs = Some(value);
        self
    }

    pub fn first_seen_at(mut self, value: i64) -> Self {
        self.first_seen_at = Some(value);
        self
    }

    pub fn hostname(mut self, value: String) -> Self {
        self.hostname = Some(value);
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

    pub fn otel_collector_version(mut self, value: String) -> Self {
        self.otel_collector_version = Some(value);
        self
    }

    pub fn otel_collector_versions(mut self, value: Vec<String>) -> Self {
        self.otel_collector_versions = Some(value);
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

impl Default for FleetAgentAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetAgentAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetAgentAttributesVisitor;
        impl<'a> Visitor<'a> for FleetAgentAttributesVisitor {
            type Value = FleetAgentAttributes;

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
                let mut datadog_agent_key: Option<String> = None;
                let mut enabled_products: Option<Vec<String>> = None;
                let mut envs: Option<Vec<String>> = None;
                let mut first_seen_at: Option<i64> = None;
                let mut hostname: Option<String> = None;
                let mut ip_addresses: Option<Vec<String>> = None;
                let mut is_single_step_instrumentation_enabled: Option<bool> = None;
                let mut last_restart_at: Option<i64> = None;
                let mut os: Option<String> = None;
                let mut otel_collector_version: Option<String> = None;
                let mut otel_collector_versions: Option<Vec<String>> = None;
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
                        "datadog_agent_key" => {
                            if v.is_null() {
                                continue;
                            }
                            datadog_agent_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled_products" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled_products =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "envs" => {
                            if v.is_null() {
                                continue;
                            }
                            envs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "first_seen_at" => {
                            if v.is_null() {
                                continue;
                            }
                            first_seen_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hostname" => {
                            if v.is_null() {
                                continue;
                            }
                            hostname = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = FleetAgentAttributes {
                    agent_version,
                    api_key_name,
                    api_key_uuid,
                    cloud_provider,
                    cluster_name,
                    datadog_agent_key,
                    enabled_products,
                    envs,
                    first_seen_at,
                    hostname,
                    ip_addresses,
                    is_single_step_instrumentation_enabled,
                    last_restart_at,
                    os,
                    otel_collector_version,
                    otel_collector_versions,
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

        deserializer.deserialize_any(FleetAgentAttributesVisitor)
    }
}
