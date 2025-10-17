// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration of the schema data to use.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsSchemaData {
    /// Class name of the schema to use.
    #[serde(rename = "class_name")]
    pub class_name: String,
    /// Class UID of the schema to use.
    #[serde(rename = "class_uid")]
    pub class_uid: i64,
    /// Optional list of profiles to modify the schema.
    #[serde(rename = "profiles")]
    pub profiles: Option<Vec<String>>,
    /// Type of schema to use.
    #[serde(rename = "schema_type")]
    pub schema_type: String,
    /// Version of the schema to use.
    #[serde(rename = "version")]
    pub version: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsSchemaData {
    pub fn new(
        class_name: String,
        class_uid: i64,
        schema_type: String,
        version: String,
    ) -> LogsSchemaData {
        LogsSchemaData {
            class_name,
            class_uid,
            profiles: None,
            schema_type,
            version,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn profiles(mut self, value: Vec<String>) -> Self {
        self.profiles = Some(value);
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

impl<'de> Deserialize<'de> for LogsSchemaData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsSchemaDataVisitor;
        impl<'a> Visitor<'a> for LogsSchemaDataVisitor {
            type Value = LogsSchemaData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut class_name: Option<String> = None;
                let mut class_uid: Option<i64> = None;
                let mut profiles: Option<Vec<String>> = None;
                let mut schema_type: Option<String> = None;
                let mut version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "class_name" => {
                            class_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "class_uid" => {
                            class_uid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "profiles" => {
                            if v.is_null() {
                                continue;
                            }
                            profiles = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "schema_type" => {
                            schema_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let class_name = class_name.ok_or_else(|| M::Error::missing_field("class_name"))?;
                let class_uid = class_uid.ok_or_else(|| M::Error::missing_field("class_uid"))?;
                let schema_type =
                    schema_type.ok_or_else(|| M::Error::missing_field("schema_type"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = LogsSchemaData {
                    class_name,
                    class_uid,
                    profiles,
                    schema_type,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsSchemaDataVisitor)
    }
}
