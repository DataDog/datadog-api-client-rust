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
pub struct NotificationRuleAttributes {
    /// Date as Unix timestamp in milliseconds.
    #[serde(rename = "created_at")]
    pub created_at: i64,
    /// User creating or modifying a rule.
    #[serde(rename = "created_by")]
    pub created_by: crate::datadogV2::model::RuleUser,
    /// Field used to enable or disable the rule.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Date as Unix timestamp in milliseconds.
    #[serde(rename = "modified_at")]
    pub modified_at: i64,
    /// User creating or modifying a rule.
    #[serde(rename = "modified_by")]
    pub modified_by: crate::datadogV2::model::RuleUser,
    /// Name of the notification rule.
    #[serde(rename = "name")]
    pub name: String,
    /// Selectors are used to filter security issues for which notifications should be generated.
    /// Users can specify rule severities, rule types, a query to filter security issues on tags and attributes, and the trigger source.
    /// Only the trigger_source field is required.
    #[serde(rename = "selectors")]
    pub selectors: crate::datadogV2::model::Selectors,
    /// List of recipients to notify when a notification rule is triggered. Many different target types are supported,
    /// such as email addresses, Slack channels, and PagerDuty services.
    /// The appropriate integrations need to be properly configured to send notifications to the specified targets.
    #[serde(rename = "targets")]
    pub targets: Vec<String>,
    /// Time aggregation period (in seconds) is used to aggregate the results of the notification rule evaluation.
    /// Results are aggregated over a selected time frame using a rolling window, which updates with each new evaluation.
    /// Notifications are only sent for new issues discovered during the window.
    /// Time aggregation is only available for vulnerability-based notification rules. When omitted or set to 0, no aggregation
    /// is done.
    #[serde(rename = "time_aggregation")]
    pub time_aggregation: Option<i64>,
    /// Version of the notification rule. It is updated when the rule is modified.
    #[serde(rename = "version")]
    pub version: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotificationRuleAttributes {
    pub fn new(
        created_at: i64,
        created_by: crate::datadogV2::model::RuleUser,
        enabled: bool,
        modified_at: i64,
        modified_by: crate::datadogV2::model::RuleUser,
        name: String,
        selectors: crate::datadogV2::model::Selectors,
        targets: Vec<String>,
        version: i64,
    ) -> NotificationRuleAttributes {
        NotificationRuleAttributes {
            created_at,
            created_by,
            enabled,
            modified_at,
            modified_by,
            name,
            selectors,
            targets,
            time_aggregation: None,
            version,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn time_aggregation(mut self, value: i64) -> Self {
        self.time_aggregation = Some(value);
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

impl<'de> Deserialize<'de> for NotificationRuleAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotificationRuleAttributesVisitor;
        impl<'a> Visitor<'a> for NotificationRuleAttributesVisitor {
            type Value = NotificationRuleAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<i64> = None;
                let mut created_by: Option<crate::datadogV2::model::RuleUser> = None;
                let mut enabled: Option<bool> = None;
                let mut modified_at: Option<i64> = None;
                let mut modified_by: Option<crate::datadogV2::model::RuleUser> = None;
                let mut name: Option<String> = None;
                let mut selectors: Option<crate::datadogV2::model::Selectors> = None;
                let mut targets: Option<Vec<String>> = None;
                let mut time_aggregation: Option<i64> = None;
                let mut version: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_by" => {
                            modified_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "selectors" => {
                            selectors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "targets" => {
                            targets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time_aggregation" => {
                            if v.is_null() {
                                continue;
                            }
                            time_aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let modified_by =
                    modified_by.ok_or_else(|| M::Error::missing_field("modified_by"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let selectors = selectors.ok_or_else(|| M::Error::missing_field("selectors"))?;
                let targets = targets.ok_or_else(|| M::Error::missing_field("targets"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = NotificationRuleAttributes {
                    created_at,
                    created_by,
                    enabled,
                    modified_at,
                    modified_by,
                    name,
                    selectors,
                    targets,
                    time_aggregation,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotificationRuleAttributesVisitor)
    }
}
