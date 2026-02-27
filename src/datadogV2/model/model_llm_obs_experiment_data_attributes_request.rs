// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating an LLM Observability experiment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsExperimentDataAttributesRequest {
    /// Configuration parameters for the experiment.
    #[serde(rename = "config")]
    pub config: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Identifier of the dataset used in this experiment.
    #[serde(rename = "dataset_id")]
    pub dataset_id: String,
    /// Version of the dataset to use. Defaults to the current version if not specified.
    #[serde(rename = "dataset_version")]
    pub dataset_version: Option<i64>,
    /// Description of the experiment.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Whether to ensure the experiment name is unique. Defaults to `true`.
    #[serde(rename = "ensure_unique")]
    pub ensure_unique: Option<bool>,
    /// Arbitrary metadata associated with the experiment.
    #[serde(rename = "metadata")]
    pub metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Name of the experiment.
    #[serde(rename = "name")]
    pub name: String,
    /// Identifier of the project this experiment belongs to.
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsExperimentDataAttributesRequest {
    pub fn new(
        dataset_id: String,
        name: String,
        project_id: String,
    ) -> LLMObsExperimentDataAttributesRequest {
        LLMObsExperimentDataAttributesRequest {
            config: None,
            dataset_id,
            dataset_version: None,
            description: None,
            ensure_unique: None,
            metadata: None,
            name,
            project_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn config(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.config = Some(value);
        self
    }

    pub fn dataset_version(mut self, value: i64) -> Self {
        self.dataset_version = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn ensure_unique(mut self, value: bool) -> Self {
        self.ensure_unique = Some(value);
        self
    }

    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for LLMObsExperimentDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsExperimentDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for LLMObsExperimentDataAttributesRequestVisitor {
            type Value = LLMObsExperimentDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut config: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut dataset_id: Option<String> = None;
                let mut dataset_version: Option<i64> = None;
                let mut description: Option<String> = None;
                let mut ensure_unique: Option<bool> = None;
                let mut metadata: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut name: Option<String> = None;
                let mut project_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "config" => {
                            if v.is_null() {
                                continue;
                            }
                            config = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dataset_id" => {
                            dataset_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dataset_version" => {
                            if v.is_null() {
                                continue;
                            }
                            dataset_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ensure_unique" => {
                            if v.is_null() {
                                continue;
                            }
                            ensure_unique =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "project_id" => {
                            project_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let dataset_id = dataset_id.ok_or_else(|| M::Error::missing_field("dataset_id"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let project_id = project_id.ok_or_else(|| M::Error::missing_field("project_id"))?;

                let content = LLMObsExperimentDataAttributesRequest {
                    config,
                    dataset_id,
                    dataset_version,
                    description,
                    ensure_unique,
                    metadata,
                    name,
                    project_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsExperimentDataAttributesRequestVisitor)
    }
}
