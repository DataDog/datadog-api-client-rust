// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An arithmetic sub-processor for use inside an array-map processor.
/// Unlike the top-level arithmetic processor, `is_enabled` is not supported.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsArrayMapArithmeticSubProcessor {
    /// Arithmetic operation to perform.
    #[serde(rename = "expression")]
    pub expression: String,
    /// Replace missing attribute values with 0.
    #[serde(rename = "is_replace_missing")]
    pub is_replace_missing: Option<bool>,
    /// Name of the sub-processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Target attribute path for the result.
    #[serde(rename = "target")]
    pub target: String,
    /// Type of logs arithmetic processor.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsArithmeticProcessorType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsArrayMapArithmeticSubProcessor {
    pub fn new(
        expression: String,
        target: String,
        type_: crate::datadogV1::model::LogsArithmeticProcessorType,
    ) -> LogsArrayMapArithmeticSubProcessor {
        LogsArrayMapArithmeticSubProcessor {
            expression,
            is_replace_missing: None,
            name: None,
            target,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn is_replace_missing(mut self, value: bool) -> Self {
        self.is_replace_missing = Some(value);
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

impl<'de> Deserialize<'de> for LogsArrayMapArithmeticSubProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsArrayMapArithmeticSubProcessorVisitor;
        impl<'a> Visitor<'a> for LogsArrayMapArithmeticSubProcessorVisitor {
            type Value = LogsArrayMapArithmeticSubProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut expression: Option<String> = None;
                let mut is_replace_missing: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut target: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::LogsArithmeticProcessorType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "expression" => {
                            expression = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_replace_missing" => {
                            if v.is_null() {
                                continue;
                            }
                            is_replace_missing =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::LogsArithmeticProcessorType::UnparsedObject(_type_) => {
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
                let expression = expression.ok_or_else(|| M::Error::missing_field("expression"))?;
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LogsArrayMapArithmeticSubProcessor {
                    expression,
                    is_replace_missing,
                    name,
                    target,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsArrayMapArithmeticSubProcessorVisitor)
    }
}
