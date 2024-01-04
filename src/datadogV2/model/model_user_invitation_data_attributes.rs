// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of a user invitation.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInvitationDataAttributes {
    /// Creation time of the user invitation.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// Time of invitation expiration.
    #[serde(rename = "expires_at")]
    pub expires_at: Option<String>,
    /// Type of invitation.
    #[serde(rename = "invite_type")]
    pub invite_type: Option<String>,
    /// UUID of the user invitation.
    #[serde(rename = "uuid")]
    pub uuid: Option<String>,
}

impl UserInvitationDataAttributes {
    pub fn new() -> UserInvitationDataAttributes {
        UserInvitationDataAttributes {
            created_at: None,
            expires_at: None,
            invite_type: None,
            uuid: None,
        }
    }
}