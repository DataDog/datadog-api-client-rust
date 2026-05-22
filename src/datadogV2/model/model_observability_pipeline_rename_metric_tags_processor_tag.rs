// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines how to rename a tag on metric events.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineRenameMetricTagsProcessorTag {
    /// The new tag key to assign in place of the original.
    #[serde(rename = "rename_to")]
    pub rename_to: String,
    /// The original tag key on the metric event.
    #[serde(rename = "tag")]
    pub tag: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineRenameMetricTagsProcessorTag {
    pub fn new(
        rename_to: String,
        tag: String,
    ) -> ObservabilityPipelineRenameMetricTagsProcessorTag {
        ObservabilityPipelineRenameMetricTagsProcessorTag {
            rename_to,
            tag,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineRenameMetricTagsProcessorTag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineRenameMetricTagsProcessorTagVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineRenameMetricTagsProcessorTagVisitor {
            type Value = ObservabilityPipelineRenameMetricTagsProcessorTag;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut rename_to: Option<String> = None;
                let mut tag: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "rename_to" => {
                            rename_to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag" => {
                            tag = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let rename_to = rename_to.ok_or_else(|| M::Error::missing_field("rename_to"))?;
                let tag = tag.ok_or_else(|| M::Error::missing_field("tag"))?;

                let content = ObservabilityPipelineRenameMetricTagsProcessorTag {
                    rename_to,
                    tag,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineRenameMetricTagsProcessorTagVisitor)
    }
}
