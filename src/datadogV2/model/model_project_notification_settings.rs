// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Project notification settings
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProjectNotificationSettings {
    /// Notification destinations (1=email, 2=slack, 3=in-app)
    #[serde(rename = "destinations")]
    pub destinations: Option<Vec<i32>>,
    /// Whether notifications are enabled
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "notify_on_case_assignment")]
    pub notify_on_case_assignment: Option<bool>,
    #[serde(rename = "notify_on_case_closed")]
    pub notify_on_case_closed: Option<bool>,
    #[serde(rename = "notify_on_case_comment")]
    pub notify_on_case_comment: Option<bool>,
    #[serde(rename = "notify_on_case_comment_mention")]
    pub notify_on_case_comment_mention: Option<bool>,
    #[serde(rename = "notify_on_case_priority_change")]
    pub notify_on_case_priority_change: Option<bool>,
    #[serde(rename = "notify_on_case_status_change")]
    pub notify_on_case_status_change: Option<bool>,
    #[serde(rename = "notify_on_case_unassignment")]
    pub notify_on_case_unassignment: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProjectNotificationSettings {
    pub fn new() -> ProjectNotificationSettings {
        ProjectNotificationSettings {
            destinations: None,
            enabled: None,
            notify_on_case_assignment: None,
            notify_on_case_closed: None,
            notify_on_case_comment: None,
            notify_on_case_comment_mention: None,
            notify_on_case_priority_change: None,
            notify_on_case_status_change: None,
            notify_on_case_unassignment: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn destinations(mut self, value: Vec<i32>) -> Self {
        self.destinations = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn notify_on_case_assignment(mut self, value: bool) -> Self {
        self.notify_on_case_assignment = Some(value);
        self
    }

    pub fn notify_on_case_closed(mut self, value: bool) -> Self {
        self.notify_on_case_closed = Some(value);
        self
    }

    pub fn notify_on_case_comment(mut self, value: bool) -> Self {
        self.notify_on_case_comment = Some(value);
        self
    }

    pub fn notify_on_case_comment_mention(mut self, value: bool) -> Self {
        self.notify_on_case_comment_mention = Some(value);
        self
    }

    pub fn notify_on_case_priority_change(mut self, value: bool) -> Self {
        self.notify_on_case_priority_change = Some(value);
        self
    }

    pub fn notify_on_case_status_change(mut self, value: bool) -> Self {
        self.notify_on_case_status_change = Some(value);
        self
    }

    pub fn notify_on_case_unassignment(mut self, value: bool) -> Self {
        self.notify_on_case_unassignment = Some(value);
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

impl Default for ProjectNotificationSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProjectNotificationSettings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProjectNotificationSettingsVisitor;
        impl<'a> Visitor<'a> for ProjectNotificationSettingsVisitor {
            type Value = ProjectNotificationSettings;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut destinations: Option<Vec<i32>> = None;
                let mut enabled: Option<bool> = None;
                let mut notify_on_case_assignment: Option<bool> = None;
                let mut notify_on_case_closed: Option<bool> = None;
                let mut notify_on_case_comment: Option<bool> = None;
                let mut notify_on_case_comment_mention: Option<bool> = None;
                let mut notify_on_case_priority_change: Option<bool> = None;
                let mut notify_on_case_status_change: Option<bool> = None;
                let mut notify_on_case_unassignment: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "destinations" => {
                            if v.is_null() {
                                continue;
                            }
                            destinations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notify_on_case_assignment" => {
                            if v.is_null() {
                                continue;
                            }
                            notify_on_case_assignment =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notify_on_case_closed" => {
                            if v.is_null() {
                                continue;
                            }
                            notify_on_case_closed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notify_on_case_comment" => {
                            if v.is_null() {
                                continue;
                            }
                            notify_on_case_comment =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notify_on_case_comment_mention" => {
                            if v.is_null() {
                                continue;
                            }
                            notify_on_case_comment_mention =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notify_on_case_priority_change" => {
                            if v.is_null() {
                                continue;
                            }
                            notify_on_case_priority_change =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notify_on_case_status_change" => {
                            if v.is_null() {
                                continue;
                            }
                            notify_on_case_status_change =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notify_on_case_unassignment" => {
                            if v.is_null() {
                                continue;
                            }
                            notify_on_case_unassignment =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ProjectNotificationSettings {
                    destinations,
                    enabled,
                    notify_on_case_assignment,
                    notify_on_case_closed,
                    notify_on_case_comment,
                    notify_on_case_comment_mention,
                    notify_on_case_priority_change,
                    notify_on_case_status_change,
                    notify_on_case_unassignment,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProjectNotificationSettingsVisitor)
    }
}
