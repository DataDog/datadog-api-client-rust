// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The incident's integration relationships from a response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentIntegrationRelationships {
    /// Relationship to user.
    #[serde(rename = "created_by_user")]
    pub created_by_user: Option<crate::datadogV2::model::RelationshipToUser>,
    /// Relationship to user.
    #[serde(rename = "last_modified_by_user")]
    pub last_modified_by_user: Option<crate::datadogV2::model::RelationshipToUser>,
}

impl IncidentIntegrationRelationships {
    pub fn new() -> IncidentIntegrationRelationships {
        IncidentIntegrationRelationships {
            created_by_user: None,
            last_modified_by_user: None,
        }
    }

    pub fn created_by_user(
        &mut self,
        value: crate::datadogV2::model::RelationshipToUser,
    ) -> &mut Self {
        self.created_by_user = Some(value);
        self
    }

    pub fn last_modified_by_user(
        &mut self,
        value: crate::datadogV2::model::RelationshipToUser,
    ) -> &mut Self {
        self.last_modified_by_user = Some(value);
        self
    }
}

impl Default for IncidentIntegrationRelationships {
    fn default() -> Self {
        Self::new()
    }
}
