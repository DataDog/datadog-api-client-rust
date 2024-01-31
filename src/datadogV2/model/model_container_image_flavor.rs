// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Container Image breakdown by supported platform.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerImageFlavor {
    /// Time the platform-specific Container Image was built.
    #[serde(rename = "built_at")]
    pub built_at: Option<String>,
    /// Operating System architecture supported by the Container Image.
    #[serde(rename = "os_architecture")]
    pub os_architecture: Option<String>,
    /// Operating System name supported by the Container Image.
    #[serde(rename = "os_name")]
    pub os_name: Option<String>,
    /// Operating System version supported by the Container Image.
    #[serde(rename = "os_version")]
    pub os_version: Option<String>,
    /// Size of the platform-specific Container Image.
    #[serde(rename = "size")]
    pub size: Option<i64>,
}

impl ContainerImageFlavor {
    pub fn new() -> ContainerImageFlavor {
        ContainerImageFlavor {
            built_at: None,
            os_architecture: None,
            os_name: None,
            os_version: None,
            size: None,
        }
    }

    pub fn with_built_at(&mut self, value: String) -> &mut Self {
        self.built_at = Some(value);
        self
    }

    pub fn with_os_architecture(&mut self, value: String) -> &mut Self {
        self.os_architecture = Some(value);
        self
    }

    pub fn with_os_name(&mut self, value: String) -> &mut Self {
        self.os_name = Some(value);
        self
    }

    pub fn with_os_version(&mut self, value: String) -> &mut Self {
        self.os_version = Some(value);
        self
    }

    pub fn with_size(&mut self, value: i64) -> &mut Self {
        self.size = Some(value);
        self
    }
}
impl Default for ContainerImageFlavor {
    fn default() -> Self {
        Self::new()
    }
}
