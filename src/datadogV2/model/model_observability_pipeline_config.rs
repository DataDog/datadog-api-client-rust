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
pub struct ObservabilityPipelineConfig {
    /// A list of destination components where processed logs are sent.
    #[serde(rename = "destinations")]
    pub destinations: Vec<crate::datadogV2::model::ObservabilityPipelineConfigDestinationItem>,
    /// A list of processors that transform or enrich log data.
    #[serde(rename = "processors")]
    pub processors: Vec<crate::datadogV2::model::ObservabilityPipelineConfigProcessorItem>,
    /// A list of configured data sources for the pipeline.
    #[serde(rename = "sources")]
    pub sources: Vec<crate::datadogV2::model::ObservabilityPipelineConfigSourceItem>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineConfig {
    pub fn new(
        destinations: Vec<crate::datadogV2::model::ObservabilityPipelineConfigDestinationItem>,
        processors: Vec<crate::datadogV2::model::ObservabilityPipelineConfigProcessorItem>,
        sources: Vec<crate::datadogV2::model::ObservabilityPipelineConfigSourceItem>,
    ) -> ObservabilityPipelineConfig {
        ObservabilityPipelineConfig {
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

impl<'de> Deserialize<'de> for ObservabilityPipelineConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineConfigVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineConfigVisitor {
            type Value = ObservabilityPipelineConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut destinations: Option<
                    Vec<crate::datadogV2::model::ObservabilityPipelineConfigDestinationItem>,
                > = None;
                let mut processors: Option<
                    Vec<crate::datadogV2::model::ObservabilityPipelineConfigProcessorItem>,
                > = None;
                let mut sources: Option<
                    Vec<crate::datadogV2::model::ObservabilityPipelineConfigSourceItem>,
                > = None;
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

                let content = ObservabilityPipelineConfig {
                    destinations,
                    processors,
                    sources,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineConfigVisitor)
    }
}
