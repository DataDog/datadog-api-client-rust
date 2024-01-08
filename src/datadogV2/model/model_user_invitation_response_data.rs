// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object of a user invitation returned by the API.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInvitationResponseData {
    /// Attributes of a user invitation.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::UserInvitationDataAttributes>>,
    /// ID of the user invitation.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Relationships data for user invitation.
    #[serde(rename = "relationships")]
    pub relationships: Option<Box<crate::datadogV2::model::UserInvitationRelationships>>,
    /// User invitations type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::UserInvitationsType>,
}

impl UserInvitationResponseData {
    pub fn new() -> UserInvitationResponseData {
        UserInvitationResponseData {
            attributes: None,
            id: None,
            relationships: None,
            type_: None,
        }
    }
}
impl Default for UserInvitationResponseData {
    fn default() -> Self {
        Self::new()
    }
}
