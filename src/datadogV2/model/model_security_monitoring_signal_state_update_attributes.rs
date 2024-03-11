// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes describing the change of state of a security signal.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringSignalStateUpdateAttributes {
    /// Optional comment to display on archived signals.
    #[serde(rename = "archive_comment")]
    pub archive_comment: Option<String>,
    /// Reason a signal is archived.
    #[serde(rename = "archive_reason")]
    pub archive_reason: Option<crate::datadogV2::model::SecurityMonitoringSignalArchiveReason>,
    /// The new triage state of the signal.
    #[serde(rename = "state")]
    pub state: crate::datadogV2::model::SecurityMonitoringSignalState,
    /// Version of the updated signal. If server side version is higher, update will be rejected.
    #[serde(rename = "version")]
    pub version: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringSignalStateUpdateAttributes {
    pub fn new(
        state: crate::datadogV2::model::SecurityMonitoringSignalState,
    ) -> SecurityMonitoringSignalStateUpdateAttributes {
        SecurityMonitoringSignalStateUpdateAttributes {
            archive_comment: None,
            archive_reason: None,
            state,
            version: None,
            _unparsed: false,
        }
    }

    pub fn archive_comment(&mut self, value: String) -> &mut Self {
        self.archive_comment = Some(value);
        self
    }

    pub fn archive_reason(
        &mut self,
        value: crate::datadogV2::model::SecurityMonitoringSignalArchiveReason,
    ) -> &mut Self {
        self.archive_reason = Some(value);
        self
    }

    pub fn version(&mut self, value: i64) -> &mut Self {
        self.version = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringSignalStateUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringSignalStateUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringSignalStateUpdateAttributesVisitor {
            type Value = SecurityMonitoringSignalStateUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut archive_comment: Option<String> = None;
                let mut archive_reason: Option<
                    crate::datadogV2::model::SecurityMonitoringSignalArchiveReason,
                > = None;
                let mut state: Option<crate::datadogV2::model::SecurityMonitoringSignalState> =
                    None;
                let mut version: Option<i64> = None;
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
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let state = state.ok_or_else(|| M::Error::missing_field("state"))?;

                let content = SecurityMonitoringSignalStateUpdateAttributes {
                    archive_comment,
                    archive_reason,
                    state,
                    version,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringSignalStateUpdateAttributesVisitor)
    }
}
