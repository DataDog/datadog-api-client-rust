// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes describing a sample log generation subscription.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SampleLogGenerationSubscriptionAttributes {
    /// The identifier of the Cloud SIEM content pack the subscription targets.
    #[serde(rename = "content_pack_id")]
    pub content_pack_id: String,
    /// The time at which the subscription was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The time at which the subscription expires and stops generating logs.
    #[serde(rename = "expires_at")]
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /// Whether the subscription is currently active and generating logs.
    #[serde(rename = "is_active")]
    pub is_active: bool,
    /// The status of the subscription.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::SampleLogGenerationSubscriptionStatus,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SampleLogGenerationSubscriptionAttributes {
    pub fn new(
        content_pack_id: String,
        created_at: chrono::DateTime<chrono::Utc>,
        expires_at: chrono::DateTime<chrono::Utc>,
        is_active: bool,
        status: crate::datadogV2::model::SampleLogGenerationSubscriptionStatus,
    ) -> SampleLogGenerationSubscriptionAttributes {
        SampleLogGenerationSubscriptionAttributes {
            content_pack_id,
            created_at,
            expires_at,
            is_active,
            status,
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

impl<'de> Deserialize<'de> for SampleLogGenerationSubscriptionAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SampleLogGenerationSubscriptionAttributesVisitor;
        impl<'a> Visitor<'a> for SampleLogGenerationSubscriptionAttributesVisitor {
            type Value = SampleLogGenerationSubscriptionAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut content_pack_id: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut expires_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut is_active: Option<bool> = None;
                let mut status: Option<
                    crate::datadogV2::model::SampleLogGenerationSubscriptionStatus,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "content_pack_id" => {
                            content_pack_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expires_at" => {
                            expires_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_active" => {
                            is_active = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::SampleLogGenerationSubscriptionStatus::UnparsedObject(_status) => {
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
                let content_pack_id =
                    content_pack_id.ok_or_else(|| M::Error::missing_field("content_pack_id"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let expires_at = expires_at.ok_or_else(|| M::Error::missing_field("expires_at"))?;
                let is_active = is_active.ok_or_else(|| M::Error::missing_field("is_active"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = SampleLogGenerationSubscriptionAttributes {
                    content_pack_id,
                    created_at,
                    expires_at,
                    is_active,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SampleLogGenerationSubscriptionAttributesVisitor)
    }
}
