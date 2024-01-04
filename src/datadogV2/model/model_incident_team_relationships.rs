// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The incident team's relationships.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentTeamRelationships {
    /// Relationship to user.
    #[serde(rename = "created_by")]
    pub created_by: Option<Box<crate::datadogV2::model::RelationshipToUser>>,
    /// Relationship to user.
    #[serde(rename = "last_modified_by")]
    pub last_modified_by: Option<Box<crate::datadogV2::model::RelationshipToUser>>,
}

impl IncidentTeamRelationships {
    pub fn new() -> IncidentTeamRelationships {
        IncidentTeamRelationships {
            created_by: None,
            last_modified_by: None,
        }
    }
}