// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An `enrichment_table` processor that enriches metrics using a static CSV file.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineMetricEnrichmentTableFileProcessor {
    /// The display name for a component.
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    /// Indicates whether the processor is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Defines a static enrichment table loaded from a CSV file for metric enrichment.
    #[serde(rename = "file")]
    pub file: crate::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableFile,
    /// The unique identifier for this component. Used in other parts of the pipeline to reference this component
    /// (for example, as the `input` to downstream components).
    #[serde(rename = "id")]
    pub id: String,
    /// A Datadog search query used to determine which metrics this processor targets.
    #[serde(rename = "include")]
    pub include: String,
    /// The processor type. The value should always be `enrichment_table`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineEnrichmentTableProcessorType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineMetricEnrichmentTableFileProcessor {
    pub fn new(
        enabled: bool,
        file: crate::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableFile,
        id: String,
        include: String,
        type_: crate::datadogV2::model::ObservabilityPipelineEnrichmentTableProcessorType,
    ) -> ObservabilityPipelineMetricEnrichmentTableFileProcessor {
        ObservabilityPipelineMetricEnrichmentTableFileProcessor {
            display_name: None,
            enabled,
            file,
            id,
            include,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineMetricEnrichmentTableFileProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineMetricEnrichmentTableFileProcessorVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineMetricEnrichmentTableFileProcessorVisitor {
            type Value = ObservabilityPipelineMetricEnrichmentTableFileProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut display_name: Option<String> = None;
                let mut enabled: Option<bool> = None;
                let mut file: Option<
                    crate::datadogV2::model::ObservabilityPipelineMetricEnrichmentTableFile,
                > = None;
                let mut id: Option<String> = None;
                let mut include: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineEnrichmentTableProcessorType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "display_name" => {
                            if v.is_null() {
                                continue;
                            }
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "file" => {
                            file = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include" => {
                            include = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineEnrichmentTableProcessorType::UnparsedObject(_type_) => {
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
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let file = file.ok_or_else(|| M::Error::missing_field("file"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let include = include.ok_or_else(|| M::Error::missing_field("include"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineMetricEnrichmentTableFileProcessor {
                    display_name,
                    enabled,
                    file,
                    id,
                    include,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineMetricEnrichmentTableFileProcessorVisitor)
    }
}
