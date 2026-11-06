// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Archived session metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumArchiveSessionAttributes {
    /// Session account id, or null when unavailable.
    #[serialize_always]
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// Session anonymous id, or null when unavailable.
    #[serialize_always]
    #[serde(rename = "anonymous_id")]
    pub anonymous_id: Option<String>,
    /// Application UUID.
    #[serde(rename = "application_id")]
    pub application_id: uuid::Uuid,
    /// Session country, or null when unavailable.
    #[serialize_always]
    #[serde(rename = "country")]
    pub country: Option<String>,
    /// Session recovery state. A missing recovery record can be represented as null. Optional request_id and recovered_at are omitted when unavailable.
    #[serialize_always]
    #[serde(rename = "recovery_status")]
    pub recovery_status: Option<crate::datadogV2::model::RumArchiveSessionRecoveryStatus>,
    /// Service name.
    #[serde(rename = "service")]
    pub service: String,
    /// Session identifier.
    #[serde(rename = "session_id")]
    pub session_id: String,
    /// Session timestamp in Unix milliseconds.
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    /// Session user id, or null when unavailable.
    #[serialize_always]
    #[serde(rename = "user_id")]
    pub user_id: Option<String>,
    /// Session version, or null when unavailable.
    #[serialize_always]
    #[serde(rename = "version")]
    pub version: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumArchiveSessionAttributes {
    pub fn new(
        account_id: Option<String>,
        anonymous_id: Option<String>,
        application_id: uuid::Uuid,
        country: Option<String>,
        recovery_status: Option<crate::datadogV2::model::RumArchiveSessionRecoveryStatus>,
        service: String,
        session_id: String,
        timestamp: i64,
        user_id: Option<String>,
        version: Option<String>,
    ) -> RumArchiveSessionAttributes {
        RumArchiveSessionAttributes {
            account_id,
            anonymous_id,
            application_id,
            country,
            recovery_status,
            service,
            session_id,
            timestamp,
            user_id,
            version,
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

impl<'de> Deserialize<'de> for RumArchiveSessionAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumArchiveSessionAttributesVisitor;
        impl<'a> Visitor<'a> for RumArchiveSessionAttributesVisitor {
            type Value = RumArchiveSessionAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<Option<String>> = None;
                let mut anonymous_id: Option<Option<String>> = None;
                let mut application_id: Option<uuid::Uuid> = None;
                let mut country: Option<Option<String>> = None;
                let mut recovery_status: Option<
                    Option<crate::datadogV2::model::RumArchiveSessionRecoveryStatus>,
                > = None;
                let mut service: Option<String> = None;
                let mut session_id: Option<String> = None;
                let mut timestamp: Option<i64> = None;
                let mut user_id: Option<Option<String>> = None;
                let mut version: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "anonymous_id" => {
                            anonymous_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "application_id" => {
                            application_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "country" => {
                            country = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "recovery_status" => {
                            recovery_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_id" => {
                            session_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp" => {
                            timestamp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_id" => {
                            user_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let account_id = account_id.ok_or_else(|| M::Error::missing_field("account_id"))?;
                let anonymous_id =
                    anonymous_id.ok_or_else(|| M::Error::missing_field("anonymous_id"))?;
                let application_id =
                    application_id.ok_or_else(|| M::Error::missing_field("application_id"))?;
                let country = country.ok_or_else(|| M::Error::missing_field("country"))?;
                let recovery_status =
                    recovery_status.ok_or_else(|| M::Error::missing_field("recovery_status"))?;
                let service = service.ok_or_else(|| M::Error::missing_field("service"))?;
                let session_id = session_id.ok_or_else(|| M::Error::missing_field("session_id"))?;
                let timestamp = timestamp.ok_or_else(|| M::Error::missing_field("timestamp"))?;
                let user_id = user_id.ok_or_else(|| M::Error::missing_field("user_id"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = RumArchiveSessionAttributes {
                    account_id,
                    anonymous_id,
                    application_id,
                    country,
                    recovery_status,
                    service,
                    session_id,
                    timestamp,
                    user_id,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumArchiveSessionAttributesVisitor)
    }
}
