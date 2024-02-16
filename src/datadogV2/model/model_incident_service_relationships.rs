// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The incident service's relationships.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentServiceRelationships {
    /// Relationship to user.
    #[serde(rename = "created_by")]
    pub created_by: Option<crate::datadogV2::model::RelationshipToUser>,
    /// Relationship to user.
    #[serde(rename = "last_modified_by")]
    pub last_modified_by: Option<crate::datadogV2::model::RelationshipToUser>,
}

impl IncidentServiceRelationships {
    pub fn new() -> IncidentServiceRelationships {
        IncidentServiceRelationships {
            created_by: None,
            last_modified_by: None,
        }
    }

    pub fn created_by(&mut self, value: crate::datadogV2::model::RelationshipToUser) -> &mut Self {
        self.created_by = Some(value);
        self
    }

    pub fn last_modified_by(
        &mut self,
        value: crate::datadogV2::model::RelationshipToUser,
    ) -> &mut Self {
        self.last_modified_by = Some(value);
        self
    }
}

impl Default for IncidentServiceRelationships {
    fn default() -> Self {
        Self::new()
    }
}
