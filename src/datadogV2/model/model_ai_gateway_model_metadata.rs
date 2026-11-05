// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Optional, typed enrichment for a model catalog entry. Missing properties remain unknown.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayModelMetadata {
    /// Model features reported by the enrichment source. Omitted properties are unknown.
    #[serde(rename = "capabilities")]
    pub capabilities: crate::datadogV2::model::AiGatewayModelCapabilities,
    /// Human-readable model description.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Human-readable model name.
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    /// Model family identifier.
    #[serde(rename = "family")]
    pub family: Option<String>,
    /// Primary model kind. Decision models return a choice or score instead of generated text.
    #[serde(rename = "kind")]
    pub kind: Option<crate::datadogV2::model::AiGatewayModelMetadataKind>,
    /// Knowledge cutoff reported by the source.
    #[serde(rename = "knowledge_cutoff")]
    pub knowledge_cutoff: Option<String>,
    /// Date the source record was last updated.
    #[serde(rename = "last_updated")]
    pub last_updated: Option<String>,
    /// Model lifecycle state.
    #[serde(rename = "lifecycle")]
    pub lifecycle: Option<crate::datadogV2::model::AiGatewayModelMetadataLifecycle>,
    /// Token limits reported by the enrichment source.
    #[serde(rename = "limits")]
    pub limits: crate::datadogV2::model::AiGatewayModelLimits,
    /// Supported model input and output modalities.
    #[serde(rename = "modalities")]
    pub modalities: crate::datadogV2::model::AiGatewayModelModalities,
    /// Whether model weights are openly available.
    #[serde(rename = "open_weights")]
    pub open_weights: Option<bool>,
    /// Supported reasoning controls. Absence means controls are unknown or unavailable.
    #[serde(rename = "reasoning")]
    pub reasoning: Option<crate::datadogV2::model::AiGatewayModelReasoning>,
    /// Provider-specific reference prices from the enrichment source. Omitted for shared canonical-model fallback matches. These do not determine gateway charges.
    #[serde(rename = "reference_pricing")]
    pub reference_pricing: Option<Vec<crate::datadogV2::model::AiGatewayModelReferencePrice>>,
    /// Model release date.
    #[serde(rename = "release_date")]
    pub release_date: Option<String>,
    /// Provenance for model metadata enrichment.
    #[serde(rename = "source")]
    pub source: crate::datadogV2::model::AiGatewayModelMetadataSource,
    /// Completeness of the matched source record. Partial also identifies metadata from a base model for a dated snapshot or from a shared canonical-model fallback.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::AiGatewayModelMetadataStatus,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayModelMetadata {
    pub fn new(
        capabilities: crate::datadogV2::model::AiGatewayModelCapabilities,
        limits: crate::datadogV2::model::AiGatewayModelLimits,
        modalities: crate::datadogV2::model::AiGatewayModelModalities,
        source: crate::datadogV2::model::AiGatewayModelMetadataSource,
        status: crate::datadogV2::model::AiGatewayModelMetadataStatus,
    ) -> AiGatewayModelMetadata {
        AiGatewayModelMetadata {
            capabilities,
            description: None,
            display_name: None,
            family: None,
            kind: None,
            knowledge_cutoff: None,
            last_updated: None,
            lifecycle: None,
            limits,
            modalities,
            open_weights: None,
            reasoning: None,
            reference_pricing: None,
            release_date: None,
            source,
            status,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }

    pub fn family(mut self, value: String) -> Self {
        self.family = Some(value);
        self
    }

    pub fn kind(mut self, value: crate::datadogV2::model::AiGatewayModelMetadataKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn knowledge_cutoff(mut self, value: String) -> Self {
        self.knowledge_cutoff = Some(value);
        self
    }

    pub fn last_updated(mut self, value: String) -> Self {
        self.last_updated = Some(value);
        self
    }

    pub fn lifecycle(
        mut self,
        value: crate::datadogV2::model::AiGatewayModelMetadataLifecycle,
    ) -> Self {
        self.lifecycle = Some(value);
        self
    }

    pub fn open_weights(mut self, value: bool) -> Self {
        self.open_weights = Some(value);
        self
    }

    pub fn reasoning(mut self, value: crate::datadogV2::model::AiGatewayModelReasoning) -> Self {
        self.reasoning = Some(value);
        self
    }

    pub fn reference_pricing(
        mut self,
        value: Vec<crate::datadogV2::model::AiGatewayModelReferencePrice>,
    ) -> Self {
        self.reference_pricing = Some(value);
        self
    }

    pub fn release_date(mut self, value: String) -> Self {
        self.release_date = Some(value);
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

impl<'de> Deserialize<'de> for AiGatewayModelMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayModelMetadataVisitor;
        impl<'a> Visitor<'a> for AiGatewayModelMetadataVisitor {
            type Value = AiGatewayModelMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut capabilities: Option<crate::datadogV2::model::AiGatewayModelCapabilities> =
                    None;
                let mut description: Option<String> = None;
                let mut display_name: Option<String> = None;
                let mut family: Option<String> = None;
                let mut kind: Option<crate::datadogV2::model::AiGatewayModelMetadataKind> = None;
                let mut knowledge_cutoff: Option<String> = None;
                let mut last_updated: Option<String> = None;
                let mut lifecycle: Option<
                    crate::datadogV2::model::AiGatewayModelMetadataLifecycle,
                > = None;
                let mut limits: Option<crate::datadogV2::model::AiGatewayModelLimits> = None;
                let mut modalities: Option<crate::datadogV2::model::AiGatewayModelModalities> =
                    None;
                let mut open_weights: Option<bool> = None;
                let mut reasoning: Option<crate::datadogV2::model::AiGatewayModelReasoning> = None;
                let mut reference_pricing: Option<
                    Vec<crate::datadogV2::model::AiGatewayModelReferencePrice>,
                > = None;
                let mut release_date: Option<String> = None;
                let mut source: Option<crate::datadogV2::model::AiGatewayModelMetadataSource> =
                    None;
                let mut status: Option<crate::datadogV2::model::AiGatewayModelMetadataStatus> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "capabilities" => {
                            capabilities =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_name" => {
                            if v.is_null() {
                                continue;
                            }
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "family" => {
                            if v.is_null() {
                                continue;
                            }
                            family = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "kind" => {
                            if v.is_null() {
                                continue;
                            }
                            kind = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _kind) = kind {
                                match _kind {
                                    crate::datadogV2::model::AiGatewayModelMetadataKind::UnparsedObject(_kind) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "knowledge_cutoff" => {
                            if v.is_null() {
                                continue;
                            }
                            knowledge_cutoff =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_updated" => {
                            if v.is_null() {
                                continue;
                            }
                            last_updated =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lifecycle" => {
                            if v.is_null() {
                                continue;
                            }
                            lifecycle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _lifecycle) = lifecycle {
                                match _lifecycle {
                                    crate::datadogV2::model::AiGatewayModelMetadataLifecycle::UnparsedObject(_lifecycle) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "limits" => {
                            limits = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modalities" => {
                            modalities = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "open_weights" => {
                            if v.is_null() {
                                continue;
                            }
                            open_weights =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reasoning" => {
                            if v.is_null() {
                                continue;
                            }
                            reasoning = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reference_pricing" => {
                            if v.is_null() {
                                continue;
                            }
                            reference_pricing =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "release_date" => {
                            if v.is_null() {
                                continue;
                            }
                            release_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::AiGatewayModelMetadataStatus::UnparsedObject(_status) => {
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
                let capabilities =
                    capabilities.ok_or_else(|| M::Error::missing_field("capabilities"))?;
                let limits = limits.ok_or_else(|| M::Error::missing_field("limits"))?;
                let modalities = modalities.ok_or_else(|| M::Error::missing_field("modalities"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = AiGatewayModelMetadata {
                    capabilities,
                    description,
                    display_name,
                    family,
                    kind,
                    knowledge_cutoff,
                    last_updated,
                    lifecycle,
                    limits,
                    modalities,
                    open_weights,
                    reasoning,
                    reference_pricing,
                    release_date,
                    source,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayModelMetadataVisitor)
    }
}
