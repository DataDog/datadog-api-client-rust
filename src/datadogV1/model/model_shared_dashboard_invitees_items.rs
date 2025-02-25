// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The allowlisted invitees for an INVITE-only shared dashboard.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SharedDashboardInviteesItems {
    /// Time of the invitee expiration. Null means the invite will not expire.
    #[serde(
        rename = "access_expiration",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub access_expiration: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Time that the invitee was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Email of the invitee.
    #[serde(rename = "email")]
    pub email: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SharedDashboardInviteesItems {
    pub fn new(email: String) -> SharedDashboardInviteesItems {
        SharedDashboardInviteesItems {
            access_expiration: None,
            created_at: None,
            email,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn access_expiration(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.access_expiration = Some(value);
        self
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
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

impl<'de> Deserialize<'de> for SharedDashboardInviteesItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SharedDashboardInviteesItemsVisitor;
        impl<'a> Visitor<'a> for SharedDashboardInviteesItemsVisitor {
            type Value = SharedDashboardInviteesItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut access_expiration: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut email: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "access_expiration" => {
                            access_expiration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
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
                let email = email.ok_or_else(|| M::Error::missing_field("email"))?;

                let content = SharedDashboardInviteesItems {
                    access_expiration,
                    created_at,
                    email,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SharedDashboardInviteesItemsVisitor)
    }
}
