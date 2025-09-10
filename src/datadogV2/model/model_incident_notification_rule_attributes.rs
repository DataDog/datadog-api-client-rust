// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The notification rule's attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentNotificationRuleAttributes {
    /// The conditions that trigger this notification rule.
    #[serde(rename = "conditions")]
    pub conditions: Vec<crate::datadogV2::model::IncidentNotificationRuleConditionsItems>,
    /// Timestamp when the notification rule was created.
    #[serde(rename = "created")]
    pub created: chrono::DateTime<chrono::Utc>,
    /// Whether the notification rule is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The notification handles (targets) for this rule.
    #[serde(rename = "handles")]
    pub handles: Vec<String>,
    /// Timestamp when the notification rule was last modified.
    #[serde(rename = "modified")]
    pub modified: chrono::DateTime<chrono::Utc>,
    /// List of incident fields that trigger re-notification when changed.
    #[serde(rename = "renotify_on")]
    pub renotify_on: Option<Vec<String>>,
    /// The trigger event for this notification rule.
    #[serde(rename = "trigger")]
    pub trigger: String,
    /// The visibility of the notification rule.
    #[serde(rename = "visibility")]
    pub visibility: crate::datadogV2::model::IncidentNotificationRuleAttributesVisibility,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentNotificationRuleAttributes {
    pub fn new(
        conditions: Vec<crate::datadogV2::model::IncidentNotificationRuleConditionsItems>,
        created: chrono::DateTime<chrono::Utc>,
        enabled: bool,
        handles: Vec<String>,
        modified: chrono::DateTime<chrono::Utc>,
        trigger: String,
        visibility: crate::datadogV2::model::IncidentNotificationRuleAttributesVisibility,
    ) -> IncidentNotificationRuleAttributes {
        IncidentNotificationRuleAttributes {
            conditions,
            created,
            enabled,
            handles,
            modified,
            renotify_on: None,
            trigger,
            visibility,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn renotify_on(mut self, value: Vec<String>) -> Self {
        self.renotify_on = Some(value);
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

impl<'de> Deserialize<'de> for IncidentNotificationRuleAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentNotificationRuleAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentNotificationRuleAttributesVisitor {
            type Value = IncidentNotificationRuleAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut conditions: Option<
                    Vec<crate::datadogV2::model::IncidentNotificationRuleConditionsItems>,
                > = None;
                let mut created: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut enabled: Option<bool> = None;
                let mut handles: Option<Vec<String>> = None;
                let mut modified: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut renotify_on: Option<Vec<String>> = None;
                let mut trigger: Option<String> = None;
                let mut visibility: Option<
                    crate::datadogV2::model::IncidentNotificationRuleAttributesVisibility,
                > = None;
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
                        "created" => {
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "handles" => {
                            handles = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "renotify_on" => {
                            if v.is_null() {
                                continue;
                            }
                            renotify_on =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trigger" => {
                            trigger = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "visibility" => {
                            visibility = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _visibility) = visibility {
                                match _visibility {
                                    crate::datadogV2::model::IncidentNotificationRuleAttributesVisibility::UnparsedObject(_visibility) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let conditions = conditions.ok_or_else(|| M::Error::missing_field("conditions"))?;
                let created = created.ok_or_else(|| M::Error::missing_field("created"))?;
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let handles = handles.ok_or_else(|| M::Error::missing_field("handles"))?;
                let modified = modified.ok_or_else(|| M::Error::missing_field("modified"))?;
                let trigger = trigger.ok_or_else(|| M::Error::missing_field("trigger"))?;
                let visibility = visibility.ok_or_else(|| M::Error::missing_field("visibility"))?;

                let content = IncidentNotificationRuleAttributes {
                    conditions,
                    created,
                    enabled,
                    handles,
                    modified,
                    renotify_on,
                    trigger,
                    visibility,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentNotificationRuleAttributesVisitor)
    }
}
