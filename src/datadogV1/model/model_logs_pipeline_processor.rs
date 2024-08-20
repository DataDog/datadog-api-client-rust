// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Nested Pipelines are pipelines within a pipeline. Use Nested Pipelines to split the processing into two steps.
/// For example, first use a high-level filtering such as team and then a second level of filtering based on the
/// integration, service, or any other tag or attribute.
///
/// A pipeline can contain Nested Pipelines and Processors whereas a Nested Pipeline can only contain Processors.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsPipelineProcessor {
    /// Filter for logs.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV1::model::LogsFilter>,
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Ordered list of processors in this pipeline.
    #[serde(rename = "processors")]
    pub processors: Option<Vec<crate::datadogV1::model::LogsProcessor>>,
    /// Type of logs pipeline processor.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsPipelineProcessorType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsPipelineProcessor {
    pub fn new(type_: crate::datadogV1::model::LogsPipelineProcessorType) -> LogsPipelineProcessor {
        LogsPipelineProcessor {
            filter: None,
            is_enabled: None,
            name: None,
            processors: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn filter(mut self, value: crate::datadogV1::model::LogsFilter) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn processors(mut self, value: Vec<crate::datadogV1::model::LogsProcessor>) -> Self {
        self.processors = Some(value);
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

impl<'de> Deserialize<'de> for LogsPipelineProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsPipelineProcessorVisitor;
        impl<'a> Visitor<'a> for LogsPipelineProcessorVisitor {
            type Value = LogsPipelineProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filter: Option<crate::datadogV1::model::LogsFilter> = None;
                let mut is_enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut processors: Option<Vec<crate::datadogV1::model::LogsProcessor>> = None;
                let mut type_: Option<crate::datadogV1::model::LogsPipelineProcessorType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "processors" => {
                            if v.is_null() {
                                continue;
                            }
                            processors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::LogsPipelineProcessorType::UnparsedObject(_type_) => {
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
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = LogsPipelineProcessor {
                    filter,
                    is_enabled,
                    name,
                    processors,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsPipelineProcessorVisitor)
    }
}
