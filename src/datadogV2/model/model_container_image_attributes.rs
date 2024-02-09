// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes for a Container Image.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerImageAttributes {
    /// Number of containers running the image.
    #[serde(rename = "container_count")]
    pub container_count: Option<i64>,
    /// List of platform-specific images associated with the image record.
    /// The list contains more than 1 entry for multi-architecture images.
    #[serde(rename = "image_flavors")]
    pub image_flavors: Option<Vec<crate::datadogV2::model::ContainerImageFlavor>>,
    /// List of image tags associated with the Container Image.
    #[serde(rename = "image_tags")]
    pub image_tags: Option<Vec<String>>,
    /// List of build times associated with the Container Image.
    /// The list contains more than 1 entry for multi-architecture images.
    #[serde(rename = "images_built_at")]
    pub images_built_at: Option<Vec<String>>,
    /// Name of the Container Image.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// List of Operating System architectures supported by the Container Image.
    #[serde(rename = "os_architectures")]
    pub os_architectures: Option<Vec<String>>,
    /// List of Operating System names supported by the Container Image.
    #[serde(rename = "os_names")]
    pub os_names: Option<Vec<String>>,
    /// List of Operating System versions supported by the Container Image.
    #[serde(rename = "os_versions")]
    pub os_versions: Option<Vec<String>>,
    /// Time the image was pushed to the container registry.
    #[serde(rename = "published_at")]
    pub published_at: Option<String>,
    /// Registry the Container Image was pushed to.
    #[serde(rename = "registry")]
    pub registry: Option<String>,
    /// Digest of the compressed image manifest.
    #[serde(rename = "repo_digest")]
    pub repo_digest: Option<String>,
    /// Repository where the Container Image is stored in.
    #[serde(rename = "repository")]
    pub repository: Option<String>,
    /// Short version of the Container Image name.
    #[serde(rename = "short_image")]
    pub short_image: Option<String>,
    /// List of size for each platform-specific image associated with the image record.
    /// The list contains more than 1 entry for multi-architecture images.
    #[serde(rename = "sizes")]
    pub sizes: Option<Vec<i64>>,
    /// List of sources where the Container Image was collected from.
    #[serde(rename = "sources")]
    pub sources: Option<Vec<String>>,
    /// List of tags associated with the Container Image.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Vulnerability counts associated with the Container Image.
    #[serde(rename = "vulnerability_count")]
    pub vulnerability_count: Option<crate::datadogV2::model::ContainerImageVulnerabilities>,
}

impl ContainerImageAttributes {
    pub fn new() -> ContainerImageAttributes {
        ContainerImageAttributes {
            container_count: None,
            image_flavors: None,
            image_tags: None,
            images_built_at: None,
            name: None,
            os_architectures: None,
            os_names: None,
            os_versions: None,
            published_at: None,
            registry: None,
            repo_digest: None,
            repository: None,
            short_image: None,
            sizes: None,
            sources: None,
            tags: None,
            vulnerability_count: None,
        }
    }

    pub fn container_count(&mut self, value: i64) -> &mut Self {
        self.container_count = Some(value);
        self
    }

    pub fn image_flavors(
        &mut self,
        value: Vec<crate::datadogV2::model::ContainerImageFlavor>,
    ) -> &mut Self {
        self.image_flavors = Some(value);
        self
    }

    pub fn image_tags(&mut self, value: Vec<String>) -> &mut Self {
        self.image_tags = Some(value);
        self
    }

    pub fn images_built_at(&mut self, value: Vec<String>) -> &mut Self {
        self.images_built_at = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn os_architectures(&mut self, value: Vec<String>) -> &mut Self {
        self.os_architectures = Some(value);
        self
    }

    pub fn os_names(&mut self, value: Vec<String>) -> &mut Self {
        self.os_names = Some(value);
        self
    }

    pub fn os_versions(&mut self, value: Vec<String>) -> &mut Self {
        self.os_versions = Some(value);
        self
    }

    pub fn published_at(&mut self, value: String) -> &mut Self {
        self.published_at = Some(value);
        self
    }

    pub fn registry(&mut self, value: String) -> &mut Self {
        self.registry = Some(value);
        self
    }

    pub fn repo_digest(&mut self, value: String) -> &mut Self {
        self.repo_digest = Some(value);
        self
    }

    pub fn repository(&mut self, value: String) -> &mut Self {
        self.repository = Some(value);
        self
    }

    pub fn short_image(&mut self, value: String) -> &mut Self {
        self.short_image = Some(value);
        self
    }

    pub fn sizes(&mut self, value: Vec<i64>) -> &mut Self {
        self.sizes = Some(value);
        self
    }

    pub fn sources(&mut self, value: Vec<String>) -> &mut Self {
        self.sources = Some(value);
        self
    }

    pub fn tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }

    pub fn vulnerability_count(
        &mut self,
        value: crate::datadogV2::model::ContainerImageVulnerabilities,
    ) -> &mut Self {
        self.vulnerability_count = Some(value);
        self
    }
}

impl Default for ContainerImageAttributes {
    fn default() -> Self {
        Self::new()
    }
}
