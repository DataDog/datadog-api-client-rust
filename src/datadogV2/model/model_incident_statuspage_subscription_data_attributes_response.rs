// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an email subscription.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentStatuspageSubscriptionDataAttributesResponse {
    /// Whether the subscription has been confirmed.
    #[serde(rename = "confirmed")]
    pub confirmed: bool,
    /// Timestamp when the subscription was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The subscribed email address.
    #[serde(rename = "email")]
    pub email: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentStatuspageSubscriptionDataAttributesResponse {
    pub fn new(
        confirmed: bool,
        created_at: chrono::DateTime<chrono::Utc>,
        email: String,
    ) -> IncidentStatuspageSubscriptionDataAttributesResponse {
        IncidentStatuspageSubscriptionDataAttributesResponse {
            confirmed,
            created_at,
            email,
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

impl<'de> Deserialize<'de> for IncidentStatuspageSubscriptionDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentStatuspageSubscriptionDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for IncidentStatuspageSubscriptionDataAttributesResponseVisitor {
            type Value = IncidentStatuspageSubscriptionDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut confirmed: Option<bool> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut email: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "confirmed" => {
                            confirmed = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "email" => {
                            email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let confirmed = confirmed.ok_or_else(|| M::Error::missing_field("confirmed"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let email = email.ok_or_else(|| M::Error::missing_field("email"))?;

                let content = IncidentStatuspageSubscriptionDataAttributesResponse {
                    confirmed,
                    created_at,
                    email,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentStatuspageSubscriptionDataAttributesResponseVisitor)
    }
}
