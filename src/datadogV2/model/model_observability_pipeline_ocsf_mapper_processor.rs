// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `ocsf_mapper` processor transforms logs into the OCSF schema using a predefined mapping configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineOcsfMapperProcessor {
    /// The unique identifier for this component. Used to reference this component in other parts of the pipeline.
    #[serde(rename = "id")]
    pub id: String,
    /// A Datadog search query used to determine which logs this processor targets.
    #[serde(rename = "include")]
    pub include: String,
    /// A list of component IDs whose output is used as the `input` for this processor.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// A list of mapping rules to convert events to the OCSF format.
    #[serde(rename = "mappings")]
    pub mappings: Vec<crate::datadogV2::model::ObservabilityPipelineOcsfMapperProcessorMapping>,
    /// The processor type. The value should always be `ocsf_mapper`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineOcsfMapperProcessorType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineOcsfMapperProcessor {
    pub fn new(
        id: String,
        include: String,
        inputs: Vec<String>,
        mappings: Vec<crate::datadogV2::model::ObservabilityPipelineOcsfMapperProcessorMapping>,
        type_: crate::datadogV2::model::ObservabilityPipelineOcsfMapperProcessorType,
    ) -> ObservabilityPipelineOcsfMapperProcessor {
        ObservabilityPipelineOcsfMapperProcessor {
            id,
            include,
            inputs,
            mappings,
            type_,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineOcsfMapperProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineOcsfMapperProcessorVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineOcsfMapperProcessorVisitor {
            type Value = ObservabilityPipelineOcsfMapperProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut include: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut mappings: Option<
                    Vec<crate::datadogV2::model::ObservabilityPipelineOcsfMapperProcessorMapping>,
                > = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineOcsfMapperProcessorType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include" => {
                            include = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputs" => {
                            inputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mappings" => {
                            mappings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineOcsfMapperProcessorType::UnparsedObject(_type_) => {
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
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let include = include.ok_or_else(|| M::Error::missing_field("include"))?;
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;
                let mappings = mappings.ok_or_else(|| M::Error::missing_field("mappings"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineOcsfMapperProcessor {
                    id,
                    include,
                    inputs,
                    mappings,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineOcsfMapperProcessorVisitor)
    }
}
