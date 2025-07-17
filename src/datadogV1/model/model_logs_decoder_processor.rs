// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The decoder processor decodes any source attribute containing a
/// base64/base16-encoded UTF-8/ASCII string back to its original value, storing the
/// result in a target attribute.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsDecoderProcessor {
    /// The encoding used to represent the binary data.
    #[serde(rename = "binary_to_text_encoding")]
    pub binary_to_text_encoding: crate::datadogV1::model::LogsDecoderProcessorBinaryToTextEncoding,
    /// The original representation of input string.
    #[serde(rename = "input_representation")]
    pub input_representation: crate::datadogV1::model::LogsDecoderProcessorInputRepresentation,
    /// Whether the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Name of the log attribute with the encoded data.
    #[serde(rename = "source")]
    pub source: String,
    /// Name of the log attribute that contains the decoded data.
    #[serde(rename = "target")]
    pub target: String,
    /// Type of logs decoder processor.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsDecoderProcessorType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsDecoderProcessor {
    pub fn new(
        binary_to_text_encoding: crate::datadogV1::model::LogsDecoderProcessorBinaryToTextEncoding,
        input_representation: crate::datadogV1::model::LogsDecoderProcessorInputRepresentation,
        source: String,
        target: String,
        type_: crate::datadogV1::model::LogsDecoderProcessorType,
    ) -> LogsDecoderProcessor {
        LogsDecoderProcessor {
            binary_to_text_encoding,
            input_representation,
            is_enabled: None,
            name: None,
            source,
            target,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
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

impl<'de> Deserialize<'de> for LogsDecoderProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsDecoderProcessorVisitor;
        impl<'a> Visitor<'a> for LogsDecoderProcessorVisitor {
            type Value = LogsDecoderProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut binary_to_text_encoding: Option<
                    crate::datadogV1::model::LogsDecoderProcessorBinaryToTextEncoding,
                > = None;
                let mut input_representation: Option<
                    crate::datadogV1::model::LogsDecoderProcessorInputRepresentation,
                > = None;
                let mut is_enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut source: Option<String> = None;
                let mut target: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::LogsDecoderProcessorType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "binary_to_text_encoding" => {
                            binary_to_text_encoding =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _binary_to_text_encoding) = binary_to_text_encoding {
                                match _binary_to_text_encoding {
                                    crate::datadogV1::model::LogsDecoderProcessorBinaryToTextEncoding::UnparsedObject(_binary_to_text_encoding) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "input_representation" => {
                            input_representation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _input_representation) = input_representation {
                                match _input_representation {
                                    crate::datadogV1::model::LogsDecoderProcessorInputRepresentation::UnparsedObject(_input_representation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "is_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::LogsDecoderProcessorType::UnparsedObject(_type_) => {
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
                let binary_to_text_encoding = binary_to_text_encoding
                    .ok_or_else(|| M::Error::missing_field("binary_to_text_encoding"))?;
                let input_representation = input_representation
                    .ok_or_else(|| M::Error::missing_field("input_representation"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LogsDecoderProcessor {
                    binary_to_text_encoding,
                    input_representation,
                    is_enabled,
                    name,
                    source,
                    target,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsDecoderProcessorVisitor)
    }
}
