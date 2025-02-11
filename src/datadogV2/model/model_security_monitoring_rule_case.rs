// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Case when signal is generated.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringRuleCase {
    /// Action to perform for each rule case.
    #[serde(rename = "actions")]
    pub actions: Option<Vec<crate::datadogV2::model::SecurityMonitoringRuleCaseAction>>,
    /// A rule case contains logical operations (`>`,`>=`, `&&`, `||`) to determine if a signal should be generated
    /// based on the event counts in the previously defined queries.
    #[serde(rename = "condition")]
    pub condition: Option<String>,
    /// Name of the case.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Notification targets for each rule case.
    #[serde(rename = "notifications")]
    pub notifications: Option<Vec<String>>,
    /// Severity of the Security Signal.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::SecurityMonitoringRuleSeverity>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringRuleCase {
    pub fn new() -> SecurityMonitoringRuleCase {
        SecurityMonitoringRuleCase {
            actions: None,
            condition: None,
            name: None,
            notifications: None,
            status: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn actions(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringRuleCaseAction>,
    ) -> Self {
        self.actions = Some(value);
        self
    }

    pub fn condition(mut self, value: String) -> Self {
        self.condition = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn notifications(mut self, value: Vec<String>) -> Self {
        self.notifications = Some(value);
        self
    }

    pub fn status(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleSeverity,
    ) -> Self {
        self.status = Some(value);
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

impl Default for SecurityMonitoringRuleCase {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleCase {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringRuleCaseVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringRuleCaseVisitor {
            type Value = SecurityMonitoringRuleCase;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut actions: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringRuleCaseAction>,
                > = None;
                let mut condition: Option<String> = None;
                let mut name: Option<String> = None;
                let mut notifications: Option<Vec<String>> = None;
                let mut status: Option<crate::datadogV2::model::SecurityMonitoringRuleSeverity> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "actions" => {
                            if v.is_null() {
                                continue;
                            }
                            actions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "condition" => {
                            if v.is_null() {
                                continue;
                            }
                            condition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notifications" => {
                            if v.is_null() {
                                continue;
                            }
                            notifications =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::SecurityMonitoringRuleSeverity::UnparsedObject(_status) => {
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

                let content = SecurityMonitoringRuleCase {
                    actions,
                    condition,
                    name,
                    notifications,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringRuleCaseVisitor)
    }
}
