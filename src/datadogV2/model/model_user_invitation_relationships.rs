// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationships data for user invitation.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInvitationRelationships {
    /// Relationship to user.
    #[serde(rename = "user")]
    pub user: Box<crate::datadogV2::model::RelationshipToUser>,
}

impl UserInvitationRelationships {
    pub fn new(
        user: Box<crate::datadogV2::model::RelationshipToUser>,
    ) -> UserInvitationRelationships {
        UserInvitationRelationships { user }
    }
}
