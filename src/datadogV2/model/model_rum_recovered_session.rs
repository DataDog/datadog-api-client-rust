// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Progress for one targeted session.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumRecoveredSession {
    /// Application UUID.
    #[serde(rename = "application_id")]
    pub application_id: uuid::Uuid,
    /// Failure details, if present.
    #[serde(rename = "error_message")]
    pub error_message: Option<String>,
    /// Recovered event count.
    #[serde(rename = "events_recovered")]
    pub events_recovered: i64,
    /// Session identifier.
    #[serde(rename = "session_id")]
    pub session_id: uuid::Uuid,
    /// Session availability or recovery state. `RECOVERED` indicates that the session is ready to query.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::RumRecoverySessionStatus,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumRecoveredSession {
    pub fn new(
        application_id: uuid::Uuid,
        events_recovered: i64,
        session_id: uuid::Uuid,
        status: crate::datadogV2::model::RumRecoverySessionStatus,
    ) -> RumRecoveredSession {
        RumRecoveredSession {
            application_id,
            error_message: None,
            events_recovered,
            session_id,
            status,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn error_message(mut self, value: String) -> Self {
        self.error_message = Some(value);
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

impl<'de> Deserialize<'de> for RumRecoveredSession {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumRecoveredSessionVisitor;
        impl<'a> Visitor<'a> for RumRecoveredSessionVisitor {
            type Value = RumRecoveredSession;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_id: Option<uuid::Uuid> = None;
                let mut error_message: Option<String> = None;
                let mut events_recovered: Option<i64> = None;
                let mut session_id: Option<uuid::Uuid> = None;
                let mut status: Option<crate::datadogV2::model::RumRecoverySessionStatus> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "application_id" => {
                            application_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_message" => {
                            if v.is_null() {
                                continue;
                            }
                            error_message =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "events_recovered" => {
                            events_recovered =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_id" => {
                            session_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::RumRecoverySessionStatus::UnparsedObject(_status) => {
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
                let application_id =
                    application_id.ok_or_else(|| M::Error::missing_field("application_id"))?;
                let events_recovered =
                    events_recovered.ok_or_else(|| M::Error::missing_field("events_recovered"))?;
                let session_id = session_id.ok_or_else(|| M::Error::missing_field("session_id"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = RumRecoveredSession {
                    application_id,
                    error_message,
                    events_recovered,
                    session_id,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumRecoveredSessionVisitor)
    }
}
