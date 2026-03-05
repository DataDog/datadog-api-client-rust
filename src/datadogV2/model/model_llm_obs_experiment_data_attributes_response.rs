// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an LLM Observability experiment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsExperimentDataAttributesResponse {
    /// Configuration parameters for the experiment.
    #[serialize_always]
    #[serde(rename = "config")]
    pub config: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Timestamp when the experiment was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Identifier of the dataset used in this experiment.
    #[serde(rename = "dataset_id")]
    pub dataset_id: String,
    /// Description of the experiment.
    #[serialize_always]
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Arbitrary metadata associated with the experiment.
    #[serialize_always]
    #[serde(rename = "metadata")]
    pub metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Name of the experiment.
    #[serde(rename = "name")]
    pub name: String,
    /// Identifier of the project this experiment belongs to.
    #[serde(rename = "project_id")]
    pub project_id: String,
    /// Timestamp when the experiment was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsExperimentDataAttributesResponse {
    pub fn new(
        config: Option<std::collections::BTreeMap<String, serde_json::Value>>,
        created_at: chrono::DateTime<chrono::Utc>,
        dataset_id: String,
        description: Option<String>,
        metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
        name: String,
        project_id: String,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> LLMObsExperimentDataAttributesResponse {
        LLMObsExperimentDataAttributesResponse {
            config,
            created_at,
            dataset_id,
            description,
            metadata,
            name,
            project_id,
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

impl<'de> Deserialize<'de> for LLMObsExperimentDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsExperimentDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for LLMObsExperimentDataAttributesResponseVisitor {
            type Value = LLMObsExperimentDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut config: Option<
                    Option<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut dataset_id: Option<String> = None;
                let mut description: Option<Option<String>> = None;
                let mut metadata: Option<
                    Option<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut name: Option<String> = None;
                let mut project_id: Option<String> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "config" => {
                            config = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dataset_id" => {
                            dataset_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "project_id" => {
                            project_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let config = config.ok_or_else(|| M::Error::missing_field("config"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let dataset_id = dataset_id.ok_or_else(|| M::Error::missing_field("dataset_id"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let metadata = metadata.ok_or_else(|| M::Error::missing_field("metadata"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let project_id = project_id.ok_or_else(|| M::Error::missing_field("project_id"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = LLMObsExperimentDataAttributesResponse {
                    config,
                    created_at,
                    dataset_id,
                    description,
                    metadata,
                    name,
                    project_id,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsExperimentDataAttributesResponseVisitor)
    }
}
