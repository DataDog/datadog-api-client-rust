// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The incident's relationships for an update request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentUpdateRelationships {
    /// Relationship to user.
    #[serde(
        rename = "commander_user",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub commander_user: Option<Option<crate::datadogV2::model::NullableRelationshipToUser>>,
    /// A relationship reference for multiple integration metadata objects.
    #[serde(rename = "integrations")]
    pub integrations: Option<crate::datadogV2::model::RelationshipToIncidentIntegrationMetadatas>,
    /// A relationship reference for postmortems.
    #[serde(rename = "postmortem")]
    pub postmortem: Option<crate::datadogV2::model::RelationshipToIncidentPostmortem>,
}

impl IncidentUpdateRelationships {
    pub fn new() -> IncidentUpdateRelationships {
        IncidentUpdateRelationships {
            commander_user: None,
            integrations: None,
            postmortem: None,
        }
    }

    pub fn with_commander_user(
        &mut self,
        value: Option<crate::datadogV2::model::NullableRelationshipToUser>,
    ) -> &mut Self {
        self.commander_user = Some(value);
        self
    }

    pub fn with_integrations(
        &mut self,
        value: crate::datadogV2::model::RelationshipToIncidentIntegrationMetadatas,
    ) -> &mut Self {
        self.integrations = Some(value);
        self
    }

    pub fn with_postmortem(
        &mut self,
        value: crate::datadogV2::model::RelationshipToIncidentPostmortem,
    ) -> &mut Self {
        self.postmortem = Some(value);
        self
    }
}
impl Default for IncidentUpdateRelationships {
    fn default() -> Self {
        Self::new()
    }
}
