// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a unified host, combining data from agent and agentless sources.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CsmUnifiedHostAttributes {
    /// The ID of the cloud account that the host belongs to. Present only when the host was discovered through agentless scanning.
    #[serde(
        rename = "account_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub account_id: Option<Option<String>>,
    /// Whether CSM Vulnerabilities is enabled for containers through the Datadog Agent. `true` if enabled; `false` if disabled.
    #[serde(
        rename = "agent_csm_vm_containers_enabled",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub agent_csm_vm_containers_enabled: Option<Option<bool>>,
    /// Whether CSM Vulnerabilities is enabled for hosts through the Datadog Agent. `true` if enabled; `false` if disabled.
    #[serde(
        rename = "agent_csm_vm_hosts_enabled",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub agent_csm_vm_hosts_enabled: Option<Option<bool>>,
    /// Whether CSM Threats is enabled for this host through the Datadog Agent. `true` if enabled; `false` if disabled.
    #[serde(
        rename = "agent_cws_enabled",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub agent_cws_enabled: Option<Option<bool>>,
    /// Whether CSM Misconfigurations is enabled for this host through the Datadog Agent. `true` if enabled; `false` if disabled.
    #[serde(
        rename = "agent_posture_management",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub agent_posture_management: Option<Option<bool>>,
    /// The version of the Datadog Agent running on this host.
    #[serde(
        rename = "agent_version",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub agent_version: Option<Option<String>>,
    /// Whether CSM Misconfigurations is enabled for this host via agentless scanning. `true` if enabled; `false` if disabled.
    #[serde(
        rename = "agentless_posture_management",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub agentless_posture_management: Option<Option<bool>>,
    /// Whether CSM Vulnerabilities is enabled for this host via agentless scanning. `true` if enabled; `false` if disabled.
    #[serde(
        rename = "agentless_vulnerability_scanning",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub agentless_vulnerability_scanning: Option<Option<bool>>,
    /// The cloud provider of a host resource.
    #[serde(rename = "cloud_provider")]
    pub cloud_provider: Option<crate::datadogV2::model::CsmCloudProvider>,
    /// The name of the Kubernetes cluster the host belongs to, if applicable.
    #[serde(
        rename = "cluster_name",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub cluster_name: Option<Option<String>>,
    /// The Datadog Agent key associated with this host. Present only for agent-sourced hosts.
    #[serde(
        rename = "datadog_agent_key",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub datadog_agent_key: Option<Option<String>>,
    /// The list of environment tags associated with this host.
    #[serde(rename = "env", default, with = "::serde_with::rust::double_option")]
    pub env: Option<Option<Vec<String>>>,
    /// The internal Datadog host identifier. Present only for agent-sourced hosts.
    #[serde(
        rename = "host_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub host_id: Option<Option<i64>>,
    /// The tool used to install the Datadog Agent on this host.
    #[serde(
        rename = "install_method_tool",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub install_method_tool: Option<Option<String>>,
    /// The operating system of the host. Present only for agent-sourced hosts.
    #[serde(rename = "os", default, with = "::serde_with::rust::double_option")]
    pub os: Option<Option<String>>,
    /// The type of cloud resource for an agentless host.
    #[serde(rename = "resource_type")]
    pub resource_type: Option<crate::datadogV2::model::CsmAgentlessHostResourceType>,
    /// The source of a unified host entry, indicating whether it was discovered via agent, agentless scanning, or both.
    #[serde(rename = "source")]
    pub source: crate::datadogV2::model::CsmUnifiedHostSource,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CsmUnifiedHostAttributes {
    pub fn new(source: crate::datadogV2::model::CsmUnifiedHostSource) -> CsmUnifiedHostAttributes {
        CsmUnifiedHostAttributes {
            account_id: None,
            agent_csm_vm_containers_enabled: None,
            agent_csm_vm_hosts_enabled: None,
            agent_cws_enabled: None,
            agent_posture_management: None,
            agent_version: None,
            agentless_posture_management: None,
            agentless_vulnerability_scanning: None,
            cloud_provider: None,
            cluster_name: None,
            datadog_agent_key: None,
            env: None,
            host_id: None,
            install_method_tool: None,
            os: None,
            resource_type: None,
            source,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account_id(mut self, value: Option<String>) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn agent_csm_vm_containers_enabled(mut self, value: Option<bool>) -> Self {
        self.agent_csm_vm_containers_enabled = Some(value);
        self
    }

    pub fn agent_csm_vm_hosts_enabled(mut self, value: Option<bool>) -> Self {
        self.agent_csm_vm_hosts_enabled = Some(value);
        self
    }

    pub fn agent_cws_enabled(mut self, value: Option<bool>) -> Self {
        self.agent_cws_enabled = Some(value);
        self
    }

    pub fn agent_posture_management(mut self, value: Option<bool>) -> Self {
        self.agent_posture_management = Some(value);
        self
    }

    pub fn agent_version(mut self, value: Option<String>) -> Self {
        self.agent_version = Some(value);
        self
    }

    pub fn agentless_posture_management(mut self, value: Option<bool>) -> Self {
        self.agentless_posture_management = Some(value);
        self
    }

    pub fn agentless_vulnerability_scanning(mut self, value: Option<bool>) -> Self {
        self.agentless_vulnerability_scanning = Some(value);
        self
    }

    pub fn cloud_provider(mut self, value: crate::datadogV2::model::CsmCloudProvider) -> Self {
        self.cloud_provider = Some(value);
        self
    }

    pub fn cluster_name(mut self, value: Option<String>) -> Self {
        self.cluster_name = Some(value);
        self
    }

    pub fn datadog_agent_key(mut self, value: Option<String>) -> Self {
        self.datadog_agent_key = Some(value);
        self
    }

    pub fn env(mut self, value: Option<Vec<String>>) -> Self {
        self.env = Some(value);
        self
    }

    pub fn host_id(mut self, value: Option<i64>) -> Self {
        self.host_id = Some(value);
        self
    }

    pub fn install_method_tool(mut self, value: Option<String>) -> Self {
        self.install_method_tool = Some(value);
        self
    }

    pub fn os(mut self, value: Option<String>) -> Self {
        self.os = Some(value);
        self
    }

    pub fn resource_type(
        mut self,
        value: crate::datadogV2::model::CsmAgentlessHostResourceType,
    ) -> Self {
        self.resource_type = Some(value);
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

impl<'de> Deserialize<'de> for CsmUnifiedHostAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CsmUnifiedHostAttributesVisitor;
        impl<'a> Visitor<'a> for CsmUnifiedHostAttributesVisitor {
            type Value = CsmUnifiedHostAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<Option<String>> = None;
                let mut agent_csm_vm_containers_enabled: Option<Option<bool>> = None;
                let mut agent_csm_vm_hosts_enabled: Option<Option<bool>> = None;
                let mut agent_cws_enabled: Option<Option<bool>> = None;
                let mut agent_posture_management: Option<Option<bool>> = None;
                let mut agent_version: Option<Option<String>> = None;
                let mut agentless_posture_management: Option<Option<bool>> = None;
                let mut agentless_vulnerability_scanning: Option<Option<bool>> = None;
                let mut cloud_provider: Option<crate::datadogV2::model::CsmCloudProvider> = None;
                let mut cluster_name: Option<Option<String>> = None;
                let mut datadog_agent_key: Option<Option<String>> = None;
                let mut env: Option<Option<Vec<String>>> = None;
                let mut host_id: Option<Option<i64>> = None;
                let mut install_method_tool: Option<Option<String>> = None;
                let mut os: Option<Option<String>> = None;
                let mut resource_type: Option<
                    crate::datadogV2::model::CsmAgentlessHostResourceType,
                > = None;
                let mut source: Option<crate::datadogV2::model::CsmUnifiedHostSource> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "agent_csm_vm_containers_enabled" => {
                            agent_csm_vm_containers_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "agent_csm_vm_hosts_enabled" => {
                            agent_csm_vm_hosts_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "agent_cws_enabled" => {
                            agent_cws_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "agent_posture_management" => {
                            agent_posture_management =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "agent_version" => {
                            agent_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "agentless_posture_management" => {
                            agentless_posture_management =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "agentless_vulnerability_scanning" => {
                            agentless_vulnerability_scanning =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cloud_provider" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_provider =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _cloud_provider) = cloud_provider {
                                match _cloud_provider {
                                    crate::datadogV2::model::CsmCloudProvider::UnparsedObject(
                                        _cloud_provider,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "cluster_name" => {
                            cluster_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "datadog_agent_key" => {
                            datadog_agent_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "env" => {
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "host_id" => {
                            host_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "install_method_tool" => {
                            install_method_tool =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "os" => {
                            os = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_type" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _resource_type) = resource_type {
                                match _resource_type {
                                    crate::datadogV2::model::CsmAgentlessHostResourceType::UnparsedObject(_resource_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _source) = source {
                                match _source {
                                    crate::datadogV2::model::CsmUnifiedHostSource::UnparsedObject(_source) => {
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
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;

                let content = CsmUnifiedHostAttributes {
                    account_id,
                    agent_csm_vm_containers_enabled,
                    agent_csm_vm_hosts_enabled,
                    agent_cws_enabled,
                    agent_posture_management,
                    agent_version,
                    agentless_posture_management,
                    agentless_vulnerability_scanning,
                    cloud_provider,
                    cluster_name,
                    datadog_agent_key,
                    env,
                    host_id,
                    install_method_tool,
                    os,
                    resource_type,
                    source,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CsmUnifiedHostAttributesVisitor)
    }
}
