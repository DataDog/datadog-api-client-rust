// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A record update payload for an LLM Observability dataset.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsDatasetRecordUpdateItem {
    /// Represents any valid JSON value.
    #[serde(
        rename = "expected_output",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub expected_output: Option<Option<crate::datadogV2::model::AnyValue>>,
    /// Unique identifier of the record to update.
    #[serde(rename = "id")]
    pub id: String,
    /// Represents any valid JSON value.
    #[serde(rename = "input", default, with = "::serde_with::rust::double_option")]
    pub input: Option<Option<crate::datadogV2::model::AnyValue>>,
    /// Updated metadata associated with the record.
    #[serde(rename = "metadata")]
    pub metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsDatasetRecordUpdateItem {
    pub fn new(id: String) -> LLMObsDatasetRecordUpdateItem {
        LLMObsDatasetRecordUpdateItem {
            expected_output: None,
            id,
            input: None,
            metadata: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn expected_output(mut self, value: Option<crate::datadogV2::model::AnyValue>) -> Self {
        self.expected_output = Some(value);
        self
    }

    pub fn input(mut self, value: Option<crate::datadogV2::model::AnyValue>) -> Self {
        self.input = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsDatasetRecordUpdateItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsDatasetRecordUpdateItemVisitor;
        impl<'a> Visitor<'a> for LLMObsDatasetRecordUpdateItemVisitor {
            type Value = LLMObsDatasetRecordUpdateItem;

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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;

                let content = LLMObsDatasetRecordUpdateItem {
                    expected_output,
                    id,
                    input,
                    metadata,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsDatasetRecordUpdateItemVisitor)
    }
}
