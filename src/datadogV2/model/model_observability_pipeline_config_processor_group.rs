// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A group of processors.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineConfigProcessorGroup {
    /// Whether this processor group is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The unique identifier for the processor group.
    #[serde(rename = "id")]
    pub id: String,
    /// Conditional expression for when this processor group should execute.
    #[serde(rename = "include")]
    pub include: String,
    /// A list of IDs for components whose output is used as the input for this processor group.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// Processors applied sequentially within this group. Events flow through each processor in order.
    #[serde(rename = "processors")]
    pub processors: Vec<crate::datadogV2::model::ObservabilityPipelineConfigProcessorItem>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineConfigProcessorGroup {
    pub fn new(
        enabled: bool,
        id: String,
        include: String,
        inputs: Vec<String>,
        processors: Vec<crate::datadogV2::model::ObservabilityPipelineConfigProcessorItem>,
    ) -> ObservabilityPipelineConfigProcessorGroup {
        ObservabilityPipelineConfigProcessorGroup {
            enabled,
            id,
            include,
            inputs,
            processors,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineConfigProcessorGroup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineConfigProcessorGroupVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineConfigProcessorGroupVisitor {
            type Value = ObservabilityPipelineConfigProcessorGroup;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut id: Option<String> = None;
                let mut include: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut processors: Option<
                    Vec<crate::datadogV2::model::ObservabilityPipelineConfigProcessorItem>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "processors" => {
                            processors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let include = include.ok_or_else(|| M::Error::missing_field("include"))?;
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;
                let processors = processors.ok_or_else(|| M::Error::missing_field("processors"))?;

                let content = ObservabilityPipelineConfigProcessorGroup {
                    enabled,
                    id,
                    include,
                    inputs,
                    processors,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineConfigProcessorGroupVisitor)
    }
}
