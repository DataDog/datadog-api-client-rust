// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedDashboardInvitesDataObjectAttributes {
    /// When the invitation was sent.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: String,
    /// An email address that an invitation has been (or if used in invitation request, will be) sent to.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: String,
    /// Indicates whether an active session exists for the invitation (produced when a user clicks the link in the email).
    #[serde(rename = "has_session", skip_serializing_if = "Option::is_none")]
    pub has_session: bool,
    /// When the invitation expires.
    #[serde(rename = "invitation_expiry", skip_serializing_if = "Option::is_none")]
    pub invitation_expiry: String,
    /// When the invited user's session expires. null if the invitation has no associated session.
    #[serde(rename = "session_expiry", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub session_expiry: Option<Time>,
    /// The unique token of the shared dashboard that was (or is to be) shared.
    #[serde(rename = "share_token", skip_serializing_if = "Option::is_none")]
    pub share_token: String,
}

