// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for a single integration's configuration schema.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetIntegrationSchemaDetailV2Attributes {
    /// The configuration file specifications for the integration. Always present, returned as an empty array when there are none.
    #[serde(rename = "files")]
    pub files: Vec<crate::datadogV2::model::FleetIntegrationSchemaFileSpecV2>,
    /// The integration folder key. Absent from the response when empty.
    #[serde(rename = "folder")]
    pub folder: Option<String>,
    /// The display name of the integration. Absent from the response when empty.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The integration version. Absent from the response when empty.
    #[serde(rename = "version")]
    pub version: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetIntegrationSchemaDetailV2Attributes {
    pub fn new(
        files: Vec<crate::datadogV2::model::FleetIntegrationSchemaFileSpecV2>,
    ) -> FleetIntegrationSchemaDetailV2Attributes {
        FleetIntegrationSchemaDetailV2Attributes {
            files,
            folder: None,
            name: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn folder(mut self, value: String) -> Self {
        self.folder = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn version(mut self, value: String) -> Self {
        self.version = Some(value);
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

impl<'de> Deserialize<'de> for FleetIntegrationSchemaDetailV2Attributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetIntegrationSchemaDetailV2AttributesVisitor;
        impl<'a> Visitor<'a> for FleetIntegrationSchemaDetailV2AttributesVisitor {
            type Value = FleetIntegrationSchemaDetailV2Attributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut files: Option<
                    Vec<crate::datadogV2::model::FleetIntegrationSchemaFileSpecV2>,
                > = None;
                let mut folder: Option<String> = None;
                let mut name: Option<String> = None;
                let mut version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "files" => {
                            files = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "folder" => {
                            if v.is_null() {
                                continue;
                            }
                            folder = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let files = files.ok_or_else(|| M::Error::missing_field("files"))?;

                let content = FleetIntegrationSchemaDetailV2Attributes {
                    files,
                    folder,
                    name,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetIntegrationSchemaDetailV2AttributesVisitor)
    }
}
