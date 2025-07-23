// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `custom_processor` processor transforms events using [Vector Remap Language (VRL)](<https://vector.dev/docs/reference/vrl/>) scripts with advanced filtering capabilities.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineCustomProcessorProcessor {
    /// The unique identifier for this processor.
    #[serde(rename = "id")]
    pub id: String,
    /// A Datadog search query used to determine which logs this processor targets. This field should always be set to `*` for the custom_processor processor.
    #[serde(rename = "include")]
    pub include: String,
    /// A list of component IDs whose output is used as the input for this processor.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// Array of VRL remap rules.
    #[serde(rename = "remaps")]
    pub remaps: Vec<crate::datadogV2::model::ObservabilityPipelineCustomProcessorProcessorRemap>,
    /// The processor type. The value should always be `custom_processor`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineCustomProcessorProcessorType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineCustomProcessorProcessor {
    pub fn new(
        id: String,
        include: String,
        inputs: Vec<String>,
        remaps: Vec<crate::datadogV2::model::ObservabilityPipelineCustomProcessorProcessorRemap>,
        type_: crate::datadogV2::model::ObservabilityPipelineCustomProcessorProcessorType,
    ) -> ObservabilityPipelineCustomProcessorProcessor {
        ObservabilityPipelineCustomProcessorProcessor {
            id,
            include,
            inputs,
            remaps,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineCustomProcessorProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineCustomProcessorProcessorVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineCustomProcessorProcessorVisitor {
            type Value = ObservabilityPipelineCustomProcessorProcessor;

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
                let mut remaps: Option<
                    Vec<
                        crate::datadogV2::model::ObservabilityPipelineCustomProcessorProcessorRemap,
                    >,
                > = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineCustomProcessorProcessorType,
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
                        "remaps" => {
                            remaps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineCustomProcessorProcessorType::UnparsedObject(_type_) => {
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
                let remaps = remaps.ok_or_else(|| M::Error::missing_field("remaps"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineCustomProcessorProcessor {
                    id,
                    include,
                    inputs,
                    remaps,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineCustomProcessorProcessorVisitor)
    }
}
