// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `dedupe` processor removes duplicate fields in log events.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineDedupeProcessor {
    /// A list of log field paths to check for duplicates.
    #[serde(rename = "fields")]
    pub fields: Vec<String>,
    /// The unique identifier for this processor.
    #[serde(rename = "id")]
    pub id: String,
    /// A Datadog search query used to determine which logs this processor targets.
    #[serde(rename = "include")]
    pub include: String,
    /// A list of component IDs whose output is used as the input for this processor.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// The deduplication mode to apply to the fields.
    #[serde(rename = "mode")]
    pub mode: crate::datadogV2::model::ObservabilityPipelineDedupeProcessorMode,
    /// The processor type. The value should always be `dedupe`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineDedupeProcessorType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineDedupeProcessor {
    pub fn new(
        fields: Vec<String>,
        id: String,
        include: String,
        inputs: Vec<String>,
        mode: crate::datadogV2::model::ObservabilityPipelineDedupeProcessorMode,
        type_: crate::datadogV2::model::ObservabilityPipelineDedupeProcessorType,
    ) -> ObservabilityPipelineDedupeProcessor {
        ObservabilityPipelineDedupeProcessor {
            fields,
            id,
            include,
            inputs,
            mode,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineDedupeProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineDedupeProcessorVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineDedupeProcessorVisitor {
            type Value = ObservabilityPipelineDedupeProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut fields: Option<Vec<String>> = None;
                let mut id: Option<String> = None;
                let mut include: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut mode: Option<
                    crate::datadogV2::model::ObservabilityPipelineDedupeProcessorMode,
                > = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineDedupeProcessorType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "fields" => {
                            fields = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include" => {
                            include = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputs" => {
                            inputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mode" => {
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mode) = mode {
                                match _mode {
                                    crate::datadogV2::model::ObservabilityPipelineDedupeProcessorMode::UnparsedObject(_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineDedupeProcessorType::UnparsedObject(_type_) => {
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
                let fields = fields.ok_or_else(|| M::Error::missing_field("fields"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let include = include.ok_or_else(|| M::Error::missing_field("include"))?;
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;
                let mode = mode.ok_or_else(|| M::Error::missing_field("mode"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineDedupeProcessor {
                    fields,
                    id,
                    include,
                    inputs,
                    mode,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineDedupeProcessorVisitor)
    }
}
