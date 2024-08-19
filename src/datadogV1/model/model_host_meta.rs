// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata associated with your host.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HostMeta {
    /// A list of Agent checks running on the host.
    #[serde(rename = "agent_checks")]
    pub agent_checks: Option<Vec<Vec<serde_json::Value>>>,
    /// The Datadog Agent version.
    #[serde(rename = "agent_version")]
    pub agent_version: Option<String>,
    /// The number of cores.
    #[serde(rename = "cpuCores")]
    pub cpu_cores: Option<i64>,
    /// An array of Mac versions.
    #[serde(rename = "fbsdV")]
    pub fbsd_v: Option<Vec<serde_json::Value>>,
    /// JSON string containing system information.
    #[serde(rename = "gohai")]
    pub gohai: Option<String>,
    /// Agent install method.
    #[serde(rename = "install_method")]
    pub install_method: Option<crate::datadogV1::model::HostMetaInstallMethod>,
    /// An array of Mac versions.
    #[serde(rename = "macV")]
    pub mac_v: Option<Vec<serde_json::Value>>,
    /// The machine architecture.
    #[serde(rename = "machine")]
    pub machine: Option<String>,
    /// Array of Unix versions.
    #[serde(rename = "nixV")]
    pub nix_v: Option<Vec<serde_json::Value>>,
    /// The OS platform.
    #[serde(rename = "platform")]
    pub platform: Option<String>,
    /// The processor.
    #[serde(rename = "processor")]
    pub processor: Option<String>,
    /// The Python version.
    #[serde(rename = "pythonV")]
    pub python_v: Option<String>,
    /// The socket fqdn.
    #[serde(rename = "socket-fqdn")]
    pub socket_fqdn: Option<String>,
    /// The socket hostname.
    #[serde(rename = "socket-hostname")]
    pub socket_hostname: Option<String>,
    /// An array of Windows versions.
    #[serde(rename = "winV")]
    pub win_v: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HostMeta {
    pub fn new() -> HostMeta {
        HostMeta {
            agent_checks: None,
            agent_version: None,
            cpu_cores: None,
            fbsd_v: None,
            gohai: None,
            install_method: None,
            mac_v: None,
            machine: None,
            nix_v: None,
            platform: None,
            processor: None,
            python_v: None,
            socket_fqdn: None,
            socket_hostname: None,
            win_v: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn agent_checks(mut self, value: Vec<Vec<serde_json::Value>>) -> Self {
        self.agent_checks = Some(value);
        self
    }

    pub fn agent_version(mut self, value: String) -> Self {
        self.agent_version = Some(value);
        self
    }

    pub fn cpu_cores(mut self, value: i64) -> Self {
        self.cpu_cores = Some(value);
        self
    }

    pub fn fbsd_v(mut self, value: Vec<serde_json::Value>) -> Self {
        self.fbsd_v = Some(value);
        self
    }

    pub fn gohai(mut self, value: String) -> Self {
        self.gohai = Some(value);
        self
    }

    pub fn install_method(mut self, value: crate::datadogV1::model::HostMetaInstallMethod) -> Self {
        self.install_method = Some(value);
        self
    }

    pub fn mac_v(mut self, value: Vec<serde_json::Value>) -> Self {
        self.mac_v = Some(value);
        self
    }

    pub fn machine(mut self, value: String) -> Self {
        self.machine = Some(value);
        self
    }

    pub fn nix_v(mut self, value: Vec<serde_json::Value>) -> Self {
        self.nix_v = Some(value);
        self
    }

    pub fn platform(mut self, value: String) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn processor(mut self, value: String) -> Self {
        self.processor = Some(value);
        self
    }

    pub fn python_v(mut self, value: String) -> Self {
        self.python_v = Some(value);
        self
    }

    pub fn socket_fqdn(mut self, value: String) -> Self {
        self.socket_fqdn = Some(value);
        self
    }

    pub fn socket_hostname(mut self, value: String) -> Self {
        self.socket_hostname = Some(value);
        self
    }

    pub fn win_v(mut self, value: Vec<serde_json::Value>) -> Self {
        self.win_v = Some(value);
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

impl Default for HostMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HostMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostMetaVisitor;
        impl<'a> Visitor<'a> for HostMetaVisitor {
            type Value = HostMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut agent_checks: Option<Vec<Vec<serde_json::Value>>> = None;
                let mut agent_version: Option<String> = None;
                let mut cpu_cores: Option<i64> = None;
                let mut fbsd_v: Option<Vec<serde_json::Value>> = None;
                let mut gohai: Option<String> = None;
                let mut install_method: Option<crate::datadogV1::model::HostMetaInstallMethod> =
                    None;
                let mut mac_v: Option<Vec<serde_json::Value>> = None;
                let mut machine: Option<String> = None;
                let mut nix_v: Option<Vec<serde_json::Value>> = None;
                let mut platform: Option<String> = None;
                let mut processor: Option<String> = None;
                let mut python_v: Option<String> = None;
                let mut socket_fqdn: Option<String> = None;
                let mut socket_hostname: Option<String> = None;
                let mut win_v: Option<Vec<serde_json::Value>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "agent_checks" => {
                            if v.is_null() {
                                continue;
                            }
                            agent_checks =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "agent_version" => {
                            if v.is_null() {
                                continue;
                            }
                            agent_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cpuCores" => {
                            if v.is_null() {
                                continue;
                            }
                            cpu_cores = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fbsdV" => {
                            if v.is_null() {
                                continue;
                            }
                            fbsd_v = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gohai" => {
                            if v.is_null() {
                                continue;
                            }
                            gohai = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "install_method" => {
                            if v.is_null() {
                                continue;
                            }
                            install_method =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "macV" => {
                            if v.is_null() {
                                continue;
                            }
                            mac_v = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "machine" => {
                            if v.is_null() {
                                continue;
                            }
                            machine = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "nixV" => {
                            if v.is_null() {
                                continue;
                            }
                            nix_v = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "platform" => {
                            if v.is_null() {
                                continue;
                            }
                            platform = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "processor" => {
                            if v.is_null() {
                                continue;
                            }
                            processor = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pythonV" => {
                            if v.is_null() {
                                continue;
                            }
                            python_v = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "socket-fqdn" => {
                            if v.is_null() {
                                continue;
                            }
                            socket_fqdn =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "socket-hostname" => {
                            if v.is_null() {
                                continue;
                            }
                            socket_hostname =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "winV" => {
                            if v.is_null() {
                                continue;
                            }
                            win_v = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = HostMeta {
                    agent_checks,
                    agent_version,
                    cpu_cores,
                    fbsd_v,
                    gohai,
                    install_method,
                    mac_v,
                    machine,
                    nix_v,
                    platform,
                    processor,
                    python_v,
                    socket_fqdn,
                    socket_hostname,
                    win_v,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HostMetaVisitor)
    }
}
