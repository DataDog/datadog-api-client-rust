// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the shared dashboard invitation
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SharedDashboardInvitesDataObjectAttributes {
    /// When the invitation was sent.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// An email address that an invitation has been (or if used in invitation request, will be) sent to.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// Indicates whether an active session exists for the invitation (produced when a user clicks the link in the email).
    #[serde(rename = "has_session")]
    pub has_session: Option<bool>,
    /// When the invitation expires.
    #[serde(rename = "invitation_expiry")]
    pub invitation_expiry: Option<String>,
    /// When the invited user's session expires. null if the invitation has no associated session.
    #[serde(
        rename = "session_expiry",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub session_expiry: Option<Option<String>>,
    /// The unique token of the shared dashboard that was (or is to be) shared.
    #[serde(rename = "share_token")]
    pub share_token: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SharedDashboardInvitesDataObjectAttributes {
    pub fn new() -> SharedDashboardInvitesDataObjectAttributes {
        SharedDashboardInvitesDataObjectAttributes {
            created_at: None,
            email: None,
            has_session: None,
            invitation_expiry: None,
            session_expiry: None,
            share_token: None,
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: String) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn email(mut self, value: String) -> Self {
        self.email = Some(value);
        self
    }

    pub fn has_session(mut self, value: bool) -> Self {
        self.has_session = Some(value);
        self
    }

    pub fn invitation_expiry(mut self, value: String) -> Self {
        self.invitation_expiry = Some(value);
        self
    }

    pub fn session_expiry(mut self, value: Option<String>) -> Self {
        self.session_expiry = Some(value);
        self
    }

    pub fn share_token(mut self, value: String) -> Self {
        self.share_token = Some(value);
        self
    }
}

impl Default for SharedDashboardInvitesDataObjectAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SharedDashboardInvitesDataObjectAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SharedDashboardInvitesDataObjectAttributesVisitor;
        impl<'a> Visitor<'a> for SharedDashboardInvitesDataObjectAttributesVisitor {
            type Value = SharedDashboardInvitesDataObjectAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<String> = None;
                let mut email: Option<String> = None;
                let mut has_session: Option<bool> = None;
                let mut invitation_expiry: Option<String> = None;
                let mut session_expiry: Option<Option<String>> = None;
                let mut share_token: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "email" => {
                            if v.is_null() {
                                continue;
                            }
                            email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "has_session" => {
                            if v.is_null() {
                                continue;
                            }
                            has_session =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "invitation_expiry" => {
                            if v.is_null() {
                                continue;
                            }
                            invitation_expiry =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_expiry" => {
                            session_expiry =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "share_token" => {
                            if v.is_null() {
                                continue;
                            }
                            share_token =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SharedDashboardInvitesDataObjectAttributes {
                    created_at,
                    email,
                    has_session,
                    invitation_expiry,
                    session_expiry,
                    share_token,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SharedDashboardInvitesDataObjectAttributesVisitor)
    }
}
