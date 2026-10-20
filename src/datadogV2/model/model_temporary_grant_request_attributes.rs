// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Request history and the linked grant state. List and detail reads include grant_state for approved requests. Decision responses record the decision; read the request for current grant state.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TemporaryGrantRequestAttributes {
    /// When the request was submitted.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// When the decision or cancellation was recorded.
    #[serde(rename = "decided_at")]
    pub decided_at: Option<chrono::DateTime<chrono::Utc>>,
    /// UUID of the reviewer, or requester when canceled.
    #[serde(rename = "decided_by_user_id")]
    pub decided_by_user_id: Option<uuid::Uuid>,
    /// The requester identity resolved by the server.
    #[serde(rename = "entity_key")]
    pub entity_key: String,
    /// Immutable exclusive expiry, normalized to UTC microsecond precision.
    #[serde(rename = "expires_at")]
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /// Current state of the grant associated with an approved request. Replaced means a successor superseded this grant; the successor need not still be active. Absent for requests that were not approved.
    #[serde(rename = "grant_state")]
    pub grant_state: Option<crate::datadogV2::model::TemporaryGrantRequestGrantState>,
    /// Reason for requesting temporary credits. Trimmed and limited to 4096 UTF-8 bytes.
    #[serde(rename = "justification")]
    pub justification: String,
    /// The quota namespace. Only AI credits are supported.
    #[serde(rename = "quota_namespace")]
    pub quota_namespace: String,
    /// UUID of the authenticated requester.
    #[serde(rename = "requested_by_user_id")]
    pub requested_by_user_id: uuid::Uuid,
    /// The durable request decision, independent of whether its grant is still active.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::TemporaryGrantRequestStatus,
    /// Target monthly AI-credit limit in positive whole credits, not additional credits. Fractional credits are not accepted.
    #[serde(rename = "total")]
    pub total: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TemporaryGrantRequestAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        entity_key: String,
        expires_at: chrono::DateTime<chrono::Utc>,
        justification: String,
        quota_namespace: String,
        requested_by_user_id: uuid::Uuid,
        status: crate::datadogV2::model::TemporaryGrantRequestStatus,
        total: i64,
    ) -> TemporaryGrantRequestAttributes {
        TemporaryGrantRequestAttributes {
            created_at,
            decided_at: None,
            decided_by_user_id: None,
            entity_key,
            expires_at,
            grant_state: None,
            justification,
            quota_namespace,
            requested_by_user_id,
            status,
            total,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn decided_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.decided_at = Some(value);
        self
    }

    pub fn decided_by_user_id(mut self, value: uuid::Uuid) -> Self {
        self.decided_by_user_id = Some(value);
        self
    }

    pub fn grant_state(
        mut self,
        value: crate::datadogV2::model::TemporaryGrantRequestGrantState,
    ) -> Self {
        self.grant_state = Some(value);
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

impl<'de> Deserialize<'de> for TemporaryGrantRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TemporaryGrantRequestAttributesVisitor;
        impl<'a> Visitor<'a> for TemporaryGrantRequestAttributesVisitor {
            type Value = TemporaryGrantRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut decided_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut decided_by_user_id: Option<uuid::Uuid> = None;
                let mut entity_key: Option<String> = None;
                let mut expires_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut grant_state: Option<
                    crate::datadogV2::model::TemporaryGrantRequestGrantState,
                > = None;
                let mut justification: Option<String> = None;
                let mut quota_namespace: Option<String> = None;
                let mut requested_by_user_id: Option<uuid::Uuid> = None;
                let mut status: Option<crate::datadogV2::model::TemporaryGrantRequestStatus> = None;
                let mut total: Option<i64> = None;
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
                        "decided_at" => {
                            if v.is_null() {
                                continue;
                            }
                            decided_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "decided_by_user_id" => {
                            if v.is_null() {
                                continue;
                            }
                            decided_by_user_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "entity_key" => {
                            entity_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expires_at" => {
                            expires_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "grant_state" => {
                            if v.is_null() {
                                continue;
                            }
                            grant_state =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _grant_state) = grant_state {
                                match _grant_state {
                                    crate::datadogV2::model::TemporaryGrantRequestGrantState::UnparsedObject(_grant_state) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "justification" => {
                            justification =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "quota_namespace" => {
                            quota_namespace =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "requested_by_user_id" => {
                            requested_by_user_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::TemporaryGrantRequestStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "total" => {
                            total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let entity_key = entity_key.ok_or_else(|| M::Error::missing_field("entity_key"))?;
                let expires_at = expires_at.ok_or_else(|| M::Error::missing_field("expires_at"))?;
                let justification =
                    justification.ok_or_else(|| M::Error::missing_field("justification"))?;
                let quota_namespace =
                    quota_namespace.ok_or_else(|| M::Error::missing_field("quota_namespace"))?;
                let requested_by_user_id = requested_by_user_id
                    .ok_or_else(|| M::Error::missing_field("requested_by_user_id"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let total = total.ok_or_else(|| M::Error::missing_field("total"))?;

                let content = TemporaryGrantRequestAttributes {
                    created_at,
                    decided_at,
                    decided_by_user_id,
                    entity_key,
                    expires_at,
                    grant_state,
                    justification,
                    quota_namespace,
                    requested_by_user_id,
                    status,
                    total,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TemporaryGrantRequestAttributesVisitor)
    }
}
