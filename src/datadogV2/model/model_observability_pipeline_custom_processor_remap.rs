// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines a single VRL remap rule with its own filtering and transformation logic.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineCustomProcessorRemap {
    /// Whether to drop events that caused errors during processing.
    #[serde(rename = "drop_on_error")]
    pub drop_on_error: bool,
    /// Whether this remap rule is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// A Datadog search query used to filter events for this specific remap rule.
    #[serde(rename = "include")]
    pub include: String,
    /// A descriptive name for this remap rule.
    #[serde(rename = "name")]
    pub name: String,
    /// The VRL script source code that defines the processing logic.
    #[serde(rename = "source")]
    pub source: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineCustomProcessorRemap {
    pub fn new(
        drop_on_error: bool,
        enabled: bool,
        include: String,
        name: String,
        source: String,
    ) -> ObservabilityPipelineCustomProcessorRemap {
        ObservabilityPipelineCustomProcessorRemap {
            drop_on_error,
            enabled,
            include,
            name,
            source,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineCustomProcessorRemap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineCustomProcessorRemapVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineCustomProcessorRemapVisitor {
            type Value = ObservabilityPipelineCustomProcessorRemap;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut drop_on_error: Option<bool> = None;
                let mut enabled: Option<bool> = None;
                let mut include: Option<String> = None;
                let mut name: Option<String> = None;
                let mut source: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "drop_on_error" => {
                            drop_on_error =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include" => {
                            include = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let drop_on_error =
                    drop_on_error.ok_or_else(|| M::Error::missing_field("drop_on_error"))?;
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let include = include.ok_or_else(|| M::Error::missing_field("include"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;

                let content = ObservabilityPipelineCustomProcessorRemap {
                    drop_on_error,
                    enabled,
                    include,
                    name,
                    source,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineCustomProcessorRemapVisitor)
    }
}
