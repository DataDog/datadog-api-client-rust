// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Email notification settings for the team
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamNotificationRuleAttributesEmail {
    /// Flag indicating email notification
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Email address to notify. When omitted and email notifications are enabled, notifications are sent to all team members.
    #[serde(rename = "recipient_email")]
    pub recipient_email: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TeamNotificationRuleAttributesEmail {
    pub fn new() -> TeamNotificationRuleAttributesEmail {
        TeamNotificationRuleAttributesEmail {
            enabled: None,
            recipient_email: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
    
    pub fn recipient_email(mut self, value: String) -> Self {
        self.recipient_email = Some(value);
        self
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}


impl Default for TeamNotificationRuleAttributesEmail {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TeamNotificationRuleAttributesEmail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamNotificationRuleAttributesEmailVisitor;
        impl<'a> Visitor<'a> for TeamNotificationRuleAttributesEmailVisitor {
            type Value = TeamNotificationRuleAttributesEmail;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut recipient_email: Option<String> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "recipient_email" => {
                            if v.is_null() {
                                continue;
                            }
                            recipient_email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }

                let content = TeamNotificationRuleAttributesEmail {
                    enabled,
                    recipient_email,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamNotificationRuleAttributesEmailVisitor)
    }
}
