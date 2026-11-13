// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes used to update an AI Gateway rate limit. Only sent fields are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayRateLimitUpdateAttributes {
    /// Spend limit in microcents per rolling minute. Set to 0 to remove the cost limit. Null is ignored. At least one limit must remain.
    #[serde(
        rename = "cost_microcents_per_minute",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub cost_microcents_per_minute: Option<Option<i64>>,
    /// Whether this rate limit is active.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Set to 0 to remove the request limit. Null is ignored. At least one limit must remain.
    #[serde(
        rename = "input_tokens_per_minute",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub input_tokens_per_minute: Option<Option<i64>>,
    /// Action taken when request rate exceeds the limit.
    #[serde(rename = "mode")]
    pub mode: Option<crate::datadogV2::model::AiGatewayRateLimitMode>,
    /// Rate limit name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Set to 0 to remove the request limit. Null is ignored. At least one limit must remain.
    #[serde(
        rename = "output_tokens_per_minute",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub output_tokens_per_minute: Option<Option<i64>>,
    /// Set to 0 to remove the request limit. Null is ignored. At least one limit must remain.
    #[serde(
        rename = "requests_per_minute",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub requests_per_minute: Option<Option<i64>>,
    /// Replaces all scope tags. An empty object clears them. Null is ignored.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option")]
    pub tags: Option<Option<std::collections::BTreeMap<String, String>>>,
    /// Replaces the Team scope. An empty string clears it. Null is ignored.
    #[serde(
        rename = "team_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub team_id: Option<Option<String>>,
    /// Replaces the user scope. An empty string clears it. Null is ignored.
    #[serde(
        rename = "user_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub user_id: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayRateLimitUpdateAttributes {
    pub fn new() -> AiGatewayRateLimitUpdateAttributes {
        AiGatewayRateLimitUpdateAttributes {
            cost_microcents_per_minute: None,
            enabled: None,
            input_tokens_per_minute: None,
            mode: None,
            name: None,
            output_tokens_per_minute: None,
            requests_per_minute: None,
            tags: None,
            team_id: None,
            user_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cost_microcents_per_minute(mut self, value: Option<i64>) -> Self {
        self.cost_microcents_per_minute = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn input_tokens_per_minute(mut self, value: Option<i64>) -> Self {
        self.input_tokens_per_minute = Some(value);
        self
    }

    pub fn mode(mut self, value: crate::datadogV2::model::AiGatewayRateLimitMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn output_tokens_per_minute(mut self, value: Option<i64>) -> Self {
        self.output_tokens_per_minute = Some(value);
        self
    }

    pub fn requests_per_minute(mut self, value: Option<i64>) -> Self {
        self.requests_per_minute = Some(value);
        self
    }

    pub fn tags(mut self, value: Option<std::collections::BTreeMap<String, String>>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn team_id(mut self, value: Option<String>) -> Self {
        self.team_id = Some(value);
        self
    }

    pub fn user_id(mut self, value: Option<String>) -> Self {
        self.user_id = Some(value);
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

impl Default for AiGatewayRateLimitUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AiGatewayRateLimitUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayRateLimitUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayRateLimitUpdateAttributesVisitor {
            type Value = AiGatewayRateLimitUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cost_microcents_per_minute: Option<Option<i64>> = None;
                let mut enabled: Option<bool> = None;
                let mut input_tokens_per_minute: Option<Option<i64>> = None;
                let mut mode: Option<crate::datadogV2::model::AiGatewayRateLimitMode> = None;
                let mut name: Option<String> = None;
                let mut output_tokens_per_minute: Option<Option<i64>> = None;
                let mut requests_per_minute: Option<Option<i64>> = None;
                let mut tags: Option<Option<std::collections::BTreeMap<String, String>>> = None;
                let mut team_id: Option<Option<String>> = None;
                let mut user_id: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cost_microcents_per_minute" => {
                            cost_microcents_per_minute =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "input_tokens_per_minute" => {
                            input_tokens_per_minute =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mode" => {
                            if v.is_null() {
                                continue;
                            }
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mode) = mode {
                                match _mode {
                                    crate::datadogV2::model::AiGatewayRateLimitMode::UnparsedObject(_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "output_tokens_per_minute" => {
                            output_tokens_per_minute =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "requests_per_minute" => {
                            requests_per_minute =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team_id" => {
                            team_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_id" => {
                            user_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AiGatewayRateLimitUpdateAttributes {
                    cost_microcents_per_minute,
                    enabled,
                    input_tokens_per_minute,
                    mode,
                    name,
                    output_tokens_per_minute,
                    requests_per_minute,
                    tags,
                    team_id,
                    user_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayRateLimitUpdateAttributesVisitor)
    }
}
