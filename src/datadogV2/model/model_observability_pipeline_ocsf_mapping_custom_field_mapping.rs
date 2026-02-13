// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines a single field mapping rule for transforming a source field to an OCSF destination field.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineOcsfMappingCustomFieldMapping {
    /// The default value to use if the source field is missing or empty.
    #[serde(rename = "default")]
    pub default: Option<serde_json::Value>,
    /// The destination OCSF field path.
    #[serde(rename = "dest")]
    pub dest: String,
    /// Lookup table configuration for mapping source values to destination values.
    #[serde(rename = "lookup")]
    pub lookup: Option<crate::datadogV2::model::ObservabilityPipelineOcsfMappingCustomLookup>,
    /// The source field path from the log event.
    #[serde(rename = "source")]
    pub source: Option<serde_json::Value>,
    /// Multiple source field paths for combined mapping.
    #[serde(rename = "sources")]
    pub sources: Option<serde_json::Value>,
    /// A static value to use for the destination field.
    #[serde(rename = "value")]
    pub value: Option<serde_json::Value>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineOcsfMappingCustomFieldMapping {
    pub fn new(dest: String) -> ObservabilityPipelineOcsfMappingCustomFieldMapping {
        ObservabilityPipelineOcsfMappingCustomFieldMapping {
            default: None,
            dest,
            lookup: None,
            source: None,
            sources: None,
            value: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn default(mut self, value: serde_json::Value) -> Self {
        self.default = Some(value);
        self
    }

    pub fn lookup(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineOcsfMappingCustomLookup,
    ) -> Self {
        self.lookup = Some(value);
        self
    }

    pub fn source(mut self, value: serde_json::Value) -> Self {
        self.source = Some(value);
        self
    }

    pub fn sources(mut self, value: serde_json::Value) -> Self {
        self.sources = Some(value);
        self
    }

    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineOcsfMappingCustomFieldMapping {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineOcsfMappingCustomFieldMappingVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineOcsfMappingCustomFieldMappingVisitor {
            type Value = ObservabilityPipelineOcsfMappingCustomFieldMapping;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut default: Option<serde_json::Value> = None;
                let mut dest: Option<String> = None;
                let mut lookup: Option<
                    crate::datadogV2::model::ObservabilityPipelineOcsfMappingCustomLookup,
                > = None;
                let mut source: Option<serde_json::Value> = None;
                let mut sources: Option<serde_json::Value> = None;
                let mut value: Option<serde_json::Value> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "default" => {
                            if v.is_null() {
                                continue;
                            }
                            default = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dest" => {
                            dest = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lookup" => {
                            if v.is_null() {
                                continue;
                            }
                            lookup = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            if v.is_null() {
                                continue;
                            }
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sources" => {
                            if v.is_null() {
                                continue;
                            }
                            sources = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let dest = dest.ok_or_else(|| M::Error::missing_field("dest"))?;

                let content = ObservabilityPipelineOcsfMappingCustomFieldMapping {
                    default,
                    dest,
                    lookup,
                    source,
                    sources,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineOcsfMappingCustomFieldMappingVisitor)
    }
}
