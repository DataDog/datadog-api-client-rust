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
pub struct SecurityMonitoringRuleCaseCreate {
    /// Action to perform for each rule case.
    #[serde(rename = "actions")]
    pub actions: Option<Vec<crate::datadogV2::model::SecurityMonitoringRuleCaseAction>>,
    /// A case contains logical operations (`>`,`>=`, `&&`, `||`) to determine if a signal should be generated
    /// based on the event counts in the previously defined queries.
    #[serde(rename = "condition")]
    pub condition: Option<String>,
    /// Name of the case.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Notification targets.
    #[serde(rename = "notifications")]
    pub notifications: Option<Vec<String>>,
    /// Severity of the Security Signal.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::SecurityMonitoringRuleSeverity,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringRuleCaseCreate {
    pub fn new(
        status: crate::datadogV2::model::SecurityMonitoringRuleSeverity,
    ) -> SecurityMonitoringRuleCaseCreate {
        SecurityMonitoringRuleCaseCreate {
            actions: None,
            condition: None,
            name: None,
            notifications: None,
            status,
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleCaseCreate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringRuleCaseCreateVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringRuleCaseCreateVisitor {
            type Value = SecurityMonitoringRuleCaseCreate;

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
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = SecurityMonitoringRuleCaseCreate {
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

        deserializer.deserialize_any(SecurityMonitoringRuleCaseCreateVisitor)
    }
}
