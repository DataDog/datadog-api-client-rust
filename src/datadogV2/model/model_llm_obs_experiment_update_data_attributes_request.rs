// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating an LLM Observability experiment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsExperimentUpdateDataAttributesRequest {
    /// Updated identifier of the dataset used in this experiment.
    #[serde(rename = "dataset_id")]
    pub dataset_id: Option<String>,
    /// Updated description of the experiment.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Error message describing why the experiment failed, if applicable.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Updated arbitrary metadata associated with the experiment.
    #[serde(rename = "metadata")]
    pub metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Updated name of the experiment.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Execution status of an LLM Observability experiment.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::LLMObsExperimentStatus>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsExperimentUpdateDataAttributesRequest {
    pub fn new() -> LLMObsExperimentUpdateDataAttributesRequest {
        LLMObsExperimentUpdateDataAttributesRequest {
            dataset_id: None,
            description: None,
            error: None,
            metadata: None,
            name: None,
            status: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn dataset_id(mut self, value: String) -> Self {
        self.dataset_id = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn error(mut self, value: String) -> Self {
        self.error = Some(value);
        self
    }

    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
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

impl Default for LLMObsExperimentUpdateDataAttributesRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsExperimentUpdateDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsExperimentUpdateDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for LLMObsExperimentUpdateDataAttributesRequestVisitor {
            type Value = LLMObsExperimentUpdateDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dataset_id: Option<String> = None;
                let mut description: Option<String> = None;
                let mut error: Option<String> = None;
                let mut metadata: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut name: Option<String> = None;
                let mut status: Option<crate::datadogV2::model::LLMObsExperimentStatus> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dataset_id" => {
                            if v.is_null() {
                                continue;
                            }
                            dataset_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error" => {
                            if v.is_null() {
                                continue;
                            }
                            error = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LLMObsExperimentUpdateDataAttributesRequest {
                    dataset_id,
                    description,
                    error,
                    metadata,
                    name,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsExperimentUpdateDataAttributesRequestVisitor)
    }
}
