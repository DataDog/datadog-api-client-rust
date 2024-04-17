// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the notification rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringNotificationRuleResponseAttributes {
    /// Timestamp of creation of the notification rule in milliseconds.
    #[serde(rename = "creation_date")]
    pub creation_date: Option<i64>,
    /// The author of the notification rule.
    #[serde(rename = "creator")]
    pub creator: Option<crate::datadogV2::model::SecurityMonitoringCreator>,
    /// Whether the notification rule is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The name of the notification rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Selectors describing the notification rule.
    #[serde(rename = "selectors")]
    pub selectors: Option<crate::datadogV2::model::SecurityMonitoringNotificationRuleSelectors>,
    /// Set of targets to notify.
    #[serde(rename = "targets")]
    pub targets: Option<Vec<String>>,
    /// Timestamp of last modification of the notification rule in milliseconds.
    #[serde(rename = "update_date")]
    pub update_date: Option<i64>,
    /// The editor of the notification rule.
    #[serde(rename = "updater")]
    pub updater: Option<crate::datadogV2::model::SecurityMonitoringUpdater>,
    /// The version of the rule being updated.
    #[serde(rename = "version")]
    pub version: Option<i32>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringNotificationRuleResponseAttributes {
    pub fn new() -> SecurityMonitoringNotificationRuleResponseAttributes {
        SecurityMonitoringNotificationRuleResponseAttributes {
            creation_date: None,
            creator: None,
            enabled: None,
            name: None,
            selectors: None,
            targets: None,
            update_date: None,
            updater: None,
            version: None,
            _unparsed: false,
        }
    }

    pub fn creation_date(mut self, value: i64) -> Self {
        self.creation_date = Some(value);
        self
    }

    pub fn creator(mut self, value: crate::datadogV2::model::SecurityMonitoringCreator) -> Self {
        self.creator = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn selectors(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringNotificationRuleSelectors,
    ) -> Self {
        self.selectors = Some(value);
        self
    }

    pub fn targets(mut self, value: Vec<String>) -> Self {
        self.targets = Some(value);
        self
    }

    pub fn update_date(mut self, value: i64) -> Self {
        self.update_date = Some(value);
        self
    }

    pub fn updater(mut self, value: crate::datadogV2::model::SecurityMonitoringUpdater) -> Self {
        self.updater = Some(value);
        self
    }

    pub fn version(mut self, value: i32) -> Self {
        self.version = Some(value);
        self
    }
}

impl Default for SecurityMonitoringNotificationRuleResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringNotificationRuleResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringNotificationRuleResponseAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringNotificationRuleResponseAttributesVisitor {
            type Value = SecurityMonitoringNotificationRuleResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut creation_date: Option<i64> = None;
                let mut creator: Option<crate::datadogV2::model::SecurityMonitoringCreator> = None;
                let mut enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut selectors: Option<
                    crate::datadogV2::model::SecurityMonitoringNotificationRuleSelectors,
                > = None;
                let mut targets: Option<Vec<String>> = None;
                let mut update_date: Option<i64> = None;
                let mut updater: Option<crate::datadogV2::model::SecurityMonitoringUpdater> = None;
                let mut version: Option<i32> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "creation_date" => {
                            if v.is_null() {
                                continue;
                            }
                            creation_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creator" => {
                            if v.is_null() {
                                continue;
                            }
                            creator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "selectors" => {
                            if v.is_null() {
                                continue;
                            }
                            selectors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "targets" => {
                            if v.is_null() {
                                continue;
                            }
                            targets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "update_date" => {
                            if v.is_null() {
                                continue;
                            }
                            update_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updater" => {
                            if v.is_null() {
                                continue;
                            }
                            updater = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SecurityMonitoringNotificationRuleResponseAttributes {
                    creation_date,
                    creator,
                    enabled,
                    name,
                    selectors,
                    targets,
                    update_date,
                    updater,
                    version,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringNotificationRuleResponseAttributesVisitor)
    }
}
