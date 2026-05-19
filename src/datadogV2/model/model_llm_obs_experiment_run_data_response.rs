// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data object for an LLM Observability experiment run.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsExperimentRunDataResponse {
    /// Aggregated metric data for this run.
    #[serde(
        rename = "aggregate_data",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub aggregate_data: Option<Option<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// Timestamp when the run was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Identifier of the experiment this run belongs to.
    #[serde(rename = "experiment_id")]
    pub experiment_id: Option<String>,
    /// Unique identifier of the experiment run.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Sequential number of this run within the experiment.
    #[serde(rename = "run_number")]
    pub run_number: Option<i32>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsExperimentRunDataResponse {
    pub fn new() -> LLMObsExperimentRunDataResponse {
        LLMObsExperimentRunDataResponse {
            aggregate_data: None,
            created_at: None,
            experiment_id: None,
            id: None,
            run_number: None,
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

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn experiment_id(mut self, value: String) -> Self {
        self.experiment_id = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn run_number(mut self, value: i32) -> Self {
        self.run_number = Some(value);
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

impl Default for LLMObsExperimentRunDataResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsExperimentRunDataResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsExperimentRunDataResponseVisitor;
        impl<'a> Visitor<'a> for LLMObsExperimentRunDataResponseVisitor {
            type Value = LLMObsExperimentRunDataResponse;

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
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut experiment_id: Option<String> = None;
                let mut id: Option<String> = None;
                let mut run_number: Option<i32> = None;
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
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "experiment_id" => {
                            if v.is_null() {
                                continue;
                            }
                            experiment_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "run_number" => {
                            if v.is_null() {
                                continue;
                            }
                            run_number = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LLMObsExperimentRunDataResponse {
                    aggregate_data,
                    created_at,
                    experiment_id,
                    id,
                    run_number,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsExperimentRunDataResponseVisitor)
    }
}
