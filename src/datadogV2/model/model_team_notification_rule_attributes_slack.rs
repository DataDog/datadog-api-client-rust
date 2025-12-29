// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Slack notification settings for the team
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamNotificationRuleAttributesSlack {
    /// Channel for Slack notification
    #[serde(rename = "channel")]
    pub channel: Option<String>,
    /// Workspace for Slack notification
    #[serde(rename = "workspace")]
    pub workspace: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamNotificationRuleAttributesSlack {
    pub fn new() -> TeamNotificationRuleAttributesSlack {
        TeamNotificationRuleAttributesSlack {
            channel: None,
            workspace: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn channel(mut self, value: String) -> Self {
        self.channel = Some(value);
        self
    }

    pub fn workspace(mut self, value: String) -> Self {
        self.workspace = Some(value);
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

impl Default for TeamNotificationRuleAttributesSlack {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TeamNotificationRuleAttributesSlack {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamNotificationRuleAttributesSlackVisitor;
        impl<'a> Visitor<'a> for TeamNotificationRuleAttributesSlackVisitor {
            type Value = TeamNotificationRuleAttributesSlack;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut channel: Option<String> = None;
                let mut workspace: Option<String> = None;
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
                        "workspace" => {
                            if v.is_null() {
                                continue;
                            }
                            workspace = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TeamNotificationRuleAttributesSlack {
                    channel,
                    workspace,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamNotificationRuleAttributesSlackVisitor)
    }
}
