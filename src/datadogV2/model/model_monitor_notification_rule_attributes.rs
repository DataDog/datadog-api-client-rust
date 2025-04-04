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
pub struct MonitorNotificationRuleAttributes {
    /// Filter used to associate the notification rule with monitors.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::MonitorNotificationRuleFilter>,
    /// The name of the monitor notification rule.
    #[serde(rename = "name")]
    pub name: String,
    /// A list of recipients to notify. Uses the same format as the monitor `message` field. Must not start with an '@'.
    #[serde(rename = "recipients")]
    pub recipients: Vec<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorNotificationRuleAttributes {
    pub fn new(name: String, recipients: Vec<String>) -> MonitorNotificationRuleAttributes {
        MonitorNotificationRuleAttributes {
            filter: None,
            name,
            recipients,
            _unparsed: false,
        }
    }

    pub fn filter(mut self, value: crate::datadogV2::model::MonitorNotificationRuleFilter) -> Self {
        self.filter = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for MonitorNotificationRuleAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorNotificationRuleAttributesVisitor;
        impl<'a> Visitor<'a> for MonitorNotificationRuleAttributesVisitor {
            type Value = MonitorNotificationRuleAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filter: Option<crate::datadogV2::model::MonitorNotificationRuleFilter> =
                    None;
                let mut name: Option<String> = None;
                let mut recipients: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "recipients" => {
                            recipients = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let recipients = recipients.ok_or_else(|| M::Error::missing_field("recipients"))?;

                let content = MonitorNotificationRuleAttributes {
                    filter,
                    name,
                    recipients,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorNotificationRuleAttributesVisitor)
    }
}
