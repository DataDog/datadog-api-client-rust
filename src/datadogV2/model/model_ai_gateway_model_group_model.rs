// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Model catalog entry embedded in a model group.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayModelGroupModel {
    /// Time when the resource was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Whether the resource is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Model catalog entry identifier.
    #[serde(rename = "id")]
    pub id: String,
    /// Optional, typed enrichment for a model catalog entry. Missing properties remain unknown.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV2::model::AiGatewayModelMetadata>,
    /// Provider-independent model identifier.
    #[serde(rename = "model_id_canonical")]
    pub model_id_canonical: String,
    /// Model identifier sent to the provider.
    #[serde(rename = "model_id_wire")]
    pub model_id_wire: String,
    /// Provider identifier.
    #[serde(rename = "provider_id")]
    pub provider_id: i64,
    /// How a model catalog entry is managed.
    #[serde(rename = "source")]
    pub source: crate::datadogV2::model::AiGatewayModelSource,
    /// Operations supported by the model.
    #[serde(rename = "supported_operations")]
    pub supported_operations: Vec<crate::datadogV2::model::AiGatewaySupportedOperation>,
    /// Time when the resource was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayModelGroupModel {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        enabled: bool,
        id: String,
        model_id_canonical: String,
        model_id_wire: String,
        provider_id: i64,
        source: crate::datadogV2::model::AiGatewayModelSource,
        supported_operations: Vec<crate::datadogV2::model::AiGatewaySupportedOperation>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> AiGatewayModelGroupModel {
        AiGatewayModelGroupModel {
            created_at,
            enabled,
            id,
            metadata: None,
            model_id_canonical,
            model_id_wire,
            provider_id,
            source,
            supported_operations,
            updated_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn metadata(mut self, value: crate::datadogV2::model::AiGatewayModelMetadata) -> Self {
        self.metadata = Some(value);
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

impl<'de> Deserialize<'de> for AiGatewayModelGroupModel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayModelGroupModelVisitor;
        impl<'a> Visitor<'a> for AiGatewayModelGroupModelVisitor {
            type Value = AiGatewayModelGroupModel;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut enabled: Option<bool> = None;
                let mut id: Option<String> = None;
                let mut metadata: Option<crate::datadogV2::model::AiGatewayModelMetadata> = None;
                let mut model_id_canonical: Option<String> = None;
                let mut model_id_wire: Option<String> = None;
                let mut provider_id: Option<i64> = None;
                let mut source: Option<crate::datadogV2::model::AiGatewayModelSource> = None;
                let mut supported_operations: Option<
                    Vec<crate::datadogV2::model::AiGatewaySupportedOperation>,
                > = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "model_id_canonical" => {
                            model_id_canonical =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "model_id_wire" => {
                            model_id_wire =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider_id" => {
                            provider_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _source) = source {
                                match _source {
                                    crate::datadogV2::model::AiGatewayModelSource::UnparsedObject(_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "supported_operations" => {
                            supported_operations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let model_id_canonical = model_id_canonical
                    .ok_or_else(|| M::Error::missing_field("model_id_canonical"))?;
                let model_id_wire =
                    model_id_wire.ok_or_else(|| M::Error::missing_field("model_id_wire"))?;
                let provider_id =
                    provider_id.ok_or_else(|| M::Error::missing_field("provider_id"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let supported_operations = supported_operations
                    .ok_or_else(|| M::Error::missing_field("supported_operations"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = AiGatewayModelGroupModel {
                    created_at,
                    enabled,
                    id,
                    metadata,
                    model_id_canonical,
                    model_id_wire,
                    provider_id,
                    source,
                    supported_operations,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayModelGroupModelVisitor)
    }
}
