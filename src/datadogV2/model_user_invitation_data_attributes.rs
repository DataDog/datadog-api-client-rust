// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInvitationDataAttributes {
    /// Creation time of the user invitation.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: String,
    /// Time of invitation expiration.
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: String,
    /// Type of invitation.
    #[serde(rename = "invite_type", skip_serializing_if = "Option::is_none")]
    pub invite_type: String,
    /// UUID of the user invitation.
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: String,
}

