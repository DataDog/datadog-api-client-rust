// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object to create a user invitation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInvitationData {
    /// Relationships data for user invitation.
    #[serde(rename = "relationships")]
    pub relationships: crate::datadogV2::model::UserInvitationRelationships,
    /// User invitations type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::UserInvitationsType,
}

impl UserInvitationData {
    pub fn new(
        relationships: crate::datadogV2::model::UserInvitationRelationships,
        type_: crate::datadogV2::model::UserInvitationsType,
    ) -> UserInvitationData {
        UserInvitationData {
            relationships,
            type_,
        }
    }
}
