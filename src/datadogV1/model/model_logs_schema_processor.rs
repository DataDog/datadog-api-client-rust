// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A processor that has additional validations and checks for a given schema. Currently supported schema types include OCSF.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsSchemaProcessor {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// The `LogsSchemaProcessor` `mappers`.
    #[serde(rename = "mappers")]
    pub mappers: Vec<crate::datadogV1::model::LogsSchemaMapper>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: String,
    /// Configuration of the schema data to use.
    #[serde(rename = "schema")]
    pub schema: crate::datadogV1::model::LogsSchemaData,
    /// Type of logs schema processor.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsSchemaProcessorType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsSchemaProcessor {
    pub fn new(
        mappers: Vec<crate::datadogV1::model::LogsSchemaMapper>,
        name: String,
        schema: crate::datadogV1::model::LogsSchemaData,
        type_: crate::datadogV1::model::LogsSchemaProcessorType,
    ) -> LogsSchemaProcessor {
        LogsSchemaProcessor {
            is_enabled: None,
            mappers,
            name,
            schema,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
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

impl<'de> Deserialize<'de> for LogsSchemaProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsSchemaProcessorVisitor;
        impl<'a> Visitor<'a> for LogsSchemaProcessorVisitor {
            type Value = LogsSchemaProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_enabled: Option<bool> = None;
                let mut mappers: Option<Vec<crate::datadogV1::model::LogsSchemaMapper>> = None;
                let mut name: Option<String> = None;
                let mut schema: Option<crate::datadogV1::model::LogsSchemaData> = None;
                let mut type_: Option<crate::datadogV1::model::LogsSchemaProcessorType> = None;
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
                        "mappers" => {
                            mappers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "schema" => {
                            schema = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::LogsSchemaProcessorType::UnparsedObject(_type_) => {
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
                let mappers = mappers.ok_or_else(|| M::Error::missing_field("mappers"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let schema = schema.ok_or_else(|| M::Error::missing_field("schema"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LogsSchemaProcessor {
                    is_enabled,
                    mappers,
                    name,
                    schema,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsSchemaProcessorVisitor)
    }
}
