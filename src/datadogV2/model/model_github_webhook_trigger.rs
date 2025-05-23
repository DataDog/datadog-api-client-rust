// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Trigger a workflow from a GitHub webhook. To trigger a workflow from GitHub, you must set a `webhookSecret`. In your GitHub Webhook Settings, set the Payload URL to "base_url"/api/v2/workflows/"workflow_id"/webhook?orgId="org_id", select application/json for the content type, and be highly recommend enabling SSL verification for security. The workflow must be published.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GithubWebhookTrigger {
    /// Defines a rate limit for a trigger.
    #[serde(rename = "rateLimit")]
    pub rate_limit: Option<crate::datadogV2::model::TriggerRateLimit>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GithubWebhookTrigger {
    pub fn new() -> GithubWebhookTrigger {
        GithubWebhookTrigger {
            rate_limit: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn rate_limit(mut self, value: crate::datadogV2::model::TriggerRateLimit) -> Self {
        self.rate_limit = Some(value);
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

impl Default for GithubWebhookTrigger {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GithubWebhookTrigger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GithubWebhookTriggerVisitor;
        impl<'a> Visitor<'a> for GithubWebhookTriggerVisitor {
            type Value = GithubWebhookTrigger;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut rate_limit: Option<crate::datadogV2::model::TriggerRateLimit> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "rateLimit" => {
                            if v.is_null() {
                                continue;
                            }
                            rate_limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = GithubWebhookTrigger {
                    rate_limit,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GithubWebhookTriggerVisitor)
    }
}
