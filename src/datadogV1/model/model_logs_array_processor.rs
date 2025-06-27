// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A processor for extracting, aggregating, or transforming values from JSON arrays within your logs.
/// Supported operations are:
/// - Select value from matching element
/// - Compute array length
/// - Append a value to an array
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsArrayProcessor {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Configuration of the array processor operation to perform.
    #[serde(rename = "operation")]
    pub operation: crate::datadogV1::model::LogsArrayProcessorOperation,
    /// Type of logs array processor.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsArrayProcessorType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsArrayProcessor {
    pub fn new(
        operation: crate::datadogV1::model::LogsArrayProcessorOperation,
        type_: crate::datadogV1::model::LogsArrayProcessorType,
    ) -> LogsArrayProcessor {
        LogsArrayProcessor {
            is_enabled: None,
            name: None,
            operation,
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

impl<'de> Deserialize<'de> for LogsArrayProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsArrayProcessorVisitor;
        impl<'a> Visitor<'a> for LogsArrayProcessorVisitor {
            type Value = LogsArrayProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut operation: Option<crate::datadogV1::model::LogsArrayProcessorOperation> =
                    None;
                let mut type_: Option<crate::datadogV1::model::LogsArrayProcessorType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                        "operation" => {
                            operation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _operation) = operation {
                                match _operation {
                                    crate::datadogV1::model::LogsArrayProcessorOperation::UnparsedObject(_operation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::LogsArrayProcessorType::UnparsedObject(_type_) => {
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
                let operation = operation.ok_or_else(|| M::Error::missing_field("operation"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LogsArrayProcessor {
                    is_enabled,
                    name,
                    operation,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsArrayProcessorVisitor)
    }
}
