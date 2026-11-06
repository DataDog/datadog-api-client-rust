// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Session recovery state. A missing recovery record can be represented as null. Optional request_id and recovered_at are omitted when unavailable.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumArchiveSessionRecoveryStatus {
    /// Recovery timestamp, when available.
    #[serde(rename = "recovered_at")]
    pub recovered_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Associated recovery request ID.
    #[serde(rename = "request_id")]
    pub request_id: Option<String>,
    /// Session availability or recovery state. `RECOVERED` indicates that the session is ready to query.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::RumRecoverySessionStatus,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumArchiveSessionRecoveryStatus {
    pub fn new(
        status: crate::datadogV2::model::RumRecoverySessionStatus,
    ) -> RumArchiveSessionRecoveryStatus {
        RumArchiveSessionRecoveryStatus {
            recovered_at: None,
            request_id: None,
            status,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn recovered_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.recovered_at = Some(value);
        self
    }

    pub fn request_id(mut self, value: String) -> Self {
        self.request_id = Some(value);
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

impl<'de> Deserialize<'de> for RumArchiveSessionRecoveryStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumArchiveSessionRecoveryStatusVisitor;
        impl<'a> Visitor<'a> for RumArchiveSessionRecoveryStatusVisitor {
            type Value = RumArchiveSessionRecoveryStatus;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut recovered_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut request_id: Option<String> = None;
                let mut status: Option<crate::datadogV2::model::RumRecoverySessionStatus> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "recovered_at" => {
                            if v.is_null() {
                                continue;
                            }
                            recovered_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "request_id" => {
                            if v.is_null() {
                                continue;
                            }
                            request_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = RumArchiveSessionRecoveryStatus {
                    recovered_at,
                    request_id,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumArchiveSessionRecoveryStatusVisitor)
    }
}
