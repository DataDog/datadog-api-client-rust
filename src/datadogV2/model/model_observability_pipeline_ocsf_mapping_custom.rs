// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Custom OCSF mapping configuration for transforming logs.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineOcsfMappingCustom {
    /// A list of field mapping rules for transforming log fields to OCSF schema fields.
    #[serde(rename = "mapping")]
    pub mapping: Vec<crate::datadogV2::model::ObservabilityPipelineOcsfMappingCustomFieldMapping>,
    /// Metadata for the custom OCSF mapping.
    #[serde(rename = "metadata")]
    pub metadata: crate::datadogV2::model::ObservabilityPipelineOcsfMappingCustomMetadata,
    /// The version of the custom mapping configuration.
    #[serde(rename = "version")]
    pub version: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineOcsfMappingCustom {
    pub fn new(
        mapping: Vec<crate::datadogV2::model::ObservabilityPipelineOcsfMappingCustomFieldMapping>,
        metadata: crate::datadogV2::model::ObservabilityPipelineOcsfMappingCustomMetadata,
        version: i64,
    ) -> ObservabilityPipelineOcsfMappingCustom {
        ObservabilityPipelineOcsfMappingCustom {
            mapping,
            metadata,
            version,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineOcsfMappingCustom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineOcsfMappingCustomVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineOcsfMappingCustomVisitor {
            type Value = ObservabilityPipelineOcsfMappingCustom;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut mapping: Option<
                    Vec<
                        crate::datadogV2::model::ObservabilityPipelineOcsfMappingCustomFieldMapping,
                    >,
                > = None;
                let mut metadata: Option<
                    crate::datadogV2::model::ObservabilityPipelineOcsfMappingCustomMetadata,
                > = None;
                let mut version: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "mapping" => {
                            mapping = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let mapping = mapping.ok_or_else(|| M::Error::missing_field("mapping"))?;
                let metadata = metadata.ok_or_else(|| M::Error::missing_field("metadata"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = ObservabilityPipelineOcsfMappingCustom {
                    mapping,
                    metadata,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineOcsfMappingCustomVisitor)
    }
}
