// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single LLM Observability dataset record.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsDatasetRecordDataResponse {
    /// Timestamp when the record was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Identifier of the dataset this record belongs to.
    #[serde(rename = "dataset_id")]
    pub dataset_id: String,
    /// Represents any valid JSON value.
    #[serialize_always]
    #[serde(rename = "expected_output")]
    pub expected_output: Option<crate::datadogV2::model::AnyValue>,
    /// Unique identifier of the record.
    #[serde(rename = "id")]
    pub id: String,
    /// Represents any valid JSON value.
    #[serialize_always]
    #[serde(rename = "input")]
    pub input: Option<crate::datadogV2::model::AnyValue>,
    /// Arbitrary metadata associated with the record.
    #[serialize_always]
    #[serde(rename = "metadata")]
    pub metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Timestamp when the record was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsDatasetRecordDataResponse {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        dataset_id: String,
        expected_output: Option<crate::datadogV2::model::AnyValue>,
        id: String,
        input: Option<crate::datadogV2::model::AnyValue>,
        metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> LLMObsDatasetRecordDataResponse {
        LLMObsDatasetRecordDataResponse {
            created_at,
            dataset_id,
            expected_output,
            id,
            input,
            metadata,
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

impl<'de> Deserialize<'de> for LLMObsDatasetRecordDataResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsDatasetRecordDataResponseVisitor;
        impl<'a> Visitor<'a> for LLMObsDatasetRecordDataResponseVisitor {
            type Value = LLMObsDatasetRecordDataResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut dataset_id: Option<String> = None;
                let mut expected_output: Option<Option<crate::datadogV2::model::AnyValue>> = None;
                let mut id: Option<String> = None;
                let mut input: Option<Option<crate::datadogV2::model::AnyValue>> = None;
                let mut metadata: Option<
                    Option<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
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
                        "dataset_id" => {
                            dataset_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expected_output" => {
                            expected_output =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _expected_output) = expected_output {
                                match _expected_output {
                                    Some(crate::datadogV2::model::AnyValue::UnparsedObject(
                                        _expected_output,
                                    )) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "input" => {
                            input = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _input) = input {
                                match _input {
                                    Some(crate::datadogV2::model::AnyValue::UnparsedObject(
                                        _input,
                                    )) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "metadata" => {
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let dataset_id = dataset_id.ok_or_else(|| M::Error::missing_field("dataset_id"))?;
                let expected_output =
                    expected_output.ok_or_else(|| M::Error::missing_field("expected_output"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let input = input.ok_or_else(|| M::Error::missing_field("input"))?;
                let metadata = metadata.ok_or_else(|| M::Error::missing_field("metadata"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = LLMObsDatasetRecordDataResponse {
                    created_at,
                    dataset_id,
                    expected_output,
                    id,
                    input,
                    metadata,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsDatasetRecordDataResponseVisitor)
    }
}
