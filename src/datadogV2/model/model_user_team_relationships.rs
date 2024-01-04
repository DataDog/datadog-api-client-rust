// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationship between membership and a user
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserTeamRelationships {
    /// Relationship between team membership and team
    #[serde(rename = "team")]
    pub team: Option<Box<crate::datadogV2::model::RelationshipToUserTeamTeam>>,
    /// Relationship between team membership and user
    #[serde(rename = "user")]
    pub user: Option<Box<crate::datadogV2::model::RelationshipToUserTeamUser>>,
}

impl UserTeamRelationships {
    pub fn new() -> UserTeamRelationships {
        UserTeamRelationships {
            team: None,
            user: None,
        }
    }
}