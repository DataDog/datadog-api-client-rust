// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines a minimal threshold that applies to a specific partition instead of the processor-wide `minimal_threshold`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineSpikeProtectionProcessorThresholdOverride {
    /// The set of field matchers that identifies the partition this override applies to. Every field name listed must also appear in `partition_fields`.
    #[serde(rename = "fields")]
    pub fields: Vec<crate::datadogV2::model::ObservabilityPipelineFieldValue>,
    /// The minimal threshold to apply to the matching partition.
    #[serde(rename = "minimal_threshold")]
    pub minimal_threshold:
        crate::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorMinimalThreshold,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineSpikeProtectionProcessorThresholdOverride {
    pub fn new(
        fields: Vec<crate::datadogV2::model::ObservabilityPipelineFieldValue>,
        minimal_threshold: crate::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorMinimalThreshold,
    ) -> ObservabilityPipelineSpikeProtectionProcessorThresholdOverride {
        ObservabilityPipelineSpikeProtectionProcessorThresholdOverride {
            fields,
            minimal_threshold,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineSpikeProtectionProcessorThresholdOverride {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineSpikeProtectionProcessorThresholdOverrideVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineSpikeProtectionProcessorThresholdOverrideVisitor {
            type Value = ObservabilityPipelineSpikeProtectionProcessorThresholdOverride;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut fields: Option<
                    Vec<crate::datadogV2::model::ObservabilityPipelineFieldValue>,
                > = None;
                let mut minimal_threshold: Option<crate::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorMinimalThreshold> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "fields" => {
                            fields = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "minimal_threshold" => {
                            minimal_threshold =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _minimal_threshold) = minimal_threshold {
                                match _minimal_threshold {
                                    crate::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorMinimalThreshold::UnparsedObject(_minimal_threshold) => {
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
                let fields = fields.ok_or_else(|| M::Error::missing_field("fields"))?;
                let minimal_threshold = minimal_threshold
                    .ok_or_else(|| M::Error::missing_field("minimal_threshold"))?;

                let content = ObservabilityPipelineSpikeProtectionProcessorThresholdOverride {
                    fields,
                    minimal_threshold,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ObservabilityPipelineSpikeProtectionProcessorThresholdOverrideVisitor)
    }
}
