// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Specifies the pipeline's configuration, including its sources, processors, and destinations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PipelineConfig {
    /// A list of destination components where processed logs are sent.
    #[serde(rename = "destinations")]
    pub destinations: Vec<crate::datadogV2::model::PipelineConfigDestination>,
    /// A list of processors that transform or enrich log data.
    #[serde(rename = "processors")]
    pub processors: Vec<crate::datadogV2::model::PipelineConfigProcessor>,
    /// A list of configured data sources for the pipeline.
    #[serde(rename = "sources")]
    pub sources: Vec<crate::datadogV2::model::PipelineConfigSource>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PipelineConfig {
    pub fn new(
        destinations: Vec<crate::datadogV2::model::PipelineConfigDestination>,
        processors: Vec<crate::datadogV2::model::PipelineConfigProcessor>,
        sources: Vec<crate::datadogV2::model::PipelineConfigSource>,
    ) -> PipelineConfig {
        PipelineConfig {
            destinations,
            processors,
            sources,
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

impl<'de> Deserialize<'de> for PipelineConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PipelineConfigVisitor;
        impl<'a> Visitor<'a> for PipelineConfigVisitor {
            type Value = PipelineConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut destinations: Option<
                    Vec<crate::datadogV2::model::PipelineConfigDestination>,
                > = None;
                let mut processors: Option<Vec<crate::datadogV2::model::PipelineConfigProcessor>> =
                    None;
                let mut sources: Option<Vec<crate::datadogV2::model::PipelineConfigSource>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "destinations" => {
                            destinations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "processors" => {
                            processors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sources" => {
                            sources = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let destinations =
                    destinations.ok_or_else(|| M::Error::missing_field("destinations"))?;
                let processors = processors.ok_or_else(|| M::Error::missing_field("processors"))?;
                let sources = sources.ok_or_else(|| M::Error::missing_field("sources"))?;

                let content = PipelineConfig {
                    destinations,
                    processors,
                    sources,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PipelineConfigVisitor)
    }
}
