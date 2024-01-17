// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes for a container.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerAttributes {
    /// The ID of the container.
    #[serde(rename = "container_id")]
    pub container_id: Option<String>,
    /// Time the container was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// Hostname of the host running the container.
    #[serde(rename = "host")]
    pub host: Option<String>,
    /// Digest of the compressed image manifest.
    #[serde(
        rename = "image_digest",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub image_digest: Option<Option<String>>,
    /// Name of the associated container image.
    #[serde(rename = "image_name")]
    pub image_name: Option<String>,
    /// List of image tags associated with the container image.
    #[serde(
        rename = "image_tags",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub image_tags: Option<Option<Vec<String>>>,
    /// Name of the container.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Time the container was started.
    #[serde(rename = "started_at")]
    pub started_at: Option<String>,
    /// State of the container. This depends on the container runtime.
    #[serde(rename = "state")]
    pub state: Option<String>,
    /// List of tags associated with the container.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
}

impl ContainerAttributes {
    pub fn new() -> ContainerAttributes {
        ContainerAttributes {
            container_id: None,
            created_at: None,
            host: None,
            image_digest: None,
            image_name: None,
            image_tags: None,
            name: None,
            started_at: None,
            state: None,
            tags: None,
        }
    }
}
impl Default for ContainerAttributes {
    fn default() -> Self {
        Self::new()
    }
}
