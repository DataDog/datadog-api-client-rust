// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata associated with an experiment span.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsExperimentSpanMeta {
    /// Error details for an experiment span.
    #[serde(rename = "error")]
    pub error: Option<crate::datadogV2::model::LLMObsExperimentSpanError>,
    /// Expected output for the span, used for evaluation.
    #[serde(rename = "expected_output")]
    pub expected_output: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Represents any valid JSON value.
    #[serde(rename = "input", default, with = "::serde_with::rust::double_option")]
    pub input: Option<Option<crate::datadogV2::model::AnyValue>>,
    /// Represents any valid JSON value.
    #[serde(rename = "output", default, with = "::serde_with::rust::double_option")]
    pub output: Option<Option<crate::datadogV2::model::AnyValue>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsExperimentSpanMeta {
    pub fn new() -> LLMObsExperimentSpanMeta {
        LLMObsExperimentSpanMeta {
            error: None,
            expected_output: None,
            input: None,
            output: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn error(mut self, value: crate::datadogV2::model::LLMObsExperimentSpanError) -> Self {
        self.error = Some(value);
        self
    }

    pub fn expected_output(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.expected_output = Some(value);
        self
    }

    pub fn input(mut self, value: Option<crate::datadogV2::model::AnyValue>) -> Self {
        self.input = Some(value);
        self
    }

    pub fn output(mut self, value: Option<crate::datadogV2::model::AnyValue>) -> Self {
        self.output = Some(value);
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

impl Default for LLMObsExperimentSpanMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsExperimentSpanMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsExperimentSpanMetaVisitor;
        impl<'a> Visitor<'a> for LLMObsExperimentSpanMetaVisitor {
            type Value = LLMObsExperimentSpanMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut error: Option<crate::datadogV2::model::LLMObsExperimentSpanError> = None;
                let mut expected_output: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut input: Option<Option<crate::datadogV2::model::AnyValue>> = None;
                let mut output: Option<Option<crate::datadogV2::model::AnyValue>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "error" => {
                            if v.is_null() {
                                continue;
                            }
                            error = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expected_output" => {
                            if v.is_null() {
                                continue;
                            }
                            expected_output =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "output" => {
                            output = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _output) = output {
                                match _output {
                                    Some(crate::datadogV2::model::AnyValue::UnparsedObject(
                                        _output,
                                    )) => {
                                        _unparsed = true;
                                    }
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

                let content = LLMObsExperimentSpanMeta {
                    error,
                    expected_output,
                    input,
                    output,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsExperimentSpanMetaVisitor)
    }
}
