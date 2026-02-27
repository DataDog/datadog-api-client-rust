// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single record to append to an LLM Observability dataset.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsDatasetRecordItem {
    /// Represents any valid JSON value.
    #[serde(
        rename = "expected_output",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub expected_output: Option<Option<crate::datadogV2::model::AnyValue>>,
    /// Represents any valid JSON value.
    #[serialize_always]
    #[serde(rename = "input")]
    pub input: Option<crate::datadogV2::model::AnyValue>,
    /// Arbitrary metadata associated with the record.
    #[serde(rename = "metadata")]
    pub metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsDatasetRecordItem {
    pub fn new(input: Option<crate::datadogV2::model::AnyValue>) -> LLMObsDatasetRecordItem {
        LLMObsDatasetRecordItem {
            expected_output: None,
            input,
            metadata: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn expected_output(mut self, value: Option<crate::datadogV2::model::AnyValue>) -> Self {
        self.expected_output = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsDatasetRecordItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsDatasetRecordItemVisitor;
        impl<'a> Visitor<'a> for LLMObsDatasetRecordItemVisitor {
            type Value = LLMObsDatasetRecordItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut expected_output: Option<Option<crate::datadogV2::model::AnyValue>> = None;
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
                let input = input.ok_or_else(|| M::Error::missing_field("input"))?;

                let content = LLMObsDatasetRecordItem {
                    expected_output,
                    input,
                    metadata,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsDatasetRecordItemVisitor)
    }
}
