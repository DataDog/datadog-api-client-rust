// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Batch cancellation acknowledgment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumRecoveryCancelResponseAttributes {
    /// Cancellation time.
    #[serde(rename = "cancelled_at")]
    pub cancelled_at: chrono::DateTime<chrono::Utc>,
    /// Handle of the user who canceled the batch.
    #[serde(rename = "cancelled_by")]
    pub cancelled_by: String,
    /// Batch discovery state. QUEUED indicates that the request is waiting for processing capacity.
    #[serde(rename = "discovery_status")]
    pub discovery_status: crate::datadogV2::model::RumRecoveryDiscoveryStatus,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumRecoveryCancelResponseAttributes {
    pub fn new(
        cancelled_at: chrono::DateTime<chrono::Utc>,
        cancelled_by: String,
        discovery_status: crate::datadogV2::model::RumRecoveryDiscoveryStatus,
    ) -> RumRecoveryCancelResponseAttributes {
        RumRecoveryCancelResponseAttributes {
            cancelled_at,
            cancelled_by,
            discovery_status,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for RumRecoveryCancelResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumRecoveryCancelResponseAttributesVisitor;
        impl<'a> Visitor<'a> for RumRecoveryCancelResponseAttributesVisitor {
            type Value = RumRecoveryCancelResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cancelled_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut cancelled_by: Option<String> = None;
                let mut discovery_status: Option<
                    crate::datadogV2::model::RumRecoveryDiscoveryStatus,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cancelled_at" => {
                            cancelled_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cancelled_by" => {
                            cancelled_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let cancelled_at =
                    cancelled_at.ok_or_else(|| M::Error::missing_field("cancelled_at"))?;
                let cancelled_by =
                    cancelled_by.ok_or_else(|| M::Error::missing_field("cancelled_by"))?;
                let discovery_status =
                    discovery_status.ok_or_else(|| M::Error::missing_field("discovery_status"))?;

                let content = RumRecoveryCancelResponseAttributes {
                    cancelled_at,
                    cancelled_by,
                    discovery_status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumRecoveryCancelResponseAttributesVisitor)
    }
}
