// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for a container.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            _unparsed: false,
        }
    }

    pub fn container_id(mut self, value: String) -> Self {
        self.container_id = Some(value);
        self
    }

    pub fn created_at(mut self, value: String) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn host(mut self, value: String) -> Self {
        self.host = Some(value);
        self
    }

    pub fn image_digest(mut self, value: Option<String>) -> Self {
        self.image_digest = Some(value);
        self
    }

    pub fn image_name(mut self, value: String) -> Self {
        self.image_name = Some(value);
        self
    }

    pub fn image_tags(mut self, value: Option<Vec<String>>) -> Self {
        self.image_tags = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn started_at(mut self, value: String) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn state(mut self, value: String) -> Self {
        self.state = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }
}

impl Default for ContainerAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ContainerAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContainerAttributesVisitor;
        impl<'a> Visitor<'a> for ContainerAttributesVisitor {
            type Value = ContainerAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut container_id: Option<String> = None;
                let mut created_at: Option<String> = None;
                let mut host: Option<String> = None;
                let mut image_digest: Option<Option<String>> = None;
                let mut image_name: Option<String> = None;
                let mut image_tags: Option<Option<Vec<String>>> = None;
                let mut name: Option<String> = None;
                let mut started_at: Option<String> = None;
                let mut state: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "container_id" => {
                            if v.is_null() {
                                continue;
                            }
                            container_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "host" => {
                            if v.is_null() {
                                continue;
                            }
                            host = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "image_digest" => {
                            image_digest =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "image_name" => {
                            if v.is_null() {
                                continue;
                            }
                            image_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "image_tags" => {
                            image_tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "started_at" => {
                            if v.is_null() {
                                continue;
                            }
                            started_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            if v.is_null() {
                                continue;
                            }
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ContainerAttributes {
                    container_id,
                    created_at,
                    host,
                    image_digest,
                    image_name,
                    image_tags,
                    name,
                    started_at,
                    state,
                    tags,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ContainerAttributesVisitor)
    }
}
