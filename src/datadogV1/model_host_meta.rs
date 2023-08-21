// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostMeta {
    /// A list of Agent checks running on the host.
    #[serde(rename = "agent_checks", skip_serializing_if = "Option::is_none")]
    pub agent_checks: Vec<Vec<interface{}>>,
    /// The Datadog Agent version.
    #[serde(rename = "agent_version", skip_serializing_if = "Option::is_none")]
    pub agent_version: String,
    /// The number of cores.
    #[serde(rename = "cpuCores", skip_serializing_if = "Option::is_none")]
    pub cpu_cores: i64,
    /// An array of Mac versions.
    #[serde(rename = "fbsdV", skip_serializing_if = "Option::is_none")]
    pub fbsd_v: Vec<interface{}>,
    /// JSON string containing system information.
    #[serde(rename = "gohai", skip_serializing_if = "Option::is_none")]
    pub gohai: String,
    /// Agent install method.
    #[serde(rename = "install_method", skip_serializing_if = "Option::is_none")]
    pub install_method: HostMetaInstallMethod,
    /// An array of Mac versions.
    #[serde(rename = "macV", skip_serializing_if = "Option::is_none")]
    pub mac_v: Vec<interface{}>,
    /// The machine architecture.
    #[serde(rename = "machine", skip_serializing_if = "Option::is_none")]
    pub machine: String,
    /// Array of Unix versions.
    #[serde(rename = "nixV", skip_serializing_if = "Option::is_none")]
    pub nix_v: Vec<interface{}>,
    /// The OS platform.
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: String,
    /// The processor.
    #[serde(rename = "processor", skip_serializing_if = "Option::is_none")]
    pub processor: String,
    /// The Python version.
    #[serde(rename = "pythonV", skip_serializing_if = "Option::is_none")]
    pub python_v: String,
    /// The socket fqdn.
    #[serde(rename = "socket-fqdn", skip_serializing_if = "Option::is_none")]
    pub socket_fqdn: String,
    /// The socket hostname.
    #[serde(rename = "socket-hostname", skip_serializing_if = "Option::is_none")]
    pub socket_hostname: String,
    /// An array of Windows versions.
    #[serde(rename = "winV", skip_serializing_if = "Option::is_none")]
    pub win_v: Vec<interface{}>,
}

