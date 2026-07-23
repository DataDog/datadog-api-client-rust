// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Operation that extracts key-value pairs from a `source` array and stores the result in the `target` attribute.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsArrayProcessorOperationExtractKeyValue {
    /// Key of the attribute in each array element that holds the name to use for the extracted attribute.
    #[serde(rename = "key_to_extract")]
    pub key_to_extract: String,
    /// Whether to override the target element if it's already set.
    #[serde(rename = "override_on_conflict")]
    pub override_on_conflict: Option<bool>,
    /// Attribute path of the array to extract key-value pairs from.
    #[serde(rename = "source")]
    pub source: String,
    /// Attribute that receives the extracted key-value pairs. If not specified, the extracted attributes are added at the root level of the log.
    #[serde(rename = "target")]
    pub target: Option<String>,
    /// Operation type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsArrayProcessorOperationExtractKeyValueType,
    /// Key of the attribute in each array element that holds the value to use for the extracted attribute.
    #[serde(rename = "value_to_extract")]
    pub value_to_extract: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsArrayProcessorOperationExtractKeyValue {
    pub fn new(
        key_to_extract: String,
        source: String,
        type_: crate::datadogV1::model::LogsArrayProcessorOperationExtractKeyValueType,
        value_to_extract: String,
    ) -> LogsArrayProcessorOperationExtractKeyValue {
        LogsArrayProcessorOperationExtractKeyValue {
            key_to_extract,
            override_on_conflict: None,
            source,
            target: None,
            type_,
            value_to_extract,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn override_on_conflict(mut self, value: bool) -> Self {
        self.override_on_conflict = Some(value);
        self
    }

    pub fn target(mut self, value: String) -> Self {
        self.target = Some(value);
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

impl<'de> Deserialize<'de> for LogsArrayProcessorOperationExtractKeyValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsArrayProcessorOperationExtractKeyValueVisitor;
        impl<'a> Visitor<'a> for LogsArrayProcessorOperationExtractKeyValueVisitor {
            type Value = LogsArrayProcessorOperationExtractKeyValue;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut key_to_extract: Option<String> = None;
                let mut override_on_conflict: Option<bool> = None;
                let mut source: Option<String> = None;
                let mut target: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV1::model::LogsArrayProcessorOperationExtractKeyValueType,
                > = None;
                let mut value_to_extract: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "key_to_extract" => {
                            key_to_extract =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "override_on_conflict" => {
                            if v.is_null() {
                                continue;
                            }
                            override_on_conflict =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            if v.is_null() {
                                continue;
                            }
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::LogsArrayProcessorOperationExtractKeyValueType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "value_to_extract" => {
                            value_to_extract =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let key_to_extract =
                    key_to_extract.ok_or_else(|| M::Error::missing_field("key_to_extract"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let value_to_extract =
                    value_to_extract.ok_or_else(|| M::Error::missing_field("value_to_extract"))?;

                let content = LogsArrayProcessorOperationExtractKeyValue {
                    key_to_extract,
                    override_on_conflict,
                    source,
                    target,
                    type_,
                    value_to_extract,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsArrayProcessorOperationExtractKeyValueVisitor)
    }
}
