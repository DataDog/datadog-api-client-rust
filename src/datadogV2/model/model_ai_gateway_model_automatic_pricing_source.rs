// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// The pricing source entry the price was read from.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayModelAutomaticPricingSource {
    /// How the gateway matched the model to the source entry, such as `wire_id`.
    #[serde(rename = "match_method")]
    pub match_method: String,
    /// Model identifier in the source.
    #[serde(rename = "model_id")]
    pub model_id: String,
    /// Pricing source name.
    #[serde(rename = "name")]
    pub name: String,
    /// Provider identifier in the source.
    #[serde(rename = "provider_id")]
    pub provider_id: String,
    /// Source revision the price was read from.
    #[serde(rename = "revision")]
    pub revision: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl AiGatewayModelAutomaticPricingSource {
    pub fn new(match_method: String, model_id: String, name: String, provider_id: String, revision: String) -> AiGatewayModelAutomaticPricingSource {
        AiGatewayModelAutomaticPricingSource {
            match_method,
            model_id,
            name,
            provider_id,
            revision,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}



impl<'de> Deserialize<'de> for AiGatewayModelAutomaticPricingSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayModelAutomaticPricingSourceVisitor;
        impl<'a> Visitor<'a> for AiGatewayModelAutomaticPricingSourceVisitor {
            type Value = AiGatewayModelAutomaticPricingSource;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut match_method: Option<String> = None;
                let mut model_id: Option<String> = None;
                let mut name: Option<String> = None;
                let mut provider_id: Option<String> = None;
                let mut revision: Option<String> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "match_method" => {
                            match_method = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "model_id" => {
                            model_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "provider_id" => {
                            provider_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "revision" => {
                            revision = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }
                let match_method = match_method.ok_or_else(|| M::Error::missing_field("match_method"))?;
                let model_id = model_id.ok_or_else(|| M::Error::missing_field("model_id"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let provider_id = provider_id.ok_or_else(|| M::Error::missing_field("provider_id"))?;
                let revision = revision.ok_or_else(|| M::Error::missing_field("revision"))?;

                let content = AiGatewayModelAutomaticPricingSource {
                    match_method,
                    model_id,
                    name,
                    provider_id,
                    revision,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayModelAutomaticPricingSourceVisitor)
    }
}
