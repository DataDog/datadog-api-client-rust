// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Agent install method.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostMetaInstallMethod {
    /// The installer version.
    #[serde(rename = "installer_version")]
    pub installer_version: Option<String>,
    /// Tool used to install the agent.
    #[serde(rename = "tool")]
    pub tool: Option<String>,
    /// The tool version.
    #[serde(rename = "tool_version")]
    pub tool_version: Option<String>,
}

impl HostMetaInstallMethod {
    pub fn new() -> HostMetaInstallMethod {
        HostMetaInstallMethod {
            installer_version: None,
            tool: None,
            tool_version: None,
        }
    }
}
