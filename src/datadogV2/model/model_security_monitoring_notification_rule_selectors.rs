// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Selectors describing the notification rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringNotificationRuleSelectors {
    /// Set of rule and signal tags for which a notification will be triggered.
    #[serde(rename = "attributes")]
    pub attributes: Vec<String>,
    /// Whether vulnerability_management rules are matched by default when the selector for rule type is empty.
    #[serde(rename = "implicit_vm_rule_match")]
    pub implicit_vm_rule_match: bool,
    /// True if the notification_rule was created with signal tags/attributes and migrated to an event query and false if it was created with an event query
    #[serde(rename = "migrated")]
    pub migrated: Option<bool>,
    /// Query for matching the notification_rule against signal content
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Set of rule tags for which a notification will be triggered.
    #[serde(rename = "rule_tags")]
    pub rule_tags: Vec<String>,
    /// Set of signal types (rule types) for which a notification will be triggered.
    #[serde(rename = "rule_types")]
    pub rule_types: Vec<crate::datadogV2::model::SecurityMonitoringRuleTypes>,
    /// Set of signal severities (rule case statuses) for which a notification will be triggered.
    #[serde(rename = "severities")]
    pub severities: Vec<crate::datadogV2::model::SecurityMonitoringRuleSeverity>,
    /// Set of signal tags for which a notification will be triggered.
    #[serde(rename = "signal_tags")]
    pub signal_tags: Vec<String>,
    /// Specifies the evaluation result (Signal or Finding).
    #[serde(rename = "trigger_source")]
    pub trigger_source:
        Option<crate::datadogV2::model::SecurityMonitoringNotificationRuleTriggerSource>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringNotificationRuleSelectors {
    pub fn new(
        attributes: Vec<String>,
        implicit_vm_rule_match: bool,
        rule_tags: Vec<String>,
        rule_types: Vec<crate::datadogV2::model::SecurityMonitoringRuleTypes>,
        severities: Vec<crate::datadogV2::model::SecurityMonitoringRuleSeverity>,
        signal_tags: Vec<String>,
    ) -> SecurityMonitoringNotificationRuleSelectors {
        SecurityMonitoringNotificationRuleSelectors {
            attributes,
            implicit_vm_rule_match,
            migrated: None,
            query: None,
            rule_tags,
            rule_types,
            severities,
            signal_tags,
            trigger_source: None,
            _unparsed: false,
        }
    }

    pub fn migrated(mut self, value: bool) -> Self {
        self.migrated = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn trigger_source(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringNotificationRuleTriggerSource,
    ) -> Self {
        self.trigger_source = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringNotificationRuleSelectors {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringNotificationRuleSelectorsVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringNotificationRuleSelectorsVisitor {
            type Value = SecurityMonitoringNotificationRuleSelectors;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<Vec<String>> = None;
                let mut implicit_vm_rule_match: Option<bool> = None;
                let mut migrated: Option<bool> = None;
                let mut query: Option<String> = None;
                let mut rule_tags: Option<Vec<String>> = None;
                let mut rule_types: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringRuleTypes>,
                > = None;
                let mut severities: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringRuleSeverity>,
                > = None;
                let mut signal_tags: Option<Vec<String>> = None;
                let mut trigger_source: Option<
                    crate::datadogV2::model::SecurityMonitoringNotificationRuleTriggerSource,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "implicit_vm_rule_match" => {
                            implicit_vm_rule_match =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "migrated" => {
                            if v.is_null() {
                                continue;
                            }
                            migrated = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_tags" => {
                            rule_tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_types" => {
                            rule_types = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "severities" => {
                            severities = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "signal_tags" => {
                            signal_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trigger_source" => {
                            if v.is_null() {
                                continue;
                            }
                            trigger_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _trigger_source) = trigger_source {
                                match _trigger_source {
                                    crate::datadogV2::model::SecurityMonitoringNotificationRuleTriggerSource::UnparsedObject(_trigger_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let attributes = attributes.ok_or_else(|| M::Error::missing_field("attributes"))?;
                let implicit_vm_rule_match = implicit_vm_rule_match
                    .ok_or_else(|| M::Error::missing_field("implicit_vm_rule_match"))?;
                let rule_tags = rule_tags.ok_or_else(|| M::Error::missing_field("rule_tags"))?;
                let rule_types = rule_types.ok_or_else(|| M::Error::missing_field("rule_types"))?;
                let severities = severities.ok_or_else(|| M::Error::missing_field("severities"))?;
                let signal_tags =
                    signal_tags.ok_or_else(|| M::Error::missing_field("signal_tags"))?;

                let content = SecurityMonitoringNotificationRuleSelectors {
                    attributes,
                    implicit_vm_rule_match,
                    migrated,
                    query,
                    rule_tags,
                    rule_types,
                    severities,
                    signal_tags,
                    trigger_source,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringNotificationRuleSelectorsVisitor)
    }
}
