// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Accepted recovery request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumRecoveryCreateResponseAttributes {
    /// Creation time.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Batch discovery state. QUEUED indicates that the request is waiting for processing capacity.
    #[serde(rename = "discovery_status")]
    pub discovery_status: Option<crate::datadogV2::model::RumRecoveryDiscoveryStatus>,
    /// Batch estimate stored at creation, when present. New batches use a fresh server estimate; older batches may contain a client-supplied estimate.
    #[serde(rename = "estimated_count")]
    pub estimated_count: Option<i64>,
    /// Failed session count.
    #[serde(rename = "failed_sessions")]
    pub failed_sessions: i64,
    /// Processed session count.
    #[serde(rename = "processed_sessions")]
    pub processed_sessions: i64,
    /// Recovery request state.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::RumRecoveryRequestStatus,
    /// Total targeted sessions. Omitted while a batch total is unknown.
    #[serde(rename = "total_sessions")]
    pub total_sessions: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumRecoveryCreateResponseAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        failed_sessions: i64,
        processed_sessions: i64,
        status: crate::datadogV2::model::RumRecoveryRequestStatus,
    ) -> RumRecoveryCreateResponseAttributes {
        RumRecoveryCreateResponseAttributes {
            created_at,
            discovery_status: None,
            estimated_count: None,
            failed_sessions,
            processed_sessions,
            status,
            total_sessions: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn discovery_status(
        mut self,
        value: crate::datadogV2::model::RumRecoveryDiscoveryStatus,
    ) -> Self {
        self.discovery_status = Some(value);
        self
    }

    pub fn estimated_count(mut self, value: i64) -> Self {
        self.estimated_count = Some(value);
        self
    }

    pub fn total_sessions(mut self, value: i64) -> Self {
        self.total_sessions = Some(value);
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

impl<'de> Deserialize<'de> for RumRecoveryCreateResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumRecoveryCreateResponseAttributesVisitor;
        impl<'a> Visitor<'a> for RumRecoveryCreateResponseAttributesVisitor {
            type Value = RumRecoveryCreateResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut discovery_status: Option<
                    crate::datadogV2::model::RumRecoveryDiscoveryStatus,
                > = None;
                let mut estimated_count: Option<i64> = None;
                let mut failed_sessions: Option<i64> = None;
                let mut processed_sessions: Option<i64> = None;
                let mut status: Option<crate::datadogV2::model::RumRecoveryRequestStatus> = None;
                let mut total_sessions: Option<i64> = None;
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
                        "discovery_status" => {
                            if v.is_null() {
                                continue;
                            }
                            discovery_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _discovery_status) = discovery_status {
                                match _discovery_status {
                                    crate::datadogV2::model::RumRecoveryDiscoveryStatus::UnparsedObject(_discovery_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "estimated_count" => {
                            if v.is_null() {
                                continue;
                            }
                            estimated_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "failed_sessions" => {
                            failed_sessions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "processed_sessions" => {
                            processed_sessions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::RumRecoveryRequestStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "total_sessions" => {
                            if v.is_null() {
                                continue;
                            }
                            total_sessions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let failed_sessions =
                    failed_sessions.ok_or_else(|| M::Error::missing_field("failed_sessions"))?;
                let processed_sessions = processed_sessions
                    .ok_or_else(|| M::Error::missing_field("processed_sessions"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = RumRecoveryCreateResponseAttributes {
                    created_at,
                    discovery_status,
                    estimated_count,
                    failed_sessions,
                    processed_sessions,
                    status,
                    total_sessions,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumRecoveryCreateResponseAttributesVisitor)
    }
}
