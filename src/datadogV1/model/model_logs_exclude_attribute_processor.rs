// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Use this processor to remove an attribute from a log during processing.
/// The processor strips the specified attribute from the log event, which is useful
/// when the attribute contains sensitive data or is no longer needed downstream.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsExcludeAttributeProcessor {
    /// Name of the log attribute to remove from the log event.
    #[serde(rename = "attribute_to_exclude")]
    pub attribute_to_exclude: String,
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Type of logs exclude attribute processor.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsExcludeAttributeProcessorType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsExcludeAttributeProcessor {
    pub fn new(
        attribute_to_exclude: String,
        type_: crate::datadogV1::model::LogsExcludeAttributeProcessorType,
    ) -> LogsExcludeAttributeProcessor {
        LogsExcludeAttributeProcessor {
            attribute_to_exclude,
            is_enabled: None,
            name: None,
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

impl<'de> Deserialize<'de> for LogsExcludeAttributeProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsExcludeAttributeProcessorVisitor;
        impl<'a> Visitor<'a> for LogsExcludeAttributeProcessorVisitor {
            type Value = LogsExcludeAttributeProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attribute_to_exclude: Option<String> = None;
                let mut is_enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::LogsExcludeAttributeProcessorType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attribute_to_exclude" => {
                            attribute_to_exclude =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::LogsExcludeAttributeProcessorType::UnparsedObject(_type_) => {
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
                let attribute_to_exclude = attribute_to_exclude
                    .ok_or_else(|| M::Error::missing_field("attribute_to_exclude"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LogsExcludeAttributeProcessor {
                    attribute_to_exclude,
                    is_enabled,
                    name,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsExcludeAttributeProcessorVisitor)
    }
}
