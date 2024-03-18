// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for a Container Image.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            _unparsed: false,
        }
    }

    pub fn container_count(mut self, value: i64) -> Self {
        self.container_count = Some(value);
        self
    }

    pub fn image_flavors(
        mut self,
        value: Vec<crate::datadogV2::model::ContainerImageFlavor>,
    ) -> Self {
        self.image_flavors = Some(value);
        self
    }

    pub fn image_tags(mut self, value: Vec<String>) -> Self {
        self.image_tags = Some(value);
        self
    }

    pub fn images_built_at(mut self, value: Vec<String>) -> Self {
        self.images_built_at = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn os_architectures(mut self, value: Vec<String>) -> Self {
        self.os_architectures = Some(value);
        self
    }

    pub fn os_names(mut self, value: Vec<String>) -> Self {
        self.os_names = Some(value);
        self
    }

    pub fn os_versions(mut self, value: Vec<String>) -> Self {
        self.os_versions = Some(value);
        self
    }

    pub fn published_at(mut self, value: String) -> Self {
        self.published_at = Some(value);
        self
    }

    pub fn registry(mut self, value: String) -> Self {
        self.registry = Some(value);
        self
    }

    pub fn repo_digest(mut self, value: String) -> Self {
        self.repo_digest = Some(value);
        self
    }

    pub fn repository(mut self, value: String) -> Self {
        self.repository = Some(value);
        self
    }

    pub fn short_image(mut self, value: String) -> Self {
        self.short_image = Some(value);
        self
    }

    pub fn sizes(mut self, value: Vec<i64>) -> Self {
        self.sizes = Some(value);
        self
    }

    pub fn sources(mut self, value: Vec<String>) -> Self {
        self.sources = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn vulnerability_count(
        mut self,
        value: crate::datadogV2::model::ContainerImageVulnerabilities,
    ) -> Self {
        self.vulnerability_count = Some(value);
        self
    }
}

impl Default for ContainerImageAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ContainerImageAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContainerImageAttributesVisitor;
        impl<'a> Visitor<'a> for ContainerImageAttributesVisitor {
            type Value = ContainerImageAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut container_count: Option<i64> = None;
                let mut image_flavors: Option<Vec<crate::datadogV2::model::ContainerImageFlavor>> =
                    None;
                let mut image_tags: Option<Vec<String>> = None;
                let mut images_built_at: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut os_architectures: Option<Vec<String>> = None;
                let mut os_names: Option<Vec<String>> = None;
                let mut os_versions: Option<Vec<String>> = None;
                let mut published_at: Option<String> = None;
                let mut registry: Option<String> = None;
                let mut repo_digest: Option<String> = None;
                let mut repository: Option<String> = None;
                let mut short_image: Option<String> = None;
                let mut sizes: Option<Vec<i64>> = None;
                let mut sources: Option<Vec<String>> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut vulnerability_count: Option<
                    crate::datadogV2::model::ContainerImageVulnerabilities,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "container_count" => {
                            if v.is_null() {
                                continue;
                            }
                            container_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "image_flavors" => {
                            if v.is_null() {
                                continue;
                            }
                            image_flavors =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "image_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            image_tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "images_built_at" => {
                            if v.is_null() {
                                continue;
                            }
                            images_built_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "os_architectures" => {
                            if v.is_null() {
                                continue;
                            }
                            os_architectures =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "os_names" => {
                            if v.is_null() {
                                continue;
                            }
                            os_names = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "os_versions" => {
                            if v.is_null() {
                                continue;
                            }
                            os_versions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "published_at" => {
                            if v.is_null() {
                                continue;
                            }
                            published_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "registry" => {
                            if v.is_null() {
                                continue;
                            }
                            registry = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repo_digest" => {
                            if v.is_null() {
                                continue;
                            }
                            repo_digest =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repository" => {
                            if v.is_null() {
                                continue;
                            }
                            repository = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "short_image" => {
                            if v.is_null() {
                                continue;
                            }
                            short_image =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sizes" => {
                            if v.is_null() {
                                continue;
                            }
                            sizes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sources" => {
                            if v.is_null() {
                                continue;
                            }
                            sources = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vulnerability_count" => {
                            if v.is_null() {
                                continue;
                            }
                            vulnerability_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ContainerImageAttributes {
                    container_count,
                    image_flavors,
                    image_tags,
                    images_built_at,
                    name,
                    os_architectures,
                    os_names,
                    os_versions,
                    published_at,
                    registry,
                    repo_digest,
                    repository,
                    short_image,
                    sizes,
                    sources,
                    tags,
                    vulnerability_count,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ContainerImageAttributesVisitor)
    }
}
