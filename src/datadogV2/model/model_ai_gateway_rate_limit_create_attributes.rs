// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes used to create an AI Gateway rate limit.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayRateLimitCreateAttributes {
    /// Maximum input tokens per rolling minute.
    #[serde(
        rename = "input_tokens_per_minute",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub input_tokens_per_minute: Option<Option<i64>>,
    /// Action taken when request rate exceeds the limit.
    #[serde(rename = "mode")]
    pub mode: Option<crate::datadogV2::model::AiGatewayRateLimitMode>,
    /// Rate limit name. Must not be empty.
    #[serde(rename = "name")]
    pub name: String,
    /// Maximum output tokens per rolling minute.
    #[serde(
        rename = "output_tokens_per_minute",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub output_tokens_per_minute: Option<Option<i64>>,
    /// Maximum requests per rolling minute. At least one of `requests_per_minute`, `input_tokens_per_minute`, and `output_tokens_per_minute` is required.
    #[serde(
        rename = "requests_per_minute",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub requests_per_minute: Option<Option<i64>>,
    /// Optional scope tags, such as `provider`, `model`, `ml_app`, or custom dimensions. A request matches when it has all of these tags. The reserved metadata keys `name`, `budget_name`, and `span_filter` are rejected.
    #[serde(rename = "tags")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
    /// Optional UUID. Count only requests attributed to this Datadog Team.
    #[serde(rename = "team_id")]
    pub team_id: Option<uuid::Uuid>,
    /// Optional UUID. Count only this user's requests.
    #[serde(rename = "user_id")]
    pub user_id: Option<uuid::Uuid>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayRateLimitCreateAttributes {
    pub fn new(name: String) -> AiGatewayRateLimitCreateAttributes {
        AiGatewayRateLimitCreateAttributes {
            input_tokens_per_minute: None,
            mode: None,
            name,
            output_tokens_per_minute: None,
            requests_per_minute: None,
            tags: None,
            team_id: None,
            user_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn input_tokens_per_minute(mut self, value: Option<i64>) -> Self {
        self.input_tokens_per_minute = Some(value);
        self
    }

    pub fn mode(mut self, value: crate::datadogV2::model::AiGatewayRateLimitMode) -> Self {
        self.mode = Some(value);
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

    pub fn tags(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn team_id(mut self, value: uuid::Uuid) -> Self {
        self.team_id = Some(value);
        self
    }

    pub fn user_id(mut self, value: uuid::Uuid) -> Self {
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

impl<'de> Deserialize<'de> for AiGatewayRateLimitCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayRateLimitCreateAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayRateLimitCreateAttributesVisitor {
            type Value = AiGatewayRateLimitCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut input_tokens_per_minute: Option<Option<i64>> = None;
                let mut mode: Option<crate::datadogV2::model::AiGatewayRateLimitMode> = None;
                let mut name: Option<String> = None;
                let mut output_tokens_per_minute: Option<Option<i64>> = None;
                let mut requests_per_minute: Option<Option<i64>> = None;
                let mut tags: Option<std::collections::BTreeMap<String, String>> = None;
                let mut team_id: Option<uuid::Uuid> = None;
                let mut user_id: Option<uuid::Uuid> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team_id" => {
                            if v.is_null() {
                                continue;
                            }
                            team_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_id" => {
                            if v.is_null() {
                                continue;
                            }
                            user_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = AiGatewayRateLimitCreateAttributes {
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

        deserializer.deserialize_any(AiGatewayRateLimitCreateAttributesVisitor)
    }
}
