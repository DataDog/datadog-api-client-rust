// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Model target or reference to another routing rule used to create or replace a rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayRoutingRuleTargetRequest {
    /// Marks the default destination of a classified tier. A classified tier has exactly one default, and the default has no rubric. It receives requests that no rubric fits, requests the decision model cannot place, and picks below the confidence threshold. Must be omitted, null, or false in other tiers.
    #[serde(
        rename = "default",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub default: Option<Option<bool>>,
    /// Saved target row ID. On replacement, retain this ID only within its original route and at most once. A stale or foreign ID is rejected.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// Model name. Omit when retaining a saved target by ID or selecting a concrete target by model_id.
    #[serde(rename = "model")]
    pub model: Option<String>,
    /// Opaque model catalog ID for a concrete model target. New selections require provider View; saved selections can be retained without metadata.
    #[serde(rename = "model_id")]
    pub model_id: Option<i64>,
    /// Provider slug. weighted_affinity and classified tiers require an explicit provider and do not allow "*".
    #[serde(rename = "provider")]
    pub provider: Option<String>,
    /// Optional reasoning effort preference for this model destination. Must be supported by the selected model and provider. For provider "*", every eligible provider must support the level. An explicit value overrides the request's reasoning setting, subject to model-group policy. Omitted or null preserves request and provider behavior; none is an explicit level, not the default. Omitted or null clears an override on full replacement. Routing-rule references cannot set this field; each referenced model destination owns its override. If an override cannot be applied at runtime, the gateway keeps the destination and restores the caller's original reasoning settings, or preserves the provider default when omitted. It rechecks all applicable route and model-group policies against those settings before sending. Model-group max_reasoning_level enforces hard limits; the override is not an additional cap. Normal fallback applies if governance denies the resulting request or the destination cannot otherwise serve it. Metrics and span tags record the override outcome and incompatibility reason. When response headers have not already been committed, `X-Ai-Gateway-Reasoning-Override` reports applied or not_applied for the returned attempt, and `X-Ai-Gateway-Reasoning-Override-Reason` reports the reason when not applied.
    #[serde(
        rename = "reasoning_level",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub reasoning_level: Option<Option<crate::datadogV2::model::AiGatewayRoutingReasoningLevel>>,
    /// Describes when the decision model picks this destination, in up to 300 characters. Required on every destination of a classified tier except the default, and different for each destination in the tier. Leading and trailing spaces are trimmed. Must be omitted or null on the default destination and in other tiers.
    #[serde(rename = "rubric", default, with = "::serde_with::rust::double_option")]
    pub rubric: Option<Option<String>>,
    /// Organization-unique routing rule slug. Allowed only in configured tiers.
    #[serde(rename = "slug")]
    pub slug: Option<String>,
    /// Type of routing target.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::AiGatewayRoutingRuleTargetType,
    /// Relative weight in a weighted_affinity tier. Omitted or null defaults to 1. Weights need not total 100. Must be omitted or null in configured and classified tiers.
    #[serde(rename = "weight", default, with = "::serde_with::rust::double_option")]
    pub weight: Option<Option<i64>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayRoutingRuleTargetRequest {
    pub fn new(
        type_: crate::datadogV2::model::AiGatewayRoutingRuleTargetType,
    ) -> AiGatewayRoutingRuleTargetRequest {
        AiGatewayRoutingRuleTargetRequest {
            default: None,
            id: None,
            model: None,
            model_id: None,
            provider: None,
            reasoning_level: None,
            rubric: None,
            slug: None,
            type_,
            weight: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn default(mut self, value: Option<bool>) -> Self {
        self.default = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn model(mut self, value: String) -> Self {
        self.model = Some(value);
        self
    }

    pub fn model_id(mut self, value: i64) -> Self {
        self.model_id = Some(value);
        self
    }

    pub fn provider(mut self, value: String) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn reasoning_level(
        mut self,
        value: Option<crate::datadogV2::model::AiGatewayRoutingReasoningLevel>,
    ) -> Self {
        self.reasoning_level = Some(value);
        self
    }

    pub fn rubric(mut self, value: Option<String>) -> Self {
        self.rubric = Some(value);
        self
    }

    pub fn slug(mut self, value: String) -> Self {
        self.slug = Some(value);
        self
    }

    pub fn weight(mut self, value: Option<i64>) -> Self {
        self.weight = Some(value);
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

impl<'de> Deserialize<'de> for AiGatewayRoutingRuleTargetRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayRoutingRuleTargetRequestVisitor;
        impl<'a> Visitor<'a> for AiGatewayRoutingRuleTargetRequestVisitor {
            type Value = AiGatewayRoutingRuleTargetRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut default: Option<Option<bool>> = None;
                let mut id: Option<i64> = None;
                let mut model: Option<String> = None;
                let mut model_id: Option<i64> = None;
                let mut provider: Option<String> = None;
                let mut reasoning_level: Option<
                    Option<crate::datadogV2::model::AiGatewayRoutingReasoningLevel>,
                > = None;
                let mut rubric: Option<Option<String>> = None;
                let mut slug: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::AiGatewayRoutingRuleTargetType> =
                    None;
                let mut weight: Option<Option<i64>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "default" => {
                            default = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "model" => {
                            if v.is_null() {
                                continue;
                            }
                            model = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "model_id" => {
                            if v.is_null() {
                                continue;
                            }
                            model_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider" => {
                            if v.is_null() {
                                continue;
                            }
                            provider = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reasoning_level" => {
                            reasoning_level =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _reasoning_level) = reasoning_level {
                                match _reasoning_level {
                                    Some(crate::datadogV2::model::AiGatewayRoutingReasoningLevel::UnparsedObject(_reasoning_level)) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "rubric" => {
                            rubric = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "slug" => {
                            if v.is_null() {
                                continue;
                            }
                            slug = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::AiGatewayRoutingRuleTargetType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "weight" => {
                            weight = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = AiGatewayRoutingRuleTargetRequest {
                    default,
                    id,
                    model,
                    model_id,
                    provider,
                    reasoning_level,
                    rubric,
                    slug,
                    type_,
                    weight,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayRoutingRuleTargetRequestVisitor)
    }
}
