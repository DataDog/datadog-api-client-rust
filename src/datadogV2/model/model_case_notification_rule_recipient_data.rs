// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Recipient data
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CaseNotificationRuleRecipientData {
    /// Slack channel name
    #[serde(rename = "channel")]
    pub channel: Option<String>,
    /// Slack channel ID
    #[serde(rename = "channel_id")]
    pub channel_id: Option<String>,
    /// Microsoft Teams channel name
    #[serde(rename = "channel_name")]
    pub channel_name: Option<String>,
    /// Microsoft Teams connector name
    #[serde(rename = "connector_name")]
    pub connector_name: Option<String>,
    /// Email address
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// HTTP webhook name
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// PagerDuty service name
    #[serde(rename = "service_name")]
    pub service_name: Option<String>,
    /// Microsoft Teams team ID
    #[serde(rename = "team_id")]
    pub team_id: Option<String>,
    /// Microsoft Teams team name
    #[serde(rename = "team_name")]
    pub team_name: Option<String>,
    /// Microsoft Teams tenant ID
    #[serde(rename = "tenant_id")]
    pub tenant_id: Option<String>,
    /// Microsoft Teams tenant name
    #[serde(rename = "tenant_name")]
    pub tenant_name: Option<String>,
    /// Slack workspace name
    #[serde(rename = "workspace")]
    pub workspace: Option<String>,
    /// Slack workspace ID
    #[serde(rename = "workspace_id")]
    pub workspace_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CaseNotificationRuleRecipientData {
    pub fn new() -> CaseNotificationRuleRecipientData {
        CaseNotificationRuleRecipientData {
            channel: None,
            channel_id: None,
            channel_name: None,
            connector_name: None,
            email: None,
            name: None,
            service_name: None,
            team_id: None,
            team_name: None,
            tenant_id: None,
            tenant_name: None,
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

    pub fn email(mut self, value: String) -> Self {
        self.email = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
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

impl Default for CaseNotificationRuleRecipientData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CaseNotificationRuleRecipientData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CaseNotificationRuleRecipientDataVisitor;
        impl<'a> Visitor<'a> for CaseNotificationRuleRecipientDataVisitor {
            type Value = CaseNotificationRuleRecipientData;

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
                let mut email: Option<String> = None;
                let mut name: Option<String> = None;
                let mut service_name: Option<String> = None;
                let mut team_id: Option<String> = None;
                let mut team_name: Option<String> = None;
                let mut tenant_id: Option<String> = None;
                let mut tenant_name: Option<String> = None;
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
                        "email" => {
                            if v.is_null() {
                                continue;
                            }
                            email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = CaseNotificationRuleRecipientData {
                    channel,
                    channel_id,
                    channel_name,
                    connector_name,
                    email,
                    name,
                    service_name,
                    team_id,
                    team_name,
                    tenant_id,
                    tenant_name,
                    workspace,
                    workspace_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CaseNotificationRuleRecipientDataVisitor)
    }
}
