// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Resources related to a team
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamRelationships {
    /// Relationship between a team and a team link
    #[serde(rename = "team_links")]
    pub team_links: Option<Box<crate::datadogV2::model::RelationshipToTeamLinks>>,
    /// Relationship between a user team permission and a team
    #[serde(rename = "user_team_permissions")]
    pub user_team_permissions:
        Option<Box<crate::datadogV2::model::RelationshipToUserTeamPermission>>,
}

impl TeamRelationships {
    pub fn new() -> TeamRelationships {
        TeamRelationships {
            team_links: None,
            user_team_permissions: None,
        }
    }
}