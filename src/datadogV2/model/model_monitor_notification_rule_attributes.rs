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
    /// Use bundle config to enable alert bundling to reduce monitor signal noises. **Note**: This feature is in preview and is subject to change.
    /// If you have any feedback, contact [Datadog support](<https://docs.datadoghq.com/help/>).
    #[serde(rename = "bundle_config")]
    pub bundle_config: Option<crate::datadogV2::model::MonitorNotificationRuleBundleConfig>,
    /// Use conditional recipients to define different recipients for different situations. Cannot be used with `recipients`.
    #[serde(rename = "conditional_recipients")]
    pub conditional_recipients:
        Option<crate::datadogV2::model::MonitorNotificationRuleConditionalRecipients>,
    /// Specifies the matching criteria for monitor notifications.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::MonitorNotificationRuleFilter>,
    /// The name of the monitor notification rule.
    #[serde(rename = "name")]
    pub name: String,
    /// A list of recipients to notify. Uses the same format as the monitor `message` field. Must not start with an '@'. Cannot be used with `conditional_recipients`.
    #[serde(rename = "recipients")]
    pub recipients: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorNotificationRuleAttributes {
    pub fn new(name: String) -> MonitorNotificationRuleAttributes {
        MonitorNotificationRuleAttributes {
            bundle_config: None,
            conditional_recipients: None,
            filter: None,
            name,
            recipients: None,
            _unparsed: false,
        }
    }

    pub fn bundle_config(
        mut self,
        value: crate::datadogV2::model::MonitorNotificationRuleBundleConfig,
    ) -> Self {
        self.bundle_config = Some(value);
        self
    }

    pub fn conditional_recipients(
        mut self,
        value: crate::datadogV2::model::MonitorNotificationRuleConditionalRecipients,
    ) -> Self {
        self.conditional_recipients = Some(value);
        self
    }

    pub fn filter(mut self, value: crate::datadogV2::model::MonitorNotificationRuleFilter) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn recipients(mut self, value: Vec<String>) -> Self {
        self.recipients = Some(value);
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
                let mut bundle_config: Option<
                    crate::datadogV2::model::MonitorNotificationRuleBundleConfig,
                > = None;
                let mut conditional_recipients: Option<
                    crate::datadogV2::model::MonitorNotificationRuleConditionalRecipients,
                > = None;
                let mut filter: Option<crate::datadogV2::model::MonitorNotificationRuleFilter> =
                    None;
                let mut name: Option<String> = None;
                let mut recipients: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bundle_config" => {
                            if v.is_null() {
                                continue;
                            }
                            bundle_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "conditional_recipients" => {
                            if v.is_null() {
                                continue;
                            }
                            conditional_recipients =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "recipients" => {
                            if v.is_null() {
                                continue;
                            }
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

                let content = MonitorNotificationRuleAttributes {
                    bundle_config,
                    conditional_recipients,
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
