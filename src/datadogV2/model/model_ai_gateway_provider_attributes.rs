// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// AI Gateway provider configuration and metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayProviderAttributes {
    /// Multiplier applied to provider costs.
    #[serde(rename = "cost_multiplier")]
    pub cost_multiplier: f64,
    /// Time when the resource was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Provider integration account identifier.
    #[serde(rename = "integration_account_id")]
    pub integration_account_id: String,
    /// Provider-specific integration configuration.
    #[serde(rename = "integration_config")]
    pub integration_config: Option<crate::datadogV2::model::AiGatewayProviderIntegrationConfig>,
    /// Known provider implementation.
    #[serde(rename = "known_provider")]
    pub known_provider: crate::datadogV2::model::AiGatewayKnownProvider,
    /// Human-readable resource name.
    #[serde(rename = "name")]
    pub name: String,
    /// Immutable routing identifier.
    #[serde(rename = "slug")]
    pub slug: String,
    /// Time when the resource was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayProviderAttributes {
    pub fn new(
        cost_multiplier: f64,
        created_at: chrono::DateTime<chrono::Utc>,
        integration_account_id: String,
        known_provider: crate::datadogV2::model::AiGatewayKnownProvider,
        name: String,
        slug: String,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> AiGatewayProviderAttributes {
        AiGatewayProviderAttributes {
            cost_multiplier,
            created_at,
            integration_account_id,
            integration_config: None,
            known_provider,
            name,
            slug,
            updated_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn integration_config(
        mut self,
        value: crate::datadogV2::model::AiGatewayProviderIntegrationConfig,
    ) -> Self {
        self.integration_config = Some(value);
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

impl<'de> Deserialize<'de> for AiGatewayProviderAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayProviderAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayProviderAttributesVisitor {
            type Value = AiGatewayProviderAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cost_multiplier: Option<f64> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut integration_account_id: Option<String> = None;
                let mut integration_config: Option<
                    crate::datadogV2::model::AiGatewayProviderIntegrationConfig,
                > = None;
                let mut known_provider: Option<crate::datadogV2::model::AiGatewayKnownProvider> =
                    None;
                let mut name: Option<String> = None;
                let mut slug: Option<String> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cost_multiplier" => {
                            cost_multiplier =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_account_id" => {
                            integration_account_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_config" => {
                            if v.is_null() {
                                continue;
                            }
                            integration_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "known_provider" => {
                            known_provider =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _known_provider) = known_provider {
                                match _known_provider {
                                    crate::datadogV2::model::AiGatewayKnownProvider::UnparsedObject(_known_provider) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "slug" => {
                            slug = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let cost_multiplier =
                    cost_multiplier.ok_or_else(|| M::Error::missing_field("cost_multiplier"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let integration_account_id = integration_account_id
                    .ok_or_else(|| M::Error::missing_field("integration_account_id"))?;
                let known_provider =
                    known_provider.ok_or_else(|| M::Error::missing_field("known_provider"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let slug = slug.ok_or_else(|| M::Error::missing_field("slug"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = AiGatewayProviderAttributes {
                    cost_multiplier,
                    created_at,
                    integration_account_id,
                    integration_config,
                    known_provider,
                    name,
                    slug,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayProviderAttributesVisitor)
    }
}
