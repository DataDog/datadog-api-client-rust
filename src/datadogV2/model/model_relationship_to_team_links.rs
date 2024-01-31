// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationship between a team and a team link
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToTeamLinks {
    /// Related team links
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::RelationshipToTeamLinkData>>,
    /// Links attributes.
    #[serde(rename = "links")]
    pub links: Option<crate::datadogV2::model::TeamRelationshipsLinks>,
}

impl RelationshipToTeamLinks {
    pub fn new() -> RelationshipToTeamLinks {
        RelationshipToTeamLinks {
            data: None,
            links: None,
        }
    }

    pub fn with_data(
        &mut self,
        value: Vec<crate::datadogV2::model::RelationshipToTeamLinkData>,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn with_links(
        &mut self,
        value: crate::datadogV2::model::TeamRelationshipsLinks,
    ) -> &mut Self {
        self.links = Some(value);
        self
    }
}
impl Default for RelationshipToTeamLinks {
    fn default() -> Self {
        Self::new()
    }
}
