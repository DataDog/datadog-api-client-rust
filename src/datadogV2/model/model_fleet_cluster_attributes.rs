// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a Kubernetes cluster in the fleet.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetClusterAttributes {
    /// Datadog Agent versions running in the cluster.
    #[serde(rename = "agent_versions")]
    pub agent_versions: Option<Vec<String>>,
    /// API key names used by agents in the cluster.
    #[serde(rename = "api_key_names")]
    pub api_key_names: Option<Vec<String>>,
    /// API key UUIDs used by agents in the cluster.
    #[serde(rename = "api_key_uuids")]
    pub api_key_uuids: Option<Vec<String>>,
    /// Cloud providers hosting the cluster.
    #[serde(rename = "cloud_providers")]
    pub cloud_providers: Option<Vec<String>>,
    /// The name of the Kubernetes cluster.
    #[serde(rename = "cluster_name")]
    pub cluster_name: Option<String>,
    /// Datadog products enabled in the cluster.
    #[serde(rename = "enabled_products")]
    pub enabled_products: Option<Vec<String>>,
    /// Environments associated with the cluster.
    #[serde(rename = "envs")]
    pub envs: Option<Vec<String>>,
    /// Timestamp when the cluster was first seen.
    #[serde(rename = "first_seen_at")]
    pub first_seen_at: Option<i64>,
    /// The tool used to install agents in the cluster.
    #[serde(rename = "install_method_tool")]
    pub install_method_tool: Option<String>,
    /// Total number of nodes in the cluster.
    #[serde(rename = "node_count")]
    pub node_count: Option<i64>,
    /// Node counts grouped by status.
    #[serde(rename = "node_count_by_status")]
    pub node_count_by_status: Option<std::collections::BTreeMap<String, i64>>,
    /// Operating systems of nodes in the cluster.
    #[serde(rename = "operating_systems")]
    pub operating_systems: Option<Vec<String>>,
    /// OpenTelemetry collector distributions in the cluster.
    #[serde(rename = "otel_collector_distributions")]
    pub otel_collector_distributions: Option<Vec<String>>,
    /// OpenTelemetry collector versions in the cluster.
    #[serde(rename = "otel_collector_versions")]
    pub otel_collector_versions: Option<Vec<String>>,
    /// Pod counts grouped by state.
    #[serde(rename = "pod_count_by_state")]
    pub pod_count_by_state: Option<std::collections::BTreeMap<String, i64>>,
    /// Services running in the cluster.
    #[serde(rename = "services")]
    pub services: Option<Vec<String>>,
    /// Teams associated with the cluster.
    #[serde(rename = "teams")]
    pub teams: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetClusterAttributes {
    pub fn new() -> FleetClusterAttributes {
        FleetClusterAttributes {
            agent_versions: None,
            api_key_names: None,
            api_key_uuids: None,
            cloud_providers: None,
            cluster_name: None,
            enabled_products: None,
            envs: None,
            first_seen_at: None,
            install_method_tool: None,
            node_count: None,
            node_count_by_status: None,
            operating_systems: None,
            otel_collector_distributions: None,
            otel_collector_versions: None,
            pod_count_by_state: None,
            services: None,
            teams: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn agent_versions(mut self, value: Vec<String>) -> Self {
        self.agent_versions = Some(value);
        self
    }

    pub fn api_key_names(mut self, value: Vec<String>) -> Self {
        self.api_key_names = Some(value);
        self
    }

    pub fn api_key_uuids(mut self, value: Vec<String>) -> Self {
        self.api_key_uuids = Some(value);
        self
    }

    pub fn cloud_providers(mut self, value: Vec<String>) -> Self {
        self.cloud_providers = Some(value);
        self
    }

    pub fn cluster_name(mut self, value: String) -> Self {
        self.cluster_name = Some(value);
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

    pub fn install_method_tool(mut self, value: String) -> Self {
        self.install_method_tool = Some(value);
        self
    }

    pub fn node_count(mut self, value: i64) -> Self {
        self.node_count = Some(value);
        self
    }

    pub fn node_count_by_status(mut self, value: std::collections::BTreeMap<String, i64>) -> Self {
        self.node_count_by_status = Some(value);
        self
    }

    pub fn operating_systems(mut self, value: Vec<String>) -> Self {
        self.operating_systems = Some(value);
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

    pub fn pod_count_by_state(mut self, value: std::collections::BTreeMap<String, i64>) -> Self {
        self.pod_count_by_state = Some(value);
        self
    }

    pub fn services(mut self, value: Vec<String>) -> Self {
        self.services = Some(value);
        self
    }

    pub fn teams(mut self, value: Vec<String>) -> Self {
        self.teams = Some(value);
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

impl Default for FleetClusterAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetClusterAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetClusterAttributesVisitor;
        impl<'a> Visitor<'a> for FleetClusterAttributesVisitor {
            type Value = FleetClusterAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut agent_versions: Option<Vec<String>> = None;
                let mut api_key_names: Option<Vec<String>> = None;
                let mut api_key_uuids: Option<Vec<String>> = None;
                let mut cloud_providers: Option<Vec<String>> = None;
                let mut cluster_name: Option<String> = None;
                let mut enabled_products: Option<Vec<String>> = None;
                let mut envs: Option<Vec<String>> = None;
                let mut first_seen_at: Option<i64> = None;
                let mut install_method_tool: Option<String> = None;
                let mut node_count: Option<i64> = None;
                let mut node_count_by_status: Option<std::collections::BTreeMap<String, i64>> =
                    None;
                let mut operating_systems: Option<Vec<String>> = None;
                let mut otel_collector_distributions: Option<Vec<String>> = None;
                let mut otel_collector_versions: Option<Vec<String>> = None;
                let mut pod_count_by_state: Option<std::collections::BTreeMap<String, i64>> = None;
                let mut services: Option<Vec<String>> = None;
                let mut teams: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "agent_versions" => {
                            if v.is_null() {
                                continue;
                            }
                            agent_versions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "api_key_names" => {
                            if v.is_null() {
                                continue;
                            }
                            api_key_names =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "api_key_uuids" => {
                            if v.is_null() {
                                continue;
                            }
                            api_key_uuids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cloud_providers" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_providers =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cluster_name" => {
                            if v.is_null() {
                                continue;
                            }
                            cluster_name =
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
                        "install_method_tool" => {
                            if v.is_null() {
                                continue;
                            }
                            install_method_tool =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "node_count" => {
                            if v.is_null() {
                                continue;
                            }
                            node_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "node_count_by_status" => {
                            if v.is_null() {
                                continue;
                            }
                            node_count_by_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "operating_systems" => {
                            if v.is_null() {
                                continue;
                            }
                            operating_systems =
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
                        "pod_count_by_state" => {
                            if v.is_null() {
                                continue;
                            }
                            pod_count_by_state =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "services" => {
                            if v.is_null() {
                                continue;
                            }
                            services = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teams" => {
                            if v.is_null() {
                                continue;
                            }
                            teams = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FleetClusterAttributes {
                    agent_versions,
                    api_key_names,
                    api_key_uuids,
                    cloud_providers,
                    cluster_name,
                    enabled_products,
                    envs,
                    first_seen_at,
                    install_method_tool,
                    node_count,
                    node_count_by_status,
                    operating_systems,
                    otel_collector_distributions,
                    otel_collector_versions,
                    pod_count_by_state,
                    services,
                    teams,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetClusterAttributesVisitor)
    }
}
