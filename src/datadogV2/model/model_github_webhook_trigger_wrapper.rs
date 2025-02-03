// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Schema for a GitHub webhook-based trigger.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GithubWebhookTriggerWrapper {
    /// Trigger a workflow VIA GitHub webhook. The workflow must be published.
    #[serde(rename = "githubWebhookTrigger")]
    pub github_webhook_trigger: crate::datadogV2::model::GithubWebhookTrigger,
    /// A list of steps that run first after a trigger fires.
    #[serde(rename = "startStepNames")]
    pub start_step_names: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GithubWebhookTriggerWrapper {
    pub fn new(
        github_webhook_trigger: crate::datadogV2::model::GithubWebhookTrigger,
        start_step_names: Vec<String>,
    ) -> GithubWebhookTriggerWrapper {
        GithubWebhookTriggerWrapper {
            github_webhook_trigger,
            start_step_names,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for GithubWebhookTriggerWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GithubWebhookTriggerWrapperVisitor;
        impl<'a> Visitor<'a> for GithubWebhookTriggerWrapperVisitor {
            type Value = GithubWebhookTriggerWrapper;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut github_webhook_trigger: Option<
                    crate::datadogV2::model::GithubWebhookTrigger,
                > = None;
                let mut start_step_names: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "githubWebhookTrigger" => {
                            github_webhook_trigger =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "startStepNames" => {
                            start_step_names =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let github_webhook_trigger = github_webhook_trigger
                    .ok_or_else(|| M::Error::missing_field("github_webhook_trigger"))?;
                let start_step_names =
                    start_step_names.ok_or_else(|| M::Error::missing_field("start_step_names"))?;

                let content = GithubWebhookTriggerWrapper {
                    github_webhook_trigger,
                    start_step_names,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GithubWebhookTriggerWrapperVisitor)
    }
}
