// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A record to insert as part of a batch update on an LLM Observability dataset.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsDatasetBatchUpdateInsertRecord {
    /// Represents any valid JSON value.
    #[serde(
        rename = "expected_output",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub expected_output: Option<Option<crate::datadogV2::model::AnyValue>>,
    /// Optional user-provided identifier for the record. If omitted, the server generates an identifier.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Represents any valid JSON value.
    #[serialize_always]
    #[serde(rename = "input")]
    pub input: Option<crate::datadogV2::model::AnyValue>,
    /// Arbitrary metadata associated with the record.
    #[serde(rename = "metadata")]
    pub metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Explicit tag operations for updating records. Operations are applied in order, Remove then Add then Set. `set` is the final override; if specified, the result of `remove` and `add` is discarded.
    #[serde(rename = "tag_operations")]
    pub tag_operations: Option<crate::datadogV2::model::LLMObsDatasetRecordTagOperations>,
    /// List of tag strings.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsDatasetBatchUpdateInsertRecord {
    pub fn new(
        input: Option<crate::datadogV2::model::AnyValue>,
    ) -> LLMObsDatasetBatchUpdateInsertRecord {
        LLMObsDatasetBatchUpdateInsertRecord {
            expected_output: None,
            id: None,
            input,
            metadata: None,
            tag_operations: None,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn expected_output(mut self, value: Option<crate::datadogV2::model::AnyValue>) -> Self {
        self.expected_output = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn tag_operations(
        mut self,
        value: crate::datadogV2::model::LLMObsDatasetRecordTagOperations,
    ) -> Self {
        self.tag_operations = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsDatasetBatchUpdateInsertRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsDatasetBatchUpdateInsertRecordVisitor;
        impl<'a> Visitor<'a> for LLMObsDatasetBatchUpdateInsertRecordVisitor {
            type Value = LLMObsDatasetBatchUpdateInsertRecord;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut expected_output: Option<Option<crate::datadogV2::model::AnyValue>> = None;
                let mut id: Option<String> = None;
                let mut input: Option<Option<crate::datadogV2::model::AnyValue>> = None;
                let mut metadata: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut tag_operations: Option<
                    crate::datadogV2::model::LLMObsDatasetRecordTagOperations,
                > = None;
                let mut tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                            if v.is_null() {
                                continue;
                            }
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
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_operations" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_operations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let input = input.ok_or_else(|| M::Error::missing_field("input"))?;

                let content = LLMObsDatasetBatchUpdateInsertRecord {
                    expected_output,
                    id,
                    input,
                    metadata,
                    tag_operations,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsDatasetBatchUpdateInsertRecordVisitor)
    }
}
