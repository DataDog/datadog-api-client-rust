// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A conditional recipient rule composed of a `scope` (the matching condition) and
/// `recipients` (who to notify when it matches).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorNotificationRuleCondition {
    /// A list of recipients to notify. Uses the same format as the monitor `message` field. Must not start with an '@'. Cannot be used with `conditional_recipients`.
    #[serde(rename = "recipients")]
    pub recipients: Vec<String>,
    /// Defines the condition under which the recipients are notified. Supported formats:
    /// - Monitor status condition using `transition_type:<status>`, for example `transition_type:is_alert`.
    /// - A single tag key:value pair, for example `env:prod`.
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorNotificationRuleCondition {
    pub fn new(recipients: Vec<String>, scope: String) -> MonitorNotificationRuleCondition {
        MonitorNotificationRuleCondition {
            recipients,
            scope,
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

impl<'de> Deserialize<'de> for MonitorNotificationRuleCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorNotificationRuleConditionVisitor;
        impl<'a> Visitor<'a> for MonitorNotificationRuleConditionVisitor {
            type Value = MonitorNotificationRuleCondition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut recipients: Option<Vec<String>> = None;
                let mut scope: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "recipients" => {
                            recipients = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let recipients = recipients.ok_or_else(|| M::Error::missing_field("recipients"))?;
                let scope = scope.ok_or_else(|| M::Error::missing_field("scope"))?;

                let content = MonitorNotificationRuleCondition {
                    recipients,
                    scope,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorNotificationRuleConditionVisitor)
    }
}
