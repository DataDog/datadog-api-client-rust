// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A CSM Agent returned by the API.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CsmAgentsAttributes {
    /// Version of the Datadog Agent.
    #[serde(rename = "agent_version")]
    pub agent_version: Option<String>,
    /// AWS Fargate details.
    #[serde(rename = "aws_fargate")]
    pub aws_fargate: Option<String>,
    /// List of cluster names associated with the Agent.
    #[serde(rename = "cluster_name")]
    pub cluster_name: Option<Vec<String>>,
    /// Unique identifier for the Datadog Agent.
    #[serde(rename = "datadog_agent")]
    pub datadog_agent: Option<String>,
    /// ARN of the ECS Fargate task.
    #[serde(rename = "ecs_fargate_task_arn")]
    pub ecs_fargate_task_arn: Option<String>,
    /// List of environments associated with the Agent.
    #[serde(rename = "envs", default, with = "::serde_with::rust::double_option")]
    pub envs: Option<Option<Vec<String>>>,
    /// ID of the host.
    #[serde(rename = "host_id")]
    pub host_id: Option<i64>,
    /// Name of the host.
    #[serde(rename = "hostname")]
    pub hostname: Option<String>,
    /// Version of the installer used for installing the Datadog Agent.
    #[serde(rename = "install_method_installer_version")]
    pub install_method_installer_version: Option<String>,
    /// Tool used for installing the Datadog Agent.
    #[serde(rename = "install_method_tool")]
    pub install_method_tool: Option<String>,
    /// Indicates if CSM VM Containers is enabled.
    #[serde(
        rename = "is_csm_vm_containers_enabled",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub is_csm_vm_containers_enabled: Option<Option<bool>>,
    /// Indicates if CSM VM Hosts is enabled.
    #[serde(
        rename = "is_csm_vm_hosts_enabled",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub is_csm_vm_hosts_enabled: Option<Option<bool>>,
    /// Indicates if CSPM is enabled.
    #[serde(
        rename = "is_cspm_enabled",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub is_cspm_enabled: Option<Option<bool>>,
    /// Indicates if CWS is enabled.
    #[serde(
        rename = "is_cws_enabled",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub is_cws_enabled: Option<Option<bool>>,
    /// Indicates if CWS Remote Configuration is enabled.
    #[serde(
        rename = "is_cws_remote_configuration_enabled",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub is_cws_remote_configuration_enabled: Option<Option<bool>>,
    /// Indicates if Remote Configuration is enabled.
    #[serde(
        rename = "is_remote_configuration_enabled",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub is_remote_configuration_enabled: Option<Option<bool>>,
    /// Operating system of the host.
    #[serde(rename = "os")]
    pub os: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CsmAgentsAttributes {
    pub fn new() -> CsmAgentsAttributes {
        CsmAgentsAttributes {
            agent_version: None,
            aws_fargate: None,
            cluster_name: None,
            datadog_agent: None,
            ecs_fargate_task_arn: None,
            envs: None,
            host_id: None,
            hostname: None,
            install_method_installer_version: None,
            install_method_tool: None,
            is_csm_vm_containers_enabled: None,
            is_csm_vm_hosts_enabled: None,
            is_cspm_enabled: None,
            is_cws_enabled: None,
            is_cws_remote_configuration_enabled: None,
            is_remote_configuration_enabled: None,
            os: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn agent_version(mut self, value: String) -> Self {
        self.agent_version = Some(value);
        self
    }

    pub fn aws_fargate(mut self, value: String) -> Self {
        self.aws_fargate = Some(value);
        self
    }

    pub fn cluster_name(mut self, value: Vec<String>) -> Self {
        self.cluster_name = Some(value);
        self
    }

    pub fn datadog_agent(mut self, value: String) -> Self {
        self.datadog_agent = Some(value);
        self
    }

    pub fn ecs_fargate_task_arn(mut self, value: String) -> Self {
        self.ecs_fargate_task_arn = Some(value);
        self
    }

    pub fn envs(mut self, value: Option<Vec<String>>) -> Self {
        self.envs = Some(value);
        self
    }

    pub fn host_id(mut self, value: i64) -> Self {
        self.host_id = Some(value);
        self
    }

    pub fn hostname(mut self, value: String) -> Self {
        self.hostname = Some(value);
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

    pub fn is_csm_vm_containers_enabled(mut self, value: Option<bool>) -> Self {
        self.is_csm_vm_containers_enabled = Some(value);
        self
    }

    pub fn is_csm_vm_hosts_enabled(mut self, value: Option<bool>) -> Self {
        self.is_csm_vm_hosts_enabled = Some(value);
        self
    }

    pub fn is_cspm_enabled(mut self, value: Option<bool>) -> Self {
        self.is_cspm_enabled = Some(value);
        self
    }

    pub fn is_cws_enabled(mut self, value: Option<bool>) -> Self {
        self.is_cws_enabled = Some(value);
        self
    }

    pub fn is_cws_remote_configuration_enabled(mut self, value: Option<bool>) -> Self {
        self.is_cws_remote_configuration_enabled = Some(value);
        self
    }

    pub fn is_remote_configuration_enabled(mut self, value: Option<bool>) -> Self {
        self.is_remote_configuration_enabled = Some(value);
        self
    }

    pub fn os(mut self, value: String) -> Self {
        self.os = Some(value);
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

impl Default for CsmAgentsAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CsmAgentsAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CsmAgentsAttributesVisitor;
        impl<'a> Visitor<'a> for CsmAgentsAttributesVisitor {
            type Value = CsmAgentsAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut agent_version: Option<String> = None;
                let mut aws_fargate: Option<String> = None;
                let mut cluster_name: Option<Vec<String>> = None;
                let mut datadog_agent: Option<String> = None;
                let mut ecs_fargate_task_arn: Option<String> = None;
                let mut envs: Option<Option<Vec<String>>> = None;
                let mut host_id: Option<i64> = None;
                let mut hostname: Option<String> = None;
                let mut install_method_installer_version: Option<String> = None;
                let mut install_method_tool: Option<String> = None;
                let mut is_csm_vm_containers_enabled: Option<Option<bool>> = None;
                let mut is_csm_vm_hosts_enabled: Option<Option<bool>> = None;
                let mut is_cspm_enabled: Option<Option<bool>> = None;
                let mut is_cws_enabled: Option<Option<bool>> = None;
                let mut is_cws_remote_configuration_enabled: Option<Option<bool>> = None;
                let mut is_remote_configuration_enabled: Option<Option<bool>> = None;
                let mut os: Option<String> = None;
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
                        "aws_fargate" => {
                            if v.is_null() {
                                continue;
                            }
                            aws_fargate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cluster_name" => {
                            if v.is_null() {
                                continue;
                            }
                            cluster_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "datadog_agent" => {
                            if v.is_null() {
                                continue;
                            }
                            datadog_agent =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ecs_fargate_task_arn" => {
                            if v.is_null() {
                                continue;
                            }
                            ecs_fargate_task_arn =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "envs" => {
                            envs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "host_id" => {
                            if v.is_null() {
                                continue;
                            }
                            host_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hostname" => {
                            if v.is_null() {
                                continue;
                            }
                            hostname = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "is_csm_vm_containers_enabled" => {
                            is_csm_vm_containers_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_csm_vm_hosts_enabled" => {
                            is_csm_vm_hosts_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_cspm_enabled" => {
                            is_cspm_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_cws_enabled" => {
                            is_cws_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_cws_remote_configuration_enabled" => {
                            is_cws_remote_configuration_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_remote_configuration_enabled" => {
                            is_remote_configuration_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "os" => {
                            if v.is_null() {
                                continue;
                            }
                            os = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CsmAgentsAttributes {
                    agent_version,
                    aws_fargate,
                    cluster_name,
                    datadog_agent,
                    ecs_fargate_task_arn,
                    envs,
                    host_id,
                    hostname,
                    install_method_installer_version,
                    install_method_tool,
                    is_csm_vm_containers_enabled,
                    is_csm_vm_hosts_enabled,
                    is_cspm_enabled,
                    is_cws_enabled,
                    is_cws_remote_configuration_enabled,
                    is_remote_configuration_enabled,
                    os,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CsmAgentsAttributesVisitor)
    }
}
