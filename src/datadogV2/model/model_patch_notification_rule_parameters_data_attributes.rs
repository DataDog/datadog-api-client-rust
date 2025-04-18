// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the notification rule patch request. It is required to update the version of the rule when patching it.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PatchNotificationRuleParametersDataAttributes {
    /// Field used to enable or disable the rule.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Name of the notification rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Selectors are used to filter security issues for which notifications should be generated.
    /// Users can specify rule severities, rule types, a query to filter security issues on tags and attributes, and the trigger source.
    /// Only the trigger_source field is required.
    #[serde(rename = "selectors")]
    pub selectors: Option<crate::datadogV2::model::Selectors>,
    /// List of recipients to notify when a notification rule is triggered. Many different target types are supported,
    /// such as email addresses, Slack channels, and PagerDuty services.
    /// The appropriate integrations need to be properly configured to send notifications to the specified targets.
    #[serde(rename = "targets")]
    pub targets: Option<Vec<String>>,
    /// Time aggregation period (in seconds) is used to aggregate the results of the notification rule evaluation.
    /// Results are aggregated over a selected time frame using a rolling window, which updates with each new evaluation.
    /// Notifications are only sent for new issues discovered during the window.
    /// Time aggregation is only available for vulnerability-based notification rules. When omitted or set to 0, no aggregation
    /// is done.
    #[serde(rename = "time_aggregation")]
    pub time_aggregation: Option<i64>,
    /// Version of the notification rule. It is updated when the rule is modified.
    #[serde(rename = "version")]
    pub version: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PatchNotificationRuleParametersDataAttributes {
    pub fn new() -> PatchNotificationRuleParametersDataAttributes {
        PatchNotificationRuleParametersDataAttributes {
            enabled: None,
            name: None,
            selectors: None,
            targets: None,
            time_aggregation: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn selectors(mut self, value: crate::datadogV2::model::Selectors) -> Self {
        self.selectors = Some(value);
        self
    }

    pub fn targets(mut self, value: Vec<String>) -> Self {
        self.targets = Some(value);
        self
    }

    pub fn time_aggregation(mut self, value: i64) -> Self {
        self.time_aggregation = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
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

impl Default for PatchNotificationRuleParametersDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for PatchNotificationRuleParametersDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PatchNotificationRuleParametersDataAttributesVisitor;
        impl<'a> Visitor<'a> for PatchNotificationRuleParametersDataAttributesVisitor {
            type Value = PatchNotificationRuleParametersDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
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
                        "time_aggregation" => {
                            if v.is_null() {
                                continue;
                            }
                            time_aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = PatchNotificationRuleParametersDataAttributes {
                    enabled,
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

        deserializer.deserialize_any(PatchNotificationRuleParametersDataAttributesVisitor)
    }
}
