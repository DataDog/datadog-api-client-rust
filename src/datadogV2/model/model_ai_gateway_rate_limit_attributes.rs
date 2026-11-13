// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an AI Gateway rate limit.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayRateLimitAttributes {
    /// Creation timestamp.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Email of the user who created this rate limit.
    #[serde(rename = "created_by")]
    pub created_by: String,
    /// Whether this rate limit is active.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Maximum input tokens per rolling minute shared by all matching traffic, or per counter for a template rate limit. Null when this rule does not limit input tokens.
    #[serde(
        rename = "input_tokens_per_minute",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub input_tokens_per_minute: Option<Option<i64>>,
    /// Whether the selector gives each matching value its own allowance, that is, whether it has at least one per_value entry. Returned even when scope_restricted hides the selector.
    #[serde(rename = "is_template")]
    pub is_template: Option<bool>,
    /// Action taken when request rate exceeds the limit.
    #[serde(rename = "mode")]
    pub mode: crate::datadogV2::model::AiGatewayRateLimitMode,
    /// Datadog monitor that alerts on this rule. Null until one is provisioned.
    #[serde(
        rename = "monitor_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub monitor_id: Option<Option<i64>>,
    /// Rate limit name.
    #[serde(rename = "name")]
    pub name: String,
    /// Maximum output tokens per rolling minute shared by all matching traffic, or per counter for a template rate limit. Null when this rule does not limit output tokens.
    #[serde(
        rename = "output_tokens_per_minute",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub output_tokens_per_minute: Option<Option<i64>>,
    /// Maximum requests per rolling minute shared by all matching traffic, or per counter for a template rate limit. Null when this rule does not limit requests.
    #[serde(
        rename = "requests_per_minute",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub requests_per_minute: Option<Option<i64>>,
    /// Increases on every change to the definition.
    #[serde(rename = "revision")]
    pub revision: i64,
    /// Whether provider or model scope metadata was withheld. Omit unchanged scope fields on an unrelated update to preserve their saved values.
    #[serde(rename = "scope_restricted")]
    pub scope_restricted: bool,
    /// Dimension rules for a template rate limit, which gives each matched user, Team, or value its own allowance with the rule's limits. Can't be combined with user_id, team_id, or tags, and can't be changed after creation. Omit for a rule with one shared allowance.
    #[serde(
        rename = "selector",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub selector: Option<Option<Vec<crate::datadogV2::model::AiGatewayRateLimitSelectorEntry>>>,
    /// Optional scope tags, such as provider, model, ml_app, or custom dimensions. A request matches when it has all of these tags.
    #[serde(rename = "tags")]
    pub tags: std::collections::BTreeMap<String, String>,
    /// Optional Team UUID. Count only requests attributed to this Datadog Team.
    #[serde(
        rename = "team_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub team_id: Option<Option<uuid::Uuid>>,
    /// Dimensions of the selector's per_value entries, such as user_id or tag:env, each matching value of which gets its own allowance. Empty unless is_template is true. Returned even when scope_restricted hides the selector.
    #[serde(rename = "template_split_dims")]
    pub template_split_dims: Option<Vec<String>>,
    /// Last modification timestamp.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Email of the user who last modified this rate limit.
    #[serde(rename = "updated_by")]
    pub updated_by: String,
    /// Optional user UUID. Count only this user's requests.
    #[serde(
        rename = "user_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub user_id: Option<Option<uuid::Uuid>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayRateLimitAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: String,
        enabled: bool,
        mode: crate::datadogV2::model::AiGatewayRateLimitMode,
        name: String,
        revision: i64,
        scope_restricted: bool,
        tags: std::collections::BTreeMap<String, String>,
        updated_at: chrono::DateTime<chrono::Utc>,
        updated_by: String,
    ) -> AiGatewayRateLimitAttributes {
        AiGatewayRateLimitAttributes {
            created_at,
            created_by,
            enabled,
            input_tokens_per_minute: None,
            is_template: None,
            mode,
            monitor_id: None,
            name,
            output_tokens_per_minute: None,
            requests_per_minute: None,
            revision,
            scope_restricted,
            selector: None,
            tags,
            team_id: None,
            template_split_dims: None,
            updated_at,
            updated_by,
            user_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn input_tokens_per_minute(mut self, value: Option<i64>) -> Self {
        self.input_tokens_per_minute = Some(value);
        self
    }

    pub fn is_template(mut self, value: bool) -> Self {
        self.is_template = Some(value);
        self
    }

    pub fn monitor_id(mut self, value: Option<i64>) -> Self {
        self.monitor_id = Some(value);
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

    pub fn selector(
        mut self,
        value: Option<Vec<crate::datadogV2::model::AiGatewayRateLimitSelectorEntry>>,
    ) -> Self {
        self.selector = Some(value);
        self
    }

    pub fn team_id(mut self, value: Option<uuid::Uuid>) -> Self {
        self.team_id = Some(value);
        self
    }

    pub fn template_split_dims(mut self, value: Vec<String>) -> Self {
        self.template_split_dims = Some(value);
        self
    }

    pub fn user_id(mut self, value: Option<uuid::Uuid>) -> Self {
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

impl<'de> Deserialize<'de> for AiGatewayRateLimitAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayRateLimitAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayRateLimitAttributesVisitor {
            type Value = AiGatewayRateLimitAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut enabled: Option<bool> = None;
                let mut input_tokens_per_minute: Option<Option<i64>> = None;
                let mut is_template: Option<bool> = None;
                let mut mode: Option<crate::datadogV2::model::AiGatewayRateLimitMode> = None;
                let mut monitor_id: Option<Option<i64>> = None;
                let mut name: Option<String> = None;
                let mut output_tokens_per_minute: Option<Option<i64>> = None;
                let mut requests_per_minute: Option<Option<i64>> = None;
                let mut revision: Option<i64> = None;
                let mut scope_restricted: Option<bool> = None;
                let mut selector: Option<
                    Option<Vec<crate::datadogV2::model::AiGatewayRateLimitSelectorEntry>>,
                > = None;
                let mut tags: Option<std::collections::BTreeMap<String, String>> = None;
                let mut team_id: Option<Option<uuid::Uuid>> = None;
                let mut template_split_dims: Option<Vec<String>> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut updated_by: Option<String> = None;
                let mut user_id: Option<Option<uuid::Uuid>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "input_tokens_per_minute" => {
                            input_tokens_per_minute =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_template" => {
                            if v.is_null() {
                                continue;
                            }
                            is_template =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mode" => {
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
                        "monitor_id" => {
                            monitor_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "revision" => {
                            revision = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope_restricted" => {
                            scope_restricted =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "selector" => {
                            selector = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team_id" => {
                            team_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "template_split_dims" => {
                            if v.is_null() {
                                continue;
                            }
                            template_split_dims =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_by" => {
                            updated_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let mode = mode.ok_or_else(|| M::Error::missing_field("mode"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let revision = revision.ok_or_else(|| M::Error::missing_field("revision"))?;
                let scope_restricted =
                    scope_restricted.ok_or_else(|| M::Error::missing_field("scope_restricted"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;
                let updated_by = updated_by.ok_or_else(|| M::Error::missing_field("updated_by"))?;

                let content = AiGatewayRateLimitAttributes {
                    created_at,
                    created_by,
                    enabled,
                    input_tokens_per_minute,
                    is_template,
                    mode,
                    monitor_id,
                    name,
                    output_tokens_per_minute,
                    requests_per_minute,
                    revision,
                    scope_restricted,
                    selector,
                    tags,
                    team_id,
                    template_split_dims,
                    updated_at,
                    updated_by,
                    user_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayRateLimitAttributesVisitor)
    }
}
