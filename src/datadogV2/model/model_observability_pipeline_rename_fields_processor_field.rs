// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines how to rename a field in log events.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineRenameFieldsProcessorField {
    /// The field name to assign the renamed value to.
    #[serde(rename = "destination")]
    pub destination: String,
    /// Indicates whether the original field, that is received from the source, should be kept (`true`) or removed (`false`) after renaming.
    #[serde(rename = "preserve_source")]
    pub preserve_source: bool,
    /// The original field name in the log event that should be renamed.
    #[serde(rename = "source")]
    pub source: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineRenameFieldsProcessorField {
    pub fn new(
        destination: String,
        preserve_source: bool,
        source: String,
    ) -> ObservabilityPipelineRenameFieldsProcessorField {
        ObservabilityPipelineRenameFieldsProcessorField {
            destination,
            preserve_source,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineRenameFieldsProcessorField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineRenameFieldsProcessorFieldVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineRenameFieldsProcessorFieldVisitor {
            type Value = ObservabilityPipelineRenameFieldsProcessorField;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut destination: Option<String> = None;
                let mut preserve_source: Option<bool> = None;
                let mut source: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "destination" => {
                            destination =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "preserve_source" => {
                            preserve_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let destination =
                    destination.ok_or_else(|| M::Error::missing_field("destination"))?;
                let preserve_source =
                    preserve_source.ok_or_else(|| M::Error::missing_field("preserve_source"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;

                let content = ObservabilityPipelineRenameFieldsProcessorField {
                    destination,
                    preserve_source,
                    source,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineRenameFieldsProcessorFieldVisitor)
    }
}
