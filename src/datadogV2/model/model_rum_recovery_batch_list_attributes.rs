// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Batch recovery progress. Status updates can appear here before they appear in request details.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumRecoveryBatchListAttributes {
    /// Creation time.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Batch discovery state. QUEUED indicates that the request is waiting for processing capacity.
    #[serde(rename = "discovery_status")]
    pub discovery_status: crate::datadogV2::model::RumRecoveryDiscoveryStatus,
    /// Batch estimate stored at creation, when present. New batches use a fresh server estimate; older batches may contain a client-supplied estimate.
    #[serde(rename = "estimated_count")]
    pub estimated_count: Option<i64>,
    /// Start of the range in Unix milliseconds. Must be positive and earlier than to.
    #[serde(rename = "from")]
    pub from: i64,
    /// RUM search query. Not blank, at most 4096 bytes for batch creation and count.
    #[serde(rename = "query")]
    pub query: String,
    /// Discovered sessions.
    #[serde(rename = "sessions_discovered")]
    pub sessions_discovered: i64,
    /// Failed sessions.
    #[serde(rename = "sessions_failed")]
    pub sessions_failed: i64,
    /// Processed sessions.
    #[serde(rename = "sessions_processed")]
    pub sessions_processed: i64,
    /// Completed slices.
    #[serde(rename = "slices_completed")]
    pub slices_completed: i64,
    /// Failed slices.
    #[serde(rename = "slices_failed")]
    pub slices_failed: i64,
    /// Recovery request state.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::RumRecoveryRequestStatus,
    /// End of the range in Unix milliseconds.
    #[serde(rename = "to")]
    pub to: i64,
    /// Total discovery slices.
    #[serde(rename = "total_slices")]
    pub total_slices: i64,
    /// Last update time.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumRecoveryBatchListAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        discovery_status: crate::datadogV2::model::RumRecoveryDiscoveryStatus,
        from: i64,
        query: String,
        sessions_discovered: i64,
        sessions_failed: i64,
        sessions_processed: i64,
        slices_completed: i64,
        slices_failed: i64,
        status: crate::datadogV2::model::RumRecoveryRequestStatus,
        to: i64,
        total_slices: i64,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> RumRecoveryBatchListAttributes {
        RumRecoveryBatchListAttributes {
            created_at,
            discovery_status,
            estimated_count: None,
            from,
            query,
            sessions_discovered,
            sessions_failed,
            sessions_processed,
            slices_completed,
            slices_failed,
            status,
            to,
            total_slices,
            updated_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn estimated_count(mut self, value: i64) -> Self {
        self.estimated_count = Some(value);
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

impl<'de> Deserialize<'de> for RumRecoveryBatchListAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumRecoveryBatchListAttributesVisitor;
        impl<'a> Visitor<'a> for RumRecoveryBatchListAttributesVisitor {
            type Value = RumRecoveryBatchListAttributes;

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
                let mut from: Option<i64> = None;
                let mut query: Option<String> = None;
                let mut sessions_discovered: Option<i64> = None;
                let mut sessions_failed: Option<i64> = None;
                let mut sessions_processed: Option<i64> = None;
                let mut slices_completed: Option<i64> = None;
                let mut slices_failed: Option<i64> = None;
                let mut status: Option<crate::datadogV2::model::RumRecoveryRequestStatus> = None;
                let mut to: Option<i64> = None;
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
                        "from" => {
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sessions_discovered" => {
                            sessions_discovered =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sessions_failed" => {
                            sessions_failed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sessions_processed" => {
                            sessions_processed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "slices_completed" => {
                            slices_completed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "slices_failed" => {
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
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_slices" => {
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
                let discovery_status =
                    discovery_status.ok_or_else(|| M::Error::missing_field("discovery_status"))?;
                let from = from.ok_or_else(|| M::Error::missing_field("from"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let sessions_discovered = sessions_discovered
                    .ok_or_else(|| M::Error::missing_field("sessions_discovered"))?;
                let sessions_failed =
                    sessions_failed.ok_or_else(|| M::Error::missing_field("sessions_failed"))?;
                let sessions_processed = sessions_processed
                    .ok_or_else(|| M::Error::missing_field("sessions_processed"))?;
                let slices_completed =
                    slices_completed.ok_or_else(|| M::Error::missing_field("slices_completed"))?;
                let slices_failed =
                    slices_failed.ok_or_else(|| M::Error::missing_field("slices_failed"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let to = to.ok_or_else(|| M::Error::missing_field("to"))?;
                let total_slices =
                    total_slices.ok_or_else(|| M::Error::missing_field("total_slices"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = RumRecoveryBatchListAttributes {
                    created_at,
                    discovery_status,
                    estimated_count,
                    from,
                    query,
                    sessions_discovered,
                    sessions_failed,
                    sessions_processed,
                    slices_completed,
                    slices_failed,
                    status,
                    to,
                    total_slices,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumRecoveryBatchListAttributesVisitor)
    }
}
