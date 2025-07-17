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
pub struct ObservabilityPipelineCustomProcessorProcessorRemap {
    /// Whether to drop events that caused errors during processing.
    #[serde(rename = "drop_on_error")]
    pub drop_on_error: Option<bool>,
    /// Whether this remap rule is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
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

impl ObservabilityPipelineCustomProcessorProcessorRemap {
    pub fn new(
        include: String,
        name: String,
        source: String,
    ) -> ObservabilityPipelineCustomProcessorProcessorRemap {
        ObservabilityPipelineCustomProcessorProcessorRemap {
            drop_on_error: None,
            enabled: None,
            include,
            name,
            source,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn drop_on_error(mut self, value: bool) -> Self {
        self.drop_on_error = Some(value);
        self
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

impl<'de> Deserialize<'de> for ObservabilityPipelineCustomProcessorProcessorRemap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineCustomProcessorProcessorRemapVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineCustomProcessorProcessorRemapVisitor {
            type Value = ObservabilityPipelineCustomProcessorProcessorRemap;

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
                            if v.is_null() {
                                continue;
                            }
                            drop_on_error =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
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
                let include = include.ok_or_else(|| M::Error::missing_field("include"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;

                let content = ObservabilityPipelineCustomProcessorProcessorRemap {
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

        deserializer.deserialize_any(ObservabilityPipelineCustomProcessorProcessorRemapVisitor)
    }
}
