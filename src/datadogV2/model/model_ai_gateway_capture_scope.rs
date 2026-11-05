// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// All selected filters combine with AND, with one value per filter. The `model` and `provider` tags apply to each actual direct or routed attempt, including fallback destinations. Routes are not selectable filters. At least one filter is required.
/// If any matching override uses `metadata_only`, that mode applies. Otherwise, if any matching override uses `full_capture`, that mode applies. The organization default applies only when no override matches. Overrides have no priority or specificity ordering.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayCaptureScope {
    /// Exact-match request tags with lowercase keys. The reserved `model` tag is the canonical model ID, and the reserved `provider` tag is the configured connection slug. The server sets both. Custom tags come from `x-dd-tag` headers. Keys and values must not have leading or trailing white space. Values are case-sensitive.
    #[serde(rename = "tags")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
    /// UUID of a Datadog team. Matches the authorized team selected by the `x-ai-gateway-team` request header. Requests without this header do not match team-scoped overrides. Open-membership teams cannot scope overrides.
    #[serde(rename = "team_id")]
    pub team_id: Option<uuid::Uuid>,
    /// Authenticated Datadog user UUID in this organization.
    #[serde(rename = "user_id")]
    pub user_id: Option<uuid::Uuid>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayCaptureScope {
    pub fn new() -> AiGatewayCaptureScope {
        AiGatewayCaptureScope {
            tags: None,
            team_id: None,
            user_id: None,
            _unparsed: false,
        }
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
}

impl Default for AiGatewayCaptureScope {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AiGatewayCaptureScope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayCaptureScopeVisitor;
        impl<'a> Visitor<'a> for AiGatewayCaptureScopeVisitor {
            type Value = AiGatewayCaptureScope;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut tags: Option<std::collections::BTreeMap<String, String>> = None;
                let mut team_id: Option<uuid::Uuid> = None;
                let mut user_id: Option<uuid::Uuid> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = AiGatewayCaptureScope {
                    tags,
                    team_id,
                    user_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayCaptureScopeVisitor)
    }
}
