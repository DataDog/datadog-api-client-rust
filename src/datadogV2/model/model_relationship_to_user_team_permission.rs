// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationship between a user team permission and a team
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToUserTeamPermission {
    /// Related user team permission data
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::RelationshipToUserTeamPermissionData>,
    /// Links attributes.
    #[serde(rename = "links")]
    pub links: Option<crate::datadogV2::model::TeamRelationshipsLinks>,
}

impl RelationshipToUserTeamPermission {
    pub fn new() -> RelationshipToUserTeamPermission {
        RelationshipToUserTeamPermission {
            data: None,
            links: None,
        }
    }

    pub fn data(
        &mut self,
        value: crate::datadogV2::model::RelationshipToUserTeamPermissionData,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn links(&mut self, value: crate::datadogV2::model::TeamRelationshipsLinks) -> &mut Self {
        self.links = Some(value);
        self
    }
}

impl Default for RelationshipToUserTeamPermission {
    fn default() -> Self {
        Self::new()
    }
}
