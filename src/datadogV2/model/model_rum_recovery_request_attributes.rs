// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Recovery progress. Batch-only attributes are omitted for targeted requests; request_type is also omitted for targeted requests.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumRecoveryRequestAttributes {
    /// Creation time.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Batch discovery state. QUEUED indicates that the request is waiting for processing capacity.
    #[serde(rename = "discovery_status")]
    pub discovery_status: Option<crate::datadogV2::model::RumRecoveryDiscoveryStatus>,
    /// Failure details, if present.
    #[serde(rename = "error_message")]
    pub error_message: Option<String>,
    /// Batch estimate stored at creation, when present. New batches use a fresh server estimate; older batches may contain a client-supplied estimate.
    #[serde(rename = "estimated_count")]
    pub estimated_count: Option<i64>,
    /// Failed session count.
    #[serde(rename = "failed_sessions")]
    pub failed_sessions: i64,
    /// Start of the range in Unix milliseconds. Must be positive and earlier than to.
    #[serde(rename = "from")]
    pub from: Option<i64>,
    /// Processed session count.
    #[serde(rename = "processed_sessions")]
    pub processed_sessions: i64,
    /// RUM search query. Not blank, at most 4096 bytes for batch creation and count.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Query-based batch recovery.
    #[serde(rename = "request_type")]
    pub request_type: Option<crate::datadogV2::model::RumRecoveryBatchRequestType>,
    /// Targeted session progress. Omitted when empty or not applicable.
    #[serde(rename = "sessions")]
    pub sessions: Option<Vec<crate::datadogV2::model::RumRecoveredSession>>,
    /// Discovered sessions.
    #[serde(rename = "sessions_discovered")]
    pub sessions_discovered: Option<i64>,
    /// Failed sessions.
    #[serde(rename = "sessions_failed")]
    pub sessions_failed: Option<i64>,
    /// Processed sessions.
    #[serde(rename = "sessions_processed")]
    pub sessions_processed: Option<i64>,
    /// Completed slices.
    #[serde(rename = "slices_completed")]
    pub slices_completed: Option<i64>,
    /// Failed slices.
    #[serde(rename = "slices_failed")]
    pub slices_failed: Option<i64>,
    /// Recovery request state.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::RumRecoveryRequestStatus,
    /// End of the range in Unix milliseconds.
    #[serde(rename = "to")]
    pub to: Option<i64>,
    /// Total targeted sessions. Omitted while a batch total is unknown.
    #[serde(rename = "total_sessions")]
    pub total_sessions: Option<i64>,
    /// Total discovery slices.
    #[serde(rename = "total_slices")]
    pub total_slices: Option<i64>,
    /// Last update time.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumRecoveryRequestAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        failed_sessions: i64,
        processed_sessions: i64,
        status: crate::datadogV2::model::RumRecoveryRequestStatus,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> RumRecoveryRequestAttributes {
        RumRecoveryRequestAttributes {
            created_at,
            discovery_status: None,
            error_message: None,
            estimated_count: None,
            failed_sessions,
            from: None,
            processed_sessions,
            query: None,
            request_type: None,
            sessions: None,
            sessions_discovered: None,
            sessions_failed: None,
            sessions_processed: None,
            slices_completed: None,
            slices_failed: None,
            status,
            to: None,
            total_sessions: None,
            total_slices: None,
            updated_at,
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

    pub fn error_message(mut self, value: String) -> Self {
        self.error_message = Some(value);
        self
    }

    pub fn estimated_count(mut self, value: i64) -> Self {
        self.estimated_count = Some(value);
        self
    }

    pub fn from(mut self, value: i64) -> Self {
        self.from = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn request_type(
        mut self,
        value: crate::datadogV2::model::RumRecoveryBatchRequestType,
    ) -> Self {
        self.request_type = Some(value);
        self
    }

    pub fn sessions(mut self, value: Vec<crate::datadogV2::model::RumRecoveredSession>) -> Self {
        self.sessions = Some(value);
        self
    }

    pub fn sessions_discovered(mut self, value: i64) -> Self {
        self.sessions_discovered = Some(value);
        self
    }

    pub fn sessions_failed(mut self, value: i64) -> Self {
        self.sessions_failed = Some(value);
        self
    }

    pub fn sessions_processed(mut self, value: i64) -> Self {
        self.sessions_processed = Some(value);
        self
    }

    pub fn slices_completed(mut self, value: i64) -> Self {
        self.slices_completed = Some(value);
        self
    }

    pub fn slices_failed(mut self, value: i64) -> Self {
        self.slices_failed = Some(value);
        self
    }

    pub fn to(mut self, value: i64) -> Self {
        self.to = Some(value);
        self
    }

    pub fn total_sessions(mut self, value: i64) -> Self {
        self.total_sessions = Some(value);
        self
    }

    pub fn total_slices(mut self, value: i64) -> Self {
        self.total_slices = Some(value);
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

impl<'de> Deserialize<'de> for RumRecoveryRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumRecoveryRequestAttributesVisitor;
        impl<'a> Visitor<'a> for RumRecoveryRequestAttributesVisitor {
            type Value = RumRecoveryRequestAttributes;

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
                let mut error_message: Option<String> = None;
                let mut estimated_count: Option<i64> = None;
                let mut failed_sessions: Option<i64> = None;
                let mut from: Option<i64> = None;
                let mut processed_sessions: Option<i64> = None;
                let mut query: Option<String> = None;
                let mut request_type: Option<crate::datadogV2::model::RumRecoveryBatchRequestType> =
                    None;
                let mut sessions: Option<Vec<crate::datadogV2::model::RumRecoveredSession>> = None;
                let mut sessions_discovered: Option<i64> = None;
                let mut sessions_failed: Option<i64> = None;
                let mut sessions_processed: Option<i64> = None;
                let mut slices_completed: Option<i64> = None;
                let mut slices_failed: Option<i64> = None;
                let mut status: Option<crate::datadogV2::model::RumRecoveryRequestStatus> = None;
                let mut to: Option<i64> = None;
                let mut total_sessions: Option<i64> = None;
                let mut total_slices: Option<i64> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
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
                        "error_message" => {
                            if v.is_null() {
                                continue;
                            }
                            error_message =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "from" => {
                            if v.is_null() {
                                continue;
                            }
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "processed_sessions" => {
                            processed_sessions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "request_type" => {
                            if v.is_null() {
                                continue;
                            }
                            request_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _request_type) = request_type {
                                match _request_type {
                                    crate::datadogV2::model::RumRecoveryBatchRequestType::UnparsedObject(_request_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "sessions" => {
                            if v.is_null() {
                                continue;
                            }
                            sessions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sessions_discovered" => {
                            if v.is_null() {
                                continue;
                            }
                            sessions_discovered =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sessions_failed" => {
                            if v.is_null() {
                                continue;
                            }
                            sessions_failed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sessions_processed" => {
                            if v.is_null() {
                                continue;
                            }
                            sessions_processed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "slices_completed" => {
                            if v.is_null() {
                                continue;
                            }
                            slices_completed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "slices_failed" => {
                            if v.is_null() {
                                continue;
                            }
                            slices_failed =
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
                        "to" => {
                            if v.is_null() {
                                continue;
                            }
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_sessions" => {
                            if v.is_null() {
                                continue;
                            }
                            total_sessions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_slices" => {
                            if v.is_null() {
                                continue;
                            }
                            total_slices =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = RumRecoveryRequestAttributes {
                    created_at,
                    discovery_status,
                    error_message,
                    estimated_count,
                    failed_sessions,
                    from,
                    processed_sessions,
                    query,
                    request_type,
                    sessions,
                    sessions_discovered,
                    sessions_failed,
                    sessions_processed,
                    slices_completed,
                    slices_failed,
                    status,
                    to,
                    total_sessions,
                    total_slices,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumRecoveryRequestAttributesVisitor)
    }
}
