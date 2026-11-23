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
    /// Whose credential the gateway sends to the provider. Defaults to `integration_account`, which sends
    /// the integration account's credential. `subscription` forwards each caller's own subscription sign-in
    /// (`ChatGPT` for `openai`, `claude.ai` for `anthropic`) and needs no integration account; it is available
    /// only to organizations with subscription providers enabled. A subscription provider's models are
    /// optional: callers can use any model their subscription includes, and a model you add sets its
    /// pricing. Immutable.
    #[serde(rename = "credential_source")]
    pub credential_source: Option<crate::datadogV2::model::AiGatewayProviderCreateCredentialSource>,
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
    pub known_provider: crate::datadogV2::model::AiGatewayKnownProvider,
    /// Manually managed models to add alongside models discovered from the provider integration.
    #[serde(rename = "models")]
    pub models: Option<Vec<crate::datadogV2::model::AiGatewayModelInput>>,
    /// Human-readable resource name.
    #[serde(rename = "name")]
    pub name: String,
    /// Immutable routing identifier.
    #[serde(rename = "slug")]
    pub slug: String,
    /// Whether this provider connection is covered by a zero data retention agreement. Defaults to `false`.
    /// For Bedrock, zero data retention is granted per region, so configure one provider per region and set
    /// this only on the covered ones.
    #[serde(rename = "zdr_eligible")]
    pub zdr_eligible: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayProviderCreateAttributes {
    pub fn new(
        known_provider: crate::datadogV2::model::AiGatewayKnownProvider,
        name: String,
        slug: String,
    ) -> AiGatewayProviderCreateAttributes {
        AiGatewayProviderCreateAttributes {
            cost_multiplier: None,
            credential_source: None,
            initial_bindings: None,
            integration_account_id: None,
            integration_config: None,
            known_provider,
            models: None,
            name,
            slug,
            zdr_eligible: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cost_multiplier(mut self, value: f64) -> Self {
        self.cost_multiplier = Some(value);
        self
    }

    pub fn credential_source(
        mut self,
        value: crate::datadogV2::model::AiGatewayProviderCreateCredentialSource,
    ) -> Self {
        self.credential_source = Some(value);
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

    pub fn models(mut self, value: Vec<crate::datadogV2::model::AiGatewayModelInput>) -> Self {
        self.models = Some(value);
        self
    }

    pub fn zdr_eligible(mut self, value: bool) -> Self {
        self.zdr_eligible = Some(value);
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
                let mut credential_source: Option<
                    crate::datadogV2::model::AiGatewayProviderCreateCredentialSource,
                > = None;
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
                let mut zdr_eligible: Option<bool> = None;
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
                        "credential_source" => {
                            if v.is_null() {
                                continue;
                            }
                            credential_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _credential_source) = credential_source {
                                match _credential_source {
                                    crate::datadogV2::model::AiGatewayProviderCreateCredentialSource::UnparsedObject(_credential_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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
                        "zdr_eligible" => {
                            if v.is_null() {
                                continue;
                            }
                            zdr_eligible =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let known_provider =
                    known_provider.ok_or_else(|| M::Error::missing_field("known_provider"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let slug = slug.ok_or_else(|| M::Error::missing_field("slug"))?;

                let content = AiGatewayProviderCreateAttributes {
                    cost_multiplier,
                    credential_source,
                    initial_bindings,
                    integration_account_id,
                    integration_config,
                    known_provider,
                    models,
                    name,
                    slug,
                    zdr_eligible,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayProviderCreateAttributesVisitor)
    }
}
