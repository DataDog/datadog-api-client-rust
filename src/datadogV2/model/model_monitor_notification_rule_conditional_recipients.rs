// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Use conditional recipients to define different recipients for different situations. Cannot be used with `recipients`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorNotificationRuleConditionalRecipients {
    /// Conditions of the notification rule.
    #[serde(rename = "conditions")]
    pub conditions: Vec<crate::datadogV2::model::MonitorNotificationRuleCondition>,
    /// A list of recipients to notify. Uses the same format as the monitor `message` field. Must not start with an '@'. Cannot be used with `conditional_recipients`.
    #[serde(rename = "fallback_recipients")]
    pub fallback_recipients: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorNotificationRuleConditionalRecipients {
    pub fn new(
        conditions: Vec<crate::datadogV2::model::MonitorNotificationRuleCondition>,
    ) -> MonitorNotificationRuleConditionalRecipients {
        MonitorNotificationRuleConditionalRecipients {
            conditions,
            fallback_recipients: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn fallback_recipients(mut self, value: Vec<String>) -> Self {
        self.fallback_recipients = Some(value);
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

impl<'de> Deserialize<'de> for MonitorNotificationRuleConditionalRecipients {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorNotificationRuleConditionalRecipientsVisitor;
        impl<'a> Visitor<'a> for MonitorNotificationRuleConditionalRecipientsVisitor {
            type Value = MonitorNotificationRuleConditionalRecipients;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut conditions: Option<
                    Vec<crate::datadogV2::model::MonitorNotificationRuleCondition>,
                > = None;
                let mut fallback_recipients: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "conditions" => {
                            conditions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fallback_recipients" => {
                            if v.is_null() {
                                continue;
                            }
                            fallback_recipients =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let conditions = conditions.ok_or_else(|| M::Error::missing_field("conditions"))?;

                let content = MonitorNotificationRuleConditionalRecipients {
                    conditions,
                    fallback_recipients,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorNotificationRuleConditionalRecipientsVisitor)
    }
}
