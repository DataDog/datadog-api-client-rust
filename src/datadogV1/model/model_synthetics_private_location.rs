// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing information about the private location to create.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsPrivateLocation {
    /// Description of the private location.
    #[serde(rename = "description")]
    pub description: String,
    /// Unique identifier of the private location.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Object containing metadata about the private location.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV1::model::SyntheticsPrivateLocationMetadata>,
    /// Name of the private location.
    #[serde(rename = "name")]
    pub name: String,
    /// Secrets for the private location. Only present in the response when creating the private location.
    #[serde(rename = "secrets")]
    pub secrets: Option<crate::datadogV1::model::SyntheticsPrivateLocationSecrets>,
    /// Array of tags attached to the private location.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsPrivateLocation {
    pub fn new(description: String, name: String, tags: Vec<String>) -> SyntheticsPrivateLocation {
        SyntheticsPrivateLocation {
            description,
            id: None,
            metadata: None,
            name,
            secrets: None,
            tags,
            _unparsed: false,
        }
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn metadata(
        mut self,
        value: crate::datadogV1::model::SyntheticsPrivateLocationMetadata,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn secrets(
        mut self,
        value: crate::datadogV1::model::SyntheticsPrivateLocationSecrets,
    ) -> Self {
        self.secrets = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SyntheticsPrivateLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsPrivateLocationVisitor;
        impl<'a> Visitor<'a> for SyntheticsPrivateLocationVisitor {
            type Value = SyntheticsPrivateLocation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut id: Option<String> = None;
                let mut metadata: Option<
                    crate::datadogV1::model::SyntheticsPrivateLocationMetadata,
                > = None;
                let mut name: Option<String> = None;
                let mut secrets: Option<crate::datadogV1::model::SyntheticsPrivateLocationSecrets> =
                    None;
                let mut tags: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "secrets" => {
                            if v.is_null() {
                                continue;
                            }
                            secrets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;

                let content = SyntheticsPrivateLocation {
                    description,
                    id,
                    metadata,
                    name,
                    secrets,
                    tags,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsPrivateLocationVisitor)
    }
}
