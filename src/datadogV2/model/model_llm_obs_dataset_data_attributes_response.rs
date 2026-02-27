// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an LLM Observability dataset.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsDatasetDataAttributesResponse {
    /// Timestamp when the dataset was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Current version number of the dataset.
    #[serde(rename = "current_version")]
    pub current_version: i64,
    /// Description of the dataset.
    #[serialize_always]
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Arbitrary metadata associated with the dataset.
    #[serialize_always]
    #[serde(rename = "metadata")]
    pub metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Name of the dataset.
    #[serde(rename = "name")]
    pub name: String,
    /// Timestamp when the dataset was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsDatasetDataAttributesResponse {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        current_version: i64,
        description: Option<String>,
        metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
        name: String,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> LLMObsDatasetDataAttributesResponse {
        LLMObsDatasetDataAttributesResponse {
            created_at,
            current_version,
            description,
            metadata,
            name,
            updated_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for LLMObsDatasetDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsDatasetDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for LLMObsDatasetDataAttributesResponseVisitor {
            type Value = LLMObsDatasetDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut current_version: Option<i64> = None;
                let mut description: Option<Option<String>> = None;
                let mut metadata: Option<
                    Option<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut name: Option<String> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "current_version" => {
                            current_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let current_version =
                    current_version.ok_or_else(|| M::Error::missing_field("current_version"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let metadata = metadata.ok_or_else(|| M::Error::missing_field("metadata"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = LLMObsDatasetDataAttributesResponse {
                    created_at,
                    current_version,
                    description,
                    metadata,
                    name,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsDatasetDataAttributesResponseVisitor)
    }
}
