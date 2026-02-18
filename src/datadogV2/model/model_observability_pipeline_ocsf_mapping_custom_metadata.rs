// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata for the custom OCSF mapping.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineOcsfMappingCustomMetadata {
    /// The OCSF event class name.
    #[serde(rename = "class")]
    pub class: String,
    /// A list of OCSF profiles to apply.
    #[serde(rename = "profiles")]
    pub profiles: Option<Vec<String>>,
    /// The OCSF schema version.
    #[serde(rename = "version")]
    pub version: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineOcsfMappingCustomMetadata {
    pub fn new(class: String, version: String) -> ObservabilityPipelineOcsfMappingCustomMetadata {
        ObservabilityPipelineOcsfMappingCustomMetadata {
            class,
            profiles: None,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineOcsfMappingCustomMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineOcsfMappingCustomMetadataVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineOcsfMappingCustomMetadataVisitor {
            type Value = ObservabilityPipelineOcsfMappingCustomMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut class: Option<String> = None;
                let mut profiles: Option<Vec<String>> = None;
                let mut version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "class" => {
                            class = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "profiles" => {
                            if v.is_null() {
                                continue;
                            }
                            profiles = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let class = class.ok_or_else(|| M::Error::missing_field("class"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = ObservabilityPipelineOcsfMappingCustomMetadata {
                    class,
                    profiles,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineOcsfMappingCustomMetadataVisitor)
    }
}
