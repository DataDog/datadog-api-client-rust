// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a fleet schedule in the v2 API response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetScheduleV2Attributes {
    /// RFC3339 timestamp when the schedule was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// User handle of the person who created the schedule.
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    /// Whether this is the default schedule for the organization.
    #[serde(rename = "is_default")]
    pub is_default: Option<bool>,
    /// Human-readable name for the schedule.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// RFC3339 timestamp of the next scheduled maintenance window start time.
    /// Absent when the next run time cannot be computed.
    #[serde(rename = "next_run")]
    pub next_run: Option<String>,
    /// Notification configuration attached to a schedule.
    ///
    /// Included when available. If the notification rule cannot be retrieved, this field is
    /// omitted and the schedule is still returned. If the notification rule is retrieved but its
    /// handles cannot be resolved, it is still included with an empty `handles` array.
    #[serde(rename = "notification_rule")]
    pub notification_rule: Option<crate::datadogV2::model::FleetScheduleV2NotificationRule>,
    /// Query used to filter and select target hosts for scheduled deployments.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Defines the recurrence pattern for the schedule.
    #[serde(rename = "rule")]
    pub rule: Option<crate::datadogV2::model::FleetScheduleV2RecurrenceRule>,
    /// The status of the schedule.
    /// - `active`: The schedule is active and will create deployments according to its recurrence rule.
    /// - `inactive`: The schedule is inactive and will not create any deployments.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::FleetScheduleStatus>,
    /// RFC3339 timestamp when the schedule was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    /// User handle of the person who last updated the schedule.
    #[serde(rename = "updated_by")]
    pub updated_by: Option<String>,
    /// Number of major versions behind the latest to target for upgrades.
    /// - 0: Always upgrade to the latest version.
    /// - 1: Upgrade to latest minus 1 major version.
    /// - 2: Upgrade to latest minus 2 major versions.
    #[serde(rename = "version_to_latest")]
    pub version_to_latest: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetScheduleV2Attributes {
    pub fn new() -> FleetScheduleV2Attributes {
        FleetScheduleV2Attributes {
            created_at: None,
            created_by: None,
            is_default: None,
            name: None,
            next_run: None,
            notification_rule: None,
            query: None,
            rule: None,
            status: None,
            updated_at: None,
            updated_by: None,
            version_to_latest: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: String) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: String) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn next_run(mut self, value: String) -> Self {
        self.next_run = Some(value);
        self
    }

    pub fn notification_rule(
        mut self,
        value: crate::datadogV2::model::FleetScheduleV2NotificationRule,
    ) -> Self {
        self.notification_rule = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn rule(mut self, value: crate::datadogV2::model::FleetScheduleV2RecurrenceRule) -> Self {
        self.rule = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV2::model::FleetScheduleStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: String) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn updated_by(mut self, value: String) -> Self {
        self.updated_by = Some(value);
        self
    }

    pub fn version_to_latest(mut self, value: i64) -> Self {
        self.version_to_latest = Some(value);
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

impl Default for FleetScheduleV2Attributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetScheduleV2Attributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetScheduleV2AttributesVisitor;
        impl<'a> Visitor<'a> for FleetScheduleV2AttributesVisitor {
            type Value = FleetScheduleV2Attributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<String> = None;
                let mut created_by: Option<String> = None;
                let mut is_default: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut next_run: Option<String> = None;
                let mut notification_rule: Option<
                    crate::datadogV2::model::FleetScheduleV2NotificationRule,
                > = None;
                let mut query: Option<String> = None;
                let mut rule: Option<crate::datadogV2::model::FleetScheduleV2RecurrenceRule> = None;
                let mut status: Option<crate::datadogV2::model::FleetScheduleStatus> = None;
                let mut updated_at: Option<String> = None;
                let mut updated_by: Option<String> = None;
                let mut version_to_latest: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_default" => {
                            if v.is_null() {
                                continue;
                            }
                            is_default = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "next_run" => {
                            if v.is_null() {
                                continue;
                            }
                            next_run = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notification_rule" => {
                            if v.is_null() {
                                continue;
                            }
                            notification_rule =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule" => {
                            if v.is_null() {
                                continue;
                            }
                            rule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::FleetScheduleStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_by" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version_to_latest" => {
                            if v.is_null() {
                                continue;
                            }
                            version_to_latest =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FleetScheduleV2Attributes {
                    created_at,
                    created_by,
                    is_default,
                    name,
                    next_run,
                    notification_rule,
                    query,
                    rule,
                    status,
                    updated_at,
                    updated_by,
                    version_to_latest,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetScheduleV2AttributesVisitor)
    }
}
