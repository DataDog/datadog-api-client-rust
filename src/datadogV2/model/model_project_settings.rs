// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Project settings
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProjectSettings {
    /// Auto-close inactive cases settings
    #[serde(rename = "auto_close_inactive_cases")]
    pub auto_close_inactive_cases: Option<crate::datadogV2::model::AutoCloseInactiveCases>,
    /// Auto-transition assigned cases settings
    #[serde(rename = "auto_transition_assigned_cases")]
    pub auto_transition_assigned_cases:
        Option<crate::datadogV2::model::AutoTransitionAssignedCases>,
    /// Incident integration settings
    #[serde(rename = "integration_incident")]
    pub integration_incident: Option<crate::datadogV2::model::IntegrationIncident>,
    /// Jira integration settings
    #[serde(rename = "integration_jira")]
    pub integration_jira: Option<crate::datadogV2::model::IntegrationJira>,
    /// Monitor integration settings
    #[serde(rename = "integration_monitor")]
    pub integration_monitor: Option<crate::datadogV2::model::IntegrationMonitor>,
    /// On-Call integration settings
    #[serde(rename = "integration_on_call")]
    pub integration_on_call: Option<crate::datadogV2::model::IntegrationOnCall>,
    /// ServiceNow integration settings
    #[serde(rename = "integration_service_now")]
    pub integration_service_now: Option<crate::datadogV2::model::IntegrationServiceNow>,
    /// Project notification settings
    #[serde(rename = "notification")]
    pub notification: Option<crate::datadogV2::model::ProjectNotificationSettings>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProjectSettings {
    pub fn new() -> ProjectSettings {
        ProjectSettings {
            auto_close_inactive_cases: None,
            auto_transition_assigned_cases: None,
            integration_incident: None,
            integration_jira: None,
            integration_monitor: None,
            integration_on_call: None,
            integration_service_now: None,
            notification: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn auto_close_inactive_cases(
        mut self,
        value: crate::datadogV2::model::AutoCloseInactiveCases,
    ) -> Self {
        self.auto_close_inactive_cases = Some(value);
        self
    }

    pub fn auto_transition_assigned_cases(
        mut self,
        value: crate::datadogV2::model::AutoTransitionAssignedCases,
    ) -> Self {
        self.auto_transition_assigned_cases = Some(value);
        self
    }

    pub fn integration_incident(
        mut self,
        value: crate::datadogV2::model::IntegrationIncident,
    ) -> Self {
        self.integration_incident = Some(value);
        self
    }

    pub fn integration_jira(mut self, value: crate::datadogV2::model::IntegrationJira) -> Self {
        self.integration_jira = Some(value);
        self
    }

    pub fn integration_monitor(
        mut self,
        value: crate::datadogV2::model::IntegrationMonitor,
    ) -> Self {
        self.integration_monitor = Some(value);
        self
    }

    pub fn integration_on_call(
        mut self,
        value: crate::datadogV2::model::IntegrationOnCall,
    ) -> Self {
        self.integration_on_call = Some(value);
        self
    }

    pub fn integration_service_now(
        mut self,
        value: crate::datadogV2::model::IntegrationServiceNow,
    ) -> Self {
        self.integration_service_now = Some(value);
        self
    }

    pub fn notification(
        mut self,
        value: crate::datadogV2::model::ProjectNotificationSettings,
    ) -> Self {
        self.notification = Some(value);
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

impl Default for ProjectSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProjectSettings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProjectSettingsVisitor;
        impl<'a> Visitor<'a> for ProjectSettingsVisitor {
            type Value = ProjectSettings;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auto_close_inactive_cases: Option<
                    crate::datadogV2::model::AutoCloseInactiveCases,
                > = None;
                let mut auto_transition_assigned_cases: Option<
                    crate::datadogV2::model::AutoTransitionAssignedCases,
                > = None;
                let mut integration_incident: Option<crate::datadogV2::model::IntegrationIncident> =
                    None;
                let mut integration_jira: Option<crate::datadogV2::model::IntegrationJira> = None;
                let mut integration_monitor: Option<crate::datadogV2::model::IntegrationMonitor> =
                    None;
                let mut integration_on_call: Option<crate::datadogV2::model::IntegrationOnCall> =
                    None;
                let mut integration_service_now: Option<
                    crate::datadogV2::model::IntegrationServiceNow,
                > = None;
                let mut notification: Option<crate::datadogV2::model::ProjectNotificationSettings> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auto_close_inactive_cases" => {
                            if v.is_null() {
                                continue;
                            }
                            auto_close_inactive_cases =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "auto_transition_assigned_cases" => {
                            if v.is_null() {
                                continue;
                            }
                            auto_transition_assigned_cases =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_incident" => {
                            if v.is_null() {
                                continue;
                            }
                            integration_incident =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_jira" => {
                            if v.is_null() {
                                continue;
                            }
                            integration_jira =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_monitor" => {
                            if v.is_null() {
                                continue;
                            }
                            integration_monitor =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_on_call" => {
                            if v.is_null() {
                                continue;
                            }
                            integration_on_call =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_service_now" => {
                            if v.is_null() {
                                continue;
                            }
                            integration_service_now =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notification" => {
                            if v.is_null() {
                                continue;
                            }
                            notification =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ProjectSettings {
                    auto_close_inactive_cases,
                    auto_transition_assigned_cases,
                    integration_incident,
                    integration_jira,
                    integration_monitor,
                    integration_on_call,
                    integration_service_now,
                    notification,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProjectSettingsVisitor)
    }
}
