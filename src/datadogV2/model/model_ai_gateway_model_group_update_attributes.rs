// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes used to update an AI Gateway model group.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayModelGroupUpdateAttributes {
    /// Whether fast mode is allowed for models and routing rules in the group.
    #[serde(rename = "allow_fast_mode")]
    pub allow_fast_mode: Option<bool>,
    /// Required when updating the default model group. Must match its current revision; a stale revision returns HTTP 409. Omit for named policies.
    #[serde(rename = "expected_revision")]
    pub expected_revision: Option<i64>,
    /// Maximum reasoning level permitted by a model group.
    #[serde(
        rename = "max_reasoning_level",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub max_reasoning_level:
        Option<Option<crate::datadogV2::model::AiGatewayNullableMaxReasoningLevel>>,
    /// Individually selected model catalog IDs. Replaces this selection only. Omit to preserve it; an empty array clears it. Null is invalid. Individual selections do not include future models. New selections require provider View; retaining or removing saved hidden selections requires only parent Edit.
    #[serde(rename = "model_ids")]
    pub model_ids: Option<Vec<i64>>,
    /// Human-readable resource name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Explicit provider selections, including current and future models. Omit to preserve selections on update; an empty array clears them. Null is invalid. New selections require provider View; saved hidden IDs can be retained or removed.
    #[serde(rename = "provider_ids")]
    pub provider_ids: Option<Vec<i64>>,
    /// Organization-unique routing rule slugs. Replaces route selections only. Omit to preserve them; an empty array clears them. Null is invalid.
    #[serde(rename = "routing_rule_slugs")]
    pub routing_rule_slugs: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayModelGroupUpdateAttributes {
    pub fn new() -> AiGatewayModelGroupUpdateAttributes {
        AiGatewayModelGroupUpdateAttributes {
            allow_fast_mode: None,
            expected_revision: None,
            max_reasoning_level: None,
            model_ids: None,
            name: None,
            provider_ids: None,
            routing_rule_slugs: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn allow_fast_mode(mut self, value: bool) -> Self {
        self.allow_fast_mode = Some(value);
        self
    }

    pub fn expected_revision(mut self, value: i64) -> Self {
        self.expected_revision = Some(value);
        self
    }

    pub fn max_reasoning_level(
        mut self,
        value: Option<crate::datadogV2::model::AiGatewayNullableMaxReasoningLevel>,
    ) -> Self {
        self.max_reasoning_level = Some(value);
        self
    }

    pub fn model_ids(mut self, value: Vec<i64>) -> Self {
        self.model_ids = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn provider_ids(mut self, value: Vec<i64>) -> Self {
        self.provider_ids = Some(value);
        self
    }

    pub fn routing_rule_slugs(mut self, value: Vec<String>) -> Self {
        self.routing_rule_slugs = Some(value);
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

impl Default for AiGatewayModelGroupUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AiGatewayModelGroupUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayModelGroupUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayModelGroupUpdateAttributesVisitor {
            type Value = AiGatewayModelGroupUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allow_fast_mode: Option<bool> = None;
                let mut expected_revision: Option<i64> = None;
                let mut max_reasoning_level: Option<
                    Option<crate::datadogV2::model::AiGatewayNullableMaxReasoningLevel>,
                > = None;
                let mut model_ids: Option<Vec<i64>> = None;
                let mut name: Option<String> = None;
                let mut provider_ids: Option<Vec<i64>> = None;
                let mut routing_rule_slugs: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allow_fast_mode" => {
                            if v.is_null() {
                                continue;
                            }
                            allow_fast_mode =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expected_revision" => {
                            if v.is_null() {
                                continue;
                            }
                            expected_revision =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                            if v.is_null() {
                                continue;
                            }
                            model_ids = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider_ids" => {
                            if v.is_null() {
                                continue;
                            }
                            provider_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "routing_rule_slugs" => {
                            if v.is_null() {
                                continue;
                            }
                            routing_rule_slugs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AiGatewayModelGroupUpdateAttributes {
                    allow_fast_mode,
                    expected_revision,
                    max_reasoning_level,
                    model_ids,
                    name,
                    provider_ids,
                    routing_rule_slugs,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayModelGroupUpdateAttributesVisitor)
    }
}
