// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostMetaInstallMethod {
    /// The installer version.
    #[serde(rename = "installer_version", skip_serializing_if = "Option::is_none")]
    pub installer_version: String,
    /// Tool used to install the agent.
    #[serde(rename = "tool", skip_serializing_if = "Option::is_none")]
    pub tool: String,
    /// The tool version.
    #[serde(rename = "tool_version", skip_serializing_if = "Option::is_none")]
    pub tool_version: String,
}

