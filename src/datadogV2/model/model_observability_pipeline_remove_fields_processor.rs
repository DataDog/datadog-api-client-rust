// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `remove_fields` processor deletes specified fields from logs.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineRemoveFieldsProcessor {
    /// The processor passes through all events if it is set to `false`. Defaults to `true`.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// A list of field names to be removed from each log event.
    #[serde(rename = "fields")]
    pub fields: Vec<String>,
    /// The unique identifier for this component. Used to reference this component in other parts of the pipeline (e.g., as input to downstream components).
    #[serde(rename = "id")]
    pub id: String,
    /// A Datadog search query used to determine which logs this processor targets.
    #[serde(rename = "include")]
    pub include: String,
    /// The `PipelineRemoveFieldsProcessor` `inputs`.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// The processor type. The value should always be `remove_fields`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineRemoveFieldsProcessorType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineRemoveFieldsProcessor {
    pub fn new(
        fields: Vec<String>,
        id: String,
        include: String,
        inputs: Vec<String>,
        type_: crate::datadogV2::model::ObservabilityPipelineRemoveFieldsProcessorType,
    ) -> ObservabilityPipelineRemoveFieldsProcessor {
        ObservabilityPipelineRemoveFieldsProcessor {
            enabled: None,
            fields,
            id,
            include,
            inputs,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineRemoveFieldsProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineRemoveFieldsProcessorVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineRemoveFieldsProcessorVisitor {
            type Value = ObservabilityPipelineRemoveFieldsProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut fields: Option<Vec<String>> = None;
                let mut id: Option<String> = None;
                let mut include: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineRemoveFieldsProcessorType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineRemoveFieldsProcessorType::UnparsedObject(_type_) => {
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
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineRemoveFieldsProcessor {
                    enabled,
                    fields,
                    id,
                    include,
                    inputs,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineRemoveFieldsProcessorVisitor)
    }
}
