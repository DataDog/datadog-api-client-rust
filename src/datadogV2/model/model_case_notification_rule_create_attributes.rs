// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Notification rule creation attributes
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CaseNotificationRuleCreateAttributes {
    /// Whether the notification rule is enabled
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Query to filter cases for this notification rule
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// List of notification recipients
    #[serde(rename = "recipients")]
    pub recipients: Vec<crate::datadogV2::model::CaseNotificationRuleRecipient>,
    /// List of triggers for this notification rule
    #[serde(rename = "triggers")]
    pub triggers: Vec<crate::datadogV2::model::CaseNotificationRuleTrigger>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CaseNotificationRuleCreateAttributes {
    pub fn new(
        recipients: Vec<crate::datadogV2::model::CaseNotificationRuleRecipient>,
        triggers: Vec<crate::datadogV2::model::CaseNotificationRuleTrigger>,
    ) -> CaseNotificationRuleCreateAttributes {
        CaseNotificationRuleCreateAttributes {
            is_enabled: None,
            query: None,
            recipients,
            triggers,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
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

impl<'de> Deserialize<'de> for CaseNotificationRuleCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CaseNotificationRuleCreateAttributesVisitor;
        impl<'a> Visitor<'a> for CaseNotificationRuleCreateAttributesVisitor {
            type Value = CaseNotificationRuleCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_enabled: Option<bool> = None;
                let mut query: Option<String> = None;
                let mut recipients: Option<
                    Vec<crate::datadogV2::model::CaseNotificationRuleRecipient>,
                > = None;
                let mut triggers: Option<
                    Vec<crate::datadogV2::model::CaseNotificationRuleTrigger>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "is_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "recipients" => {
                            recipients = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "triggers" => {
                            triggers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let recipients = recipients.ok_or_else(|| M::Error::missing_field("recipients"))?;
                let triggers = triggers.ok_or_else(|| M::Error::missing_field("triggers"))?;

                let content = CaseNotificationRuleCreateAttributes {
                    is_enabled,
                    query,
                    recipients,
                    triggers,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CaseNotificationRuleCreateAttributesVisitor)
    }
}
