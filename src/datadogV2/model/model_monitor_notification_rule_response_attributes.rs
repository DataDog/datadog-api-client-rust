// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the monitor notification rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorNotificationRuleResponseAttributes {
    /// Use conditional recipients to define different recipients for different situations.
    #[serde(rename = "conditional_recipients")]
    pub conditional_recipients:
        Option<crate::datadogV2::model::MonitorNotificationRuleConditionalRecipients>,
    /// Creation time of the monitor notification rule.
    #[serde(rename = "created")]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// Filter used to associate the notification rule with monitors.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::MonitorNotificationRuleFilter>,
    /// Time the monitor notification rule was last modified.
    #[serde(rename = "modified")]
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
    /// The name of the monitor notification rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// A list of recipients to notify. Uses the same format as the monitor `message` field. Must not start with an '@'.
    #[serde(rename = "recipients")]
    pub recipients: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorNotificationRuleResponseAttributes {
    pub fn new() -> MonitorNotificationRuleResponseAttributes {
        MonitorNotificationRuleResponseAttributes {
            conditional_recipients: None,
            created: None,
            filter: None,
            modified: None,
            name: None,
            recipients: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn conditional_recipients(
        mut self,
        value: crate::datadogV2::model::MonitorNotificationRuleConditionalRecipients,
    ) -> Self {
        self.conditional_recipients = Some(value);
        self
    }

    pub fn created(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created = Some(value);
        self
    }

    pub fn filter(mut self, value: crate::datadogV2::model::MonitorNotificationRuleFilter) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn modified(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn recipients(mut self, value: Vec<String>) -> Self {
        self.recipients = Some(value);
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

impl Default for MonitorNotificationRuleResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorNotificationRuleResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorNotificationRuleResponseAttributesVisitor;
        impl<'a> Visitor<'a> for MonitorNotificationRuleResponseAttributesVisitor {
            type Value = MonitorNotificationRuleResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut conditional_recipients: Option<
                    crate::datadogV2::model::MonitorNotificationRuleConditionalRecipients,
                > = None;
                let mut created: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut filter: Option<crate::datadogV2::model::MonitorNotificationRuleFilter> =
                    None;
                let mut modified: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut recipients: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "conditional_recipients" => {
                            if v.is_null() {
                                continue;
                            }
                            conditional_recipients =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created" => {
                            if v.is_null() {
                                continue;
                            }
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _filter) = filter {
                                match _filter {
                                    crate::datadogV2::model::MonitorNotificationRuleFilter::UnparsedObject(_filter) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "modified" => {
                            if v.is_null() {
                                continue;
                            }
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "recipients" => {
                            if v.is_null() {
                                continue;
                            }
                            recipients = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = MonitorNotificationRuleResponseAttributes {
                    conditional_recipients,
                    created,
                    filter,
                    modified,
                    name,
                    recipients,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorNotificationRuleResponseAttributesVisitor)
    }
}
