// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Metadata associated with your host.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostMeta {
    /// A list of Agent checks running on the host.
    #[serde(rename = "agent_checks")]
    pub agent_checks: Option<Vec<Vec<std::collections::HashMap<String, serde_json::Value>>>>,
    /// The Datadog Agent version.
    #[serde(rename = "agent_version")]
    pub agent_version: Option<String>,
    /// The number of cores.
    #[serde(rename = "cpuCores")]
    pub cpu_cores: Option<i64>,
    /// An array of Mac versions.
    #[serde(rename = "fbsdV")]
    pub fbsd_v: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    /// JSON string containing system information.
    #[serde(rename = "gohai")]
    pub gohai: Option<String>,
    /// Agent install method.
    #[serde(rename = "install_method")]
    pub install_method: Option<Box<crate::datadogV1::model::HostMetaInstallMethod>>,
    /// An array of Mac versions.
    #[serde(rename = "macV")]
    pub mac_v: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    /// The machine architecture.
    #[serde(rename = "machine")]
    pub machine: Option<String>,
    /// Array of Unix versions.
    #[serde(rename = "nixV")]
    pub nix_v: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
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
    pub win_v: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
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
        }
    }
}
