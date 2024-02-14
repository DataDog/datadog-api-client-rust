// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Team update relationships
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamUpdateRelationships {
    /// Relationship between a team and a team link
    #[serde(rename = "team_links")]
    pub team_links: Option<crate::datadogV2::model::RelationshipToTeamLinks>,
}

impl TeamUpdateRelationships {
    pub fn new() -> TeamUpdateRelationships {
        TeamUpdateRelationships { team_links: None }
    }

    pub fn team_links(
        &mut self,
        value: crate::datadogV2::model::RelationshipToTeamLinks,
    ) -> &mut Self {
        self.team_links = Some(value);
        self
    }
}

impl Default for TeamUpdateRelationships {
    fn default() -> Self {
        Self::new()
    }
}
