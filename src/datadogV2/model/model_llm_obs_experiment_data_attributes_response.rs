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
    /// Pre-computed aggregate metrics for this experiment run, including eval score distributions, token costs, and error rates.
    #[serde(
        rename = "aggregate_data",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub aggregate_data: Option<Option<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// User data for the author of an experiment. Only present when `include[user_data]` is `true`.
    #[serde(rename = "author")]
    pub author: Option<crate::datadogV2::model::LLMObsExperimentUser>,
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
    /// Name of the dataset used in this experiment.
    /// Only present when `include[dataset_names]` is `true`.
    #[serde(
        rename = "dataset_name",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub dataset_name: Option<Option<String>>,
    /// Version of the dataset used in this experiment.
    #[serde(rename = "dataset_version")]
    pub dataset_version: Option<i64>,
    /// Timestamp when the experiment was soft-deleted, if applicable.
    #[serde(
        rename = "deleted_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub deleted_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Description of the experiment.
    #[serialize_always]
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Error message describing why the experiment failed, if applicable.
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option")]
    pub error: Option<Option<String>>,
    /// Logical name of the experiment, shared across all runs of the same pipeline.
    #[serde(rename = "experiment")]
    pub experiment: Option<String>,
    /// Arbitrary metadata associated with the experiment.
    #[serialize_always]
    #[serde(rename = "metadata")]
    pub metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Name of the experiment.
    #[serde(rename = "name")]
    pub name: String,
    /// Identifier of the parent (baseline) experiment this experiment was run against, if any.
    #[serde(
        rename = "parent_experiment_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub parent_experiment_id: Option<Option<String>>,
    /// Identifier of the project this experiment belongs to.
    #[serde(rename = "project_id")]
    pub project_id: String,
    /// Expected number of runs for this experiment.
    #[serde(rename = "run_count")]
    pub run_count: Option<i32>,
    /// Execution status of an LLM Observability experiment.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::LLMObsExperimentStatus>,
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
            aggregate_data: None,
            author: None,
            config,
            created_at,
            dataset_id,
            dataset_name: None,
            dataset_version: None,
            deleted_at: None,
            description,
            error: None,
            experiment: None,
            metadata,
            name,
            parent_experiment_id: None,
            project_id,
            run_count: None,
            status: None,
            updated_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn aggregate_data(
        mut self,
        value: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.aggregate_data = Some(value);
        self
    }

    pub fn author(mut self, value: crate::datadogV2::model::LLMObsExperimentUser) -> Self {
        self.author = Some(value);
        self
    }

    pub fn dataset_name(mut self, value: Option<String>) -> Self {
        self.dataset_name = Some(value);
        self
    }

    pub fn dataset_version(mut self, value: i64) -> Self {
        self.dataset_version = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn error(mut self, value: Option<String>) -> Self {
        self.error = Some(value);
        self
    }

    pub fn experiment(mut self, value: String) -> Self {
        self.experiment = Some(value);
        self
    }

    pub fn parent_experiment_id(mut self, value: Option<String>) -> Self {
        self.parent_experiment_id = Some(value);
        self
    }

    pub fn run_count(mut self, value: i32) -> Self {
        self.run_count = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV2::model::LLMObsExperimentStatus) -> Self {
        self.status = Some(value);
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
                let mut aggregate_data: Option<
                    Option<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut author: Option<crate::datadogV2::model::LLMObsExperimentUser> = None;
                let mut config: Option<
                    Option<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut dataset_id: Option<String> = None;
                let mut dataset_name: Option<Option<String>> = None;
                let mut dataset_version: Option<i64> = None;
                let mut deleted_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut description: Option<Option<String>> = None;
                let mut error: Option<Option<String>> = None;
                let mut experiment: Option<String> = None;
                let mut metadata: Option<
                    Option<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut name: Option<String> = None;
                let mut parent_experiment_id: Option<Option<String>> = None;
                let mut project_id: Option<String> = None;
                let mut run_count: Option<i32> = None;
                let mut status: Option<crate::datadogV2::model::LLMObsExperimentStatus> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregate_data" => {
                            aggregate_data =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "author" => {
                            if v.is_null() {
                                continue;
                            }
                            author = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "config" => {
                            config = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dataset_id" => {
                            dataset_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dataset_name" => {
                            dataset_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dataset_version" => {
                            if v.is_null() {
                                continue;
                            }
                            dataset_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deleted_at" => {
                            deleted_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error" => {
                            error = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "experiment" => {
                            if v.is_null() {
                                continue;
                            }
                            experiment = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parent_experiment_id" => {
                            parent_experiment_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project_id" => {
                            project_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "run_count" => {
                            if v.is_null() {
                                continue;
                            }
                            run_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::LLMObsExperimentStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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
                    aggregate_data,
                    author,
                    config,
                    created_at,
                    dataset_id,
                    dataset_name,
                    dataset_version,
                    deleted_at,
                    description,
                    error,
                    experiment,
                    metadata,
                    name,
                    parent_experiment_id,
                    project_id,
                    run_count,
                    status,
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
