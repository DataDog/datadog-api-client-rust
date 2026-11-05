// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Complete AI Gateway model group configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayModelGroupAttributes {
    /// Whether fast mode is allowed for models and routing rules in the group.
    #[serde(rename = "allow_fast_mode")]
    pub allow_fast_mode: bool,
    /// Time when the resource was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Maximum reasoning level permitted by a model group.
    #[serialize_always]
    #[serde(rename = "max_reasoning_level")]
    pub max_reasoning_level: Option<crate::datadogV2::model::AiGatewayNullableMaxReasoningLevel>,
    /// All saved model catalog IDs, including selections whose provider metadata is hidden. Preserve these IDs independently of the visible catalog.
    #[serde(rename = "model_ids")]
    pub model_ids: Vec<i64>,
    /// Metadata for saved model selections whose providers the caller can view. Missing metadata does not remove a saved ID.
    #[serde(rename = "models")]
    pub models: Vec<crate::datadogV2::model::AiGatewayModelGroupModel>,
    /// Human-readable resource name.
    #[serde(rename = "name")]
    pub name: String,
    /// All explicitly selected provider IDs, including hidden providers. Embedded provider metadata includes only viewable providers.
    #[serde(rename = "provider_ids")]
    pub provider_ids: Vec<i64>,
    /// Viewable explicitly selected providers, separate from individually selected models. Use provider_ids to retain all selections.
    #[serde(rename = "providers")]
    pub providers: Vec<crate::datadogV2::model::AiGatewayModelGroupProvider>,
    /// Revision of the organization-wide policy. Present for the default model group and absent for named policies.
    #[serde(rename = "revision")]
    pub revision: Option<i64>,
    #[serde(rename = "routing_rules")]
    pub routing_rules: Vec<crate::datadogV2::model::AiGatewayModelGroupRoutingRule>,
    /// Time when the resource was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayModelGroupAttributes {
    pub fn new(
        allow_fast_mode: bool,
        created_at: chrono::DateTime<chrono::Utc>,
        max_reasoning_level: Option<crate::datadogV2::model::AiGatewayNullableMaxReasoningLevel>,
        model_ids: Vec<i64>,
        models: Vec<crate::datadogV2::model::AiGatewayModelGroupModel>,
        name: String,
        provider_ids: Vec<i64>,
        providers: Vec<crate::datadogV2::model::AiGatewayModelGroupProvider>,
        routing_rules: Vec<crate::datadogV2::model::AiGatewayModelGroupRoutingRule>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> AiGatewayModelGroupAttributes {
        AiGatewayModelGroupAttributes {
            allow_fast_mode,
            created_at,
            max_reasoning_level,
            model_ids,
            models,
            name,
            provider_ids,
            providers,
            revision: None,
            routing_rules,
            updated_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn revision(mut self, value: i64) -> Self {
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

impl<'de> Deserialize<'de> for AiGatewayModelGroupAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayModelGroupAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayModelGroupAttributesVisitor {
            type Value = AiGatewayModelGroupAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allow_fast_mode: Option<bool> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut max_reasoning_level: Option<
                    Option<crate::datadogV2::model::AiGatewayNullableMaxReasoningLevel>,
                > = None;
                let mut model_ids: Option<Vec<i64>> = None;
                let mut models: Option<Vec<crate::datadogV2::model::AiGatewayModelGroupModel>> =
                    None;
                let mut name: Option<String> = None;
                let mut provider_ids: Option<Vec<i64>> = None;
                let mut providers: Option<
                    Vec<crate::datadogV2::model::AiGatewayModelGroupProvider>,
                > = None;
                let mut revision: Option<i64> = None;
                let mut routing_rules: Option<
                    Vec<crate::datadogV2::model::AiGatewayModelGroupRoutingRule>,
                > = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allow_fast_mode" => {
                            allow_fast_mode =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "max_reasoning_level" => {
                            max_reasoning_level =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _max_reasoning_level) = max_reasoning_level {
                                match _max_reasoning_level {
                                    Some(crate::datadogV2::model::AiGatewayNullableMaxReasoningLevel::UnparsedObject(_max_reasoning_level)) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "model_ids" => {
                            model_ids = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "models" => {
                            models = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider_ids" => {
                            provider_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "providers" => {
                            providers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "revision" => {
                            if v.is_null() {
                                continue;
                            }
                            revision = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "routing_rules" => {
                            routing_rules =
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
                let allow_fast_mode =
                    allow_fast_mode.ok_or_else(|| M::Error::missing_field("allow_fast_mode"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let max_reasoning_level = max_reasoning_level
                    .ok_or_else(|| M::Error::missing_field("max_reasoning_level"))?;
                let model_ids = model_ids.ok_or_else(|| M::Error::missing_field("model_ids"))?;
                let models = models.ok_or_else(|| M::Error::missing_field("models"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let provider_ids =
                    provider_ids.ok_or_else(|| M::Error::missing_field("provider_ids"))?;
                let providers = providers.ok_or_else(|| M::Error::missing_field("providers"))?;
                let routing_rules =
                    routing_rules.ok_or_else(|| M::Error::missing_field("routing_rules"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = AiGatewayModelGroupAttributes {
                    allow_fast_mode,
                    created_at,
                    max_reasoning_level,
                    model_ids,
                    models,
                    name,
                    provider_ids,
                    providers,
                    revision,
                    routing_rules,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayModelGroupAttributesVisitor)
    }
}
