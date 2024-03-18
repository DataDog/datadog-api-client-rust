// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes describing a triage state update operation over a security signal.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringSignalTriageAttributes {
    /// Optional comment to display on archived signals.
    #[serde(rename = "archive_comment")]
    pub archive_comment: Option<String>,
    /// Timestamp of the last edit to the comment.
    #[serde(rename = "archive_comment_timestamp")]
    pub archive_comment_timestamp: Option<i64>,
    /// Object representing a given user entity.
    #[serde(rename = "archive_comment_user")]
    pub archive_comment_user: Option<crate::datadogV2::model::SecurityMonitoringTriageUser>,
    /// Reason a signal is archived.
    #[serde(rename = "archive_reason")]
    pub archive_reason: Option<crate::datadogV2::model::SecurityMonitoringSignalArchiveReason>,
    /// Object representing a given user entity.
    #[serde(rename = "assignee")]
    pub assignee: crate::datadogV2::model::SecurityMonitoringTriageUser,
    /// Array of incidents that are associated with this signal.
    #[serde(rename = "incident_ids")]
    pub incident_ids: Vec<i64>,
    /// The new triage state of the signal.
    #[serde(rename = "state")]
    pub state: crate::datadogV2::model::SecurityMonitoringSignalState,
    /// Timestamp of the last update to the signal state.
    #[serde(rename = "state_update_timestamp")]
    pub state_update_timestamp: Option<i64>,
    /// Object representing a given user entity.
    #[serde(rename = "state_update_user")]
    pub state_update_user: Option<crate::datadogV2::model::SecurityMonitoringTriageUser>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringSignalTriageAttributes {
    pub fn new(
        assignee: crate::datadogV2::model::SecurityMonitoringTriageUser,
        incident_ids: Vec<i64>,
        state: crate::datadogV2::model::SecurityMonitoringSignalState,
    ) -> SecurityMonitoringSignalTriageAttributes {
        SecurityMonitoringSignalTriageAttributes {
            archive_comment: None,
            archive_comment_timestamp: None,
            archive_comment_user: None,
            archive_reason: None,
            assignee,
            incident_ids,
            state,
            state_update_timestamp: None,
            state_update_user: None,
            _unparsed: false,
        }
    }

    pub fn archive_comment(mut self, value: String) -> Self {
        self.archive_comment = Some(value);
        self
    }

    pub fn archive_comment_timestamp(mut self, value: i64) -> Self {
        self.archive_comment_timestamp = Some(value);
        self
    }

    pub fn archive_comment_user(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringTriageUser,
    ) -> Self {
        self.archive_comment_user = Some(value);
        self
    }

    pub fn archive_reason(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringSignalArchiveReason,
    ) -> Self {
        self.archive_reason = Some(value);
        self
    }

    pub fn state_update_timestamp(mut self, value: i64) -> Self {
        self.state_update_timestamp = Some(value);
        self
    }

    pub fn state_update_user(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringTriageUser,
    ) -> Self {
        self.state_update_user = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringSignalTriageAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringSignalTriageAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringSignalTriageAttributesVisitor {
            type Value = SecurityMonitoringSignalTriageAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut archive_comment: Option<String> = None;
                let mut archive_comment_timestamp: Option<i64> = None;
                let mut archive_comment_user: Option<
                    crate::datadogV2::model::SecurityMonitoringTriageUser,
                > = None;
                let mut archive_reason: Option<
                    crate::datadogV2::model::SecurityMonitoringSignalArchiveReason,
                > = None;
                let mut assignee: Option<crate::datadogV2::model::SecurityMonitoringTriageUser> =
                    None;
                let mut incident_ids: Option<Vec<i64>> = None;
                let mut state: Option<crate::datadogV2::model::SecurityMonitoringSignalState> =
                    None;
                let mut state_update_timestamp: Option<i64> = None;
                let mut state_update_user: Option<
                    crate::datadogV2::model::SecurityMonitoringTriageUser,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "archive_comment" => {
                            if v.is_null() {
                                continue;
                            }
                            archive_comment =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "archive_comment_timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            archive_comment_timestamp =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "archive_comment_user" => {
                            if v.is_null() {
                                continue;
                            }
                            archive_comment_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "archive_reason" => {
                            if v.is_null() {
                                continue;
                            }
                            archive_reason =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _archive_reason) = archive_reason {
                                match _archive_reason {
                                    crate::datadogV2::model::SecurityMonitoringSignalArchiveReason::UnparsedObject(_archive_reason) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "assignee" => {
                            assignee = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incident_ids" => {
                            incident_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _state) = state {
                                match _state {
                                    crate::datadogV2::model::SecurityMonitoringSignalState::UnparsedObject(_state) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "state_update_timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            state_update_timestamp =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state_update_user" => {
                            if v.is_null() {
                                continue;
                            }
                            state_update_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let assignee = assignee.ok_or_else(|| M::Error::missing_field("assignee"))?;
                let incident_ids =
                    incident_ids.ok_or_else(|| M::Error::missing_field("incident_ids"))?;
                let state = state.ok_or_else(|| M::Error::missing_field("state"))?;

                let content = SecurityMonitoringSignalTriageAttributes {
                    archive_comment,
                    archive_comment_timestamp,
                    archive_comment_user,
                    archive_reason,
                    assignee,
                    incident_ids,
                    state,
                    state_update_timestamp,
                    state_update_user,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringSignalTriageAttributesVisitor)
    }
}
