// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Provenance for model metadata enrichment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayModelMetadataSource {
    /// Identifier strategy used to match the gateway catalog entry. Date-suffix and shared canonical-model fallback matches produce partial metadata.
    #[serde(rename = "matched_by")]
    pub matched_by: crate::datadogV2::model::AiGatewayModelMetadataSourceMatchedBy,
    /// Model identifier in the metadata source provider catalog.
    #[serde(rename = "model_id")]
    pub model_id: String,
    /// Metadata source name.
    #[serde(rename = "name")]
    pub name: String,
    /// Last successful observation of this valid source record. Retained invalid entries keep their previous timestamp.
    #[serde(rename = "observed_at")]
    pub observed_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Provider namespace in the metadata source, or canonical for a shared model record.
    #[serde(rename = "provider")]
    pub provider: String,
    /// SHA-256 content revision of the source snapshot that supplied this record.
    #[serde(rename = "revision")]
    pub revision: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayModelMetadataSource {
    pub fn new(
        matched_by: crate::datadogV2::model::AiGatewayModelMetadataSourceMatchedBy,
        model_id: String,
        name: String,
        provider: String,
    ) -> AiGatewayModelMetadataSource {
        AiGatewayModelMetadataSource {
            matched_by,
            model_id,
            name,
            observed_at: None,
            provider,
            revision: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn observed_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.observed_at = Some(value);
        self
    }

    pub fn revision(mut self, value: String) -> Self {
        self.revision = Some(value);
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

impl<'de> Deserialize<'de> for AiGatewayModelMetadataSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayModelMetadataSourceVisitor;
        impl<'a> Visitor<'a> for AiGatewayModelMetadataSourceVisitor {
            type Value = AiGatewayModelMetadataSource;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut matched_by: Option<
                    crate::datadogV2::model::AiGatewayModelMetadataSourceMatchedBy,
                > = None;
                let mut model_id: Option<String> = None;
                let mut name: Option<String> = None;
                let mut observed_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut provider: Option<String> = None;
                let mut revision: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "matched_by" => {
                            matched_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _matched_by) = matched_by {
                                match _matched_by {
                                    crate::datadogV2::model::AiGatewayModelMetadataSourceMatchedBy::UnparsedObject(_matched_by) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "model_id" => {
                            model_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "observed_at" => {
                            if v.is_null() {
                                continue;
                            }
                            observed_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider" => {
                            provider = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "revision" => {
                            if v.is_null() {
                                continue;
                            }
                            revision = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let matched_by = matched_by.ok_or_else(|| M::Error::missing_field("matched_by"))?;
                let model_id = model_id.ok_or_else(|| M::Error::missing_field("model_id"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let provider = provider.ok_or_else(|| M::Error::missing_field("provider"))?;

                let content = AiGatewayModelMetadataSource {
                    matched_by,
                    model_id,
                    name,
                    observed_at,
                    provider,
                    revision,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayModelMetadataSourceVisitor)
    }
}
