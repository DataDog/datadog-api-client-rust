// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes used to create an AI Gateway provider.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayProviderCreateAttributes {
    /// Multiplier applied to provider costs. Must be greater than 0 and at most 1000000.
    #[serde(rename = "cost_multiplier")]
    pub cost_multiplier: Option<f64>,
    /// Reserved for initial View/Edit permissions. Supplying this attribute, including an empty array or null, returns HTTP 503 before discovery or persistence. Omit it for normal creation.
    #[serde(rename = "initial_bindings")]
    pub initial_bindings: Option<Vec<crate::datadogV2::model::AiGatewayProviderInitialBinding>>,
    /// Provider integration account identifier.
    #[serde(rename = "integration_account_id")]
    pub integration_account_id: Option<String>,
    /// Provider-specific integration configuration.
    #[serde(rename = "integration_config")]
    pub integration_config: Option<crate::datadogV2::model::AiGatewayProviderIntegrationConfig>,
    /// Known provider implementation.
    #[serde(rename = "known_provider")]
    pub known_provider: Option<crate::datadogV2::model::AiGatewayKnownProvider>,
    /// Manually managed models to add alongside models discovered from the provider integration.
    #[serde(rename = "models")]
    pub models: Option<Vec<crate::datadogV2::model::AiGatewayModelInput>>,
    /// Human-readable resource name.
    #[serde(rename = "name")]
    pub name: String,
    /// Immutable routing identifier.
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayProviderCreateAttributes {
    pub fn new(name: String, slug: String) -> AiGatewayProviderCreateAttributes {
        AiGatewayProviderCreateAttributes {
            cost_multiplier: None,
            initial_bindings: None,
            integration_account_id: None,
            integration_config: None,
            known_provider: None,
            models: None,
            name,
            slug,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cost_multiplier(mut self, value: f64) -> Self {
        self.cost_multiplier = Some(value);
        self
    }

    pub fn initial_bindings(
        mut self,
        value: Vec<crate::datadogV2::model::AiGatewayProviderInitialBinding>,
    ) -> Self {
        self.initial_bindings = Some(value);
        self
    }

    pub fn integration_account_id(mut self, value: String) -> Self {
        self.integration_account_id = Some(value);
        self
    }

    pub fn integration_config(
        mut self,
        value: crate::datadogV2::model::AiGatewayProviderIntegrationConfig,
    ) -> Self {
        self.integration_config = Some(value);
        self
    }

    pub fn known_provider(
        mut self,
        value: crate::datadogV2::model::AiGatewayKnownProvider,
    ) -> Self {
        self.known_provider = Some(value);
        self
    }

    pub fn models(mut self, value: Vec<crate::datadogV2::model::AiGatewayModelInput>) -> Self {
        self.models = Some(value);
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

impl<'de> Deserialize<'de> for AiGatewayProviderCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayProviderCreateAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayProviderCreateAttributesVisitor {
            type Value = AiGatewayProviderCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cost_multiplier: Option<f64> = None;
                let mut initial_bindings: Option<
                    Vec<crate::datadogV2::model::AiGatewayProviderInitialBinding>,
                > = None;
                let mut integration_account_id: Option<String> = None;
                let mut integration_config: Option<
                    crate::datadogV2::model::AiGatewayProviderIntegrationConfig,
                > = None;
                let mut known_provider: Option<crate::datadogV2::model::AiGatewayKnownProvider> =
                    None;
                let mut models: Option<Vec<crate::datadogV2::model::AiGatewayModelInput>> = None;
                let mut name: Option<String> = None;
                let mut slug: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cost_multiplier" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            cost_multiplier =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "initial_bindings" => {
                            if v.is_null() {
                                continue;
                            }
                            initial_bindings =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_account_id" => {
                            if v.is_null() {
                                continue;
                            }
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
                            if v.is_null() {
                                continue;
                            }
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
                        "models" => {
                            if v.is_null() {
                                continue;
                            }
                            models = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "slug" => {
                            slug = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let slug = slug.ok_or_else(|| M::Error::missing_field("slug"))?;

                let content = AiGatewayProviderCreateAttributes {
                    cost_multiplier,
                    initial_bindings,
                    integration_account_id,
                    integration_config,
                    known_provider,
                    models,
                    name,
                    slug,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayProviderCreateAttributesVisitor)
    }
}
