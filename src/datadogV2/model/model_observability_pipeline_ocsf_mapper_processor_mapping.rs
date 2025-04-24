// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines how specific events are transformed to OCSF using a mapping configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineOcsfMapperProcessorMapping {
    /// A Datadog search query used to select the logs that this mapping should apply to.
    #[serde(rename = "include")]
    pub include: String,
    /// The definition of `ObservabilityPipelineOcsfMapperProcessorMappingMapping` object.
    #[serde(rename = "mapping")]
    pub mapping: crate::datadogV2::model::ObservabilityPipelineOcsfMapperProcessorMappingMapping,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineOcsfMapperProcessorMapping {
    pub fn new(
        include: String,
        mapping: crate::datadogV2::model::ObservabilityPipelineOcsfMapperProcessorMappingMapping,
    ) -> ObservabilityPipelineOcsfMapperProcessorMapping {
        ObservabilityPipelineOcsfMapperProcessorMapping {
            include,
            mapping,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ObservabilityPipelineOcsfMapperProcessorMapping {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineOcsfMapperProcessorMappingVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineOcsfMapperProcessorMappingVisitor {
            type Value = ObservabilityPipelineOcsfMapperProcessorMapping;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut include: Option<String> = None;
                let mut mapping: Option<
                    crate::datadogV2::model::ObservabilityPipelineOcsfMapperProcessorMappingMapping,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "include" => {
                            include = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mapping" => {
                            mapping = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mapping) = mapping {
                                match _mapping {
                                    crate::datadogV2::model::ObservabilityPipelineOcsfMapperProcessorMappingMapping::UnparsedObject(_mapping) => {
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
                let include = include.ok_or_else(|| M::Error::missing_field("include"))?;
                let mapping = mapping.ok_or_else(|| M::Error::missing_field("mapping"))?;

                let content = ObservabilityPipelineOcsfMapperProcessorMapping {
                    include,
                    mapping,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineOcsfMapperProcessorMappingVisitor)
    }
}
