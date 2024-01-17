// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of the shared dashboard invitation
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
        }
    }
}
impl Default for SharedDashboardInvitesDataObjectAttributes {
    fn default() -> Self {
        Self::new()
    }
}
