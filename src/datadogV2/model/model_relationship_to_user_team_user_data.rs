// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A user's relationship with a team
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToUserTeamUserData {
    /// The ID of the user associated with the team
    #[serde(rename = "id")]
    pub id: String,
    /// User team user type
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::UserTeamUserType,
}

impl RelationshipToUserTeamUserData {
    pub fn new(
        id: String,
        type_: crate::datadogV2::model::UserTeamUserType,
    ) -> RelationshipToUserTeamUserData {
        RelationshipToUserTeamUserData { id, type_ }
    }
}
