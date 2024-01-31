// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A user's relationship with a team
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserTeam {
    /// Team membership attributes
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::UserTeamAttributes>,
    /// The ID of a user's relationship with a team
    #[serde(rename = "id")]
    pub id: String,
    /// Relationship between membership and a user
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::UserTeamRelationships>,
    /// Team membership type
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::UserTeamType,
}

impl UserTeam {
    pub fn new(id: String, type_: crate::datadogV2::model::UserTeamType) -> UserTeam {
        UserTeam {
            attributes: None,
            id,
            relationships: None,
            type_,
        }
    }

    pub fn with_attributes(
        &mut self,
        value: crate::datadogV2::model::UserTeamAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn with_relationships(
        &mut self,
        value: crate::datadogV2::model::UserTeamRelationships,
    ) -> &mut Self {
        self.relationships = Some(value);
        self
    }
}
