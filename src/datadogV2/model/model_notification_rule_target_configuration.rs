// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for a notification target. Which fields apply depends on the target's `type`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotificationRuleTargetConfiguration {
    /// Slack channel name, for a `SLACK_CHANNEL` target.
    #[serde(rename = "channel")]
    pub channel: Option<String>,
    /// Slack channel ID, for a `SLACK_CHANNEL` target, or Microsoft Teams channel ID, for an `MS_TEAMS_CHANNEL` target.
    #[serde(rename = "channel_id")]
    pub channel_id: Option<String>,
    /// Microsoft Teams channel name, for an `MS_TEAMS_CHANNEL` target.
    #[serde(rename = "channel_name")]
    pub channel_name: Option<String>,
    /// Microsoft Teams connector name, for an `MS_TEAMS_CHANNEL` target.
    #[serde(rename = "connector_name")]
    pub connector_name: Option<String>,
    /// Recipient for an `EMAIL` target.
    #[serde(rename = "recipient")]
    pub recipient: Option<crate::datadogV2::model::NotificationRuleTargetConfigurationRecipient>,
    /// PagerDuty service name, for a `PAGERDUTY_SERVICE` target.
    #[serde(rename = "service_name")]
    pub service_name: Option<String>,
    /// Microsoft Teams team ID, for an `MS_TEAMS_CHANNEL` target.
    #[serde(rename = "team_id")]
    pub team_id: Option<String>,
    /// Microsoft Teams team name, for an `MS_TEAMS_CHANNEL` target.
    #[serde(rename = "team_name")]
    pub team_name: Option<String>,
    /// Microsoft Teams tenant ID, for an `MS_TEAMS_CHANNEL` target.
    #[serde(rename = "tenant_id")]
    pub tenant_id: Option<String>,
    /// Microsoft Teams tenant name, for an `MS_TEAMS_CHANNEL` target.
    #[serde(rename = "tenant_name")]
    pub tenant_name: Option<String>,
    /// Slack username, for a `SLACK_USER` target.
    #[serde(rename = "username")]
    pub username: Option<String>,
    /// Name of the configured webhook, for a `WEBHOOK` target.
    #[serde(rename = "webhook_name")]
    pub webhook_name: Option<String>,
    /// Slack workspace name, for a `SLACK_CHANNEL` or `SLACK_USER` target.
    #[serde(rename = "workspace")]
    pub workspace: Option<String>,
    /// Slack workspace ID, for a `SLACK_CHANNEL` target.
    #[serde(rename = "workspace_id")]
    pub workspace_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotificationRuleTargetConfiguration {
    pub fn new() -> NotificationRuleTargetConfiguration {
        NotificationRuleTargetConfiguration {
            channel: None,
            channel_id: None,
            channel_name: None,
            connector_name: None,
            recipient: None,
            service_name: None,
            team_id: None,
            team_name: None,
            tenant_id: None,
            tenant_name: None,
            username: None,
            webhook_name: None,
            workspace: None,
            workspace_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn channel(mut self, value: String) -> Self {
        self.channel = Some(value);
        self
    }

    pub fn channel_id(mut self, value: String) -> Self {
        self.channel_id = Some(value);
        self
    }

    pub fn channel_name(mut self, value: String) -> Self {
        self.channel_name = Some(value);
        self
    }

    pub fn connector_name(mut self, value: String) -> Self {
        self.connector_name = Some(value);
        self
    }

    pub fn recipient(
        mut self,
        value: crate::datadogV2::model::NotificationRuleTargetConfigurationRecipient,
    ) -> Self {
        self.recipient = Some(value);
        self
    }

    pub fn service_name(mut self, value: String) -> Self {
        self.service_name = Some(value);
        self
    }

    pub fn team_id(mut self, value: String) -> Self {
        self.team_id = Some(value);
        self
    }

    pub fn team_name(mut self, value: String) -> Self {
        self.team_name = Some(value);
        self
    }

    pub fn tenant_id(mut self, value: String) -> Self {
        self.tenant_id = Some(value);
        self
    }

    pub fn tenant_name(mut self, value: String) -> Self {
        self.tenant_name = Some(value);
        self
    }

    pub fn username(mut self, value: String) -> Self {
        self.username = Some(value);
        self
    }

    pub fn webhook_name(mut self, value: String) -> Self {
        self.webhook_name = Some(value);
        self
    }

    pub fn workspace(mut self, value: String) -> Self {
        self.workspace = Some(value);
        self
    }

    pub fn workspace_id(mut self, value: String) -> Self {
        self.workspace_id = Some(value);
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

impl Default for NotificationRuleTargetConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for NotificationRuleTargetConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotificationRuleTargetConfigurationVisitor;
        impl<'a> Visitor<'a> for NotificationRuleTargetConfigurationVisitor {
            type Value = NotificationRuleTargetConfiguration;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut channel: Option<String> = None;
                let mut channel_id: Option<String> = None;
                let mut channel_name: Option<String> = None;
                let mut connector_name: Option<String> = None;
                let mut recipient: Option<
                    crate::datadogV2::model::NotificationRuleTargetConfigurationRecipient,
                > = None;
                let mut service_name: Option<String> = None;
                let mut team_id: Option<String> = None;
                let mut team_name: Option<String> = None;
                let mut tenant_id: Option<String> = None;
                let mut tenant_name: Option<String> = None;
                let mut username: Option<String> = None;
                let mut webhook_name: Option<String> = None;
                let mut workspace: Option<String> = None;
                let mut workspace_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "channel" => {
                            if v.is_null() {
                                continue;
                            }
                            channel = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "channel_id" => {
                            if v.is_null() {
                                continue;
                            }
                            channel_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "channel_name" => {
                            if v.is_null() {
                                continue;
                            }
                            channel_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "connector_name" => {
                            if v.is_null() {
                                continue;
                            }
                            connector_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "recipient" => {
                            if v.is_null() {
                                continue;
                            }
                            recipient = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service_name" => {
                            if v.is_null() {
                                continue;
                            }
                            service_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team_id" => {
                            if v.is_null() {
                                continue;
                            }
                            team_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team_name" => {
                            if v.is_null() {
                                continue;
                            }
                            team_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tenant_id" => {
                            if v.is_null() {
                                continue;
                            }
                            tenant_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tenant_name" => {
                            if v.is_null() {
                                continue;
                            }
                            tenant_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "username" => {
                            if v.is_null() {
                                continue;
                            }
                            username = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "webhook_name" => {
                            if v.is_null() {
                                continue;
                            }
                            webhook_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "workspace" => {
                            if v.is_null() {
                                continue;
                            }
                            workspace = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "workspace_id" => {
                            if v.is_null() {
                                continue;
                            }
                            workspace_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = NotificationRuleTargetConfiguration {
                    channel,
                    channel_id,
                    channel_name,
                    connector_name,
                    recipient,
                    service_name,
                    team_id,
                    team_name,
                    tenant_id,
                    tenant_name,
                    username,
                    webhook_name,
                    workspace,
                    workspace_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotificationRuleTargetConfigurationVisitor)
    }
}
