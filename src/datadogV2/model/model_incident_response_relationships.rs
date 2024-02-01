// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The incident's relationships from a response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentResponseRelationships {
    /// A relationship reference for attachments.
    #[serde(rename = "attachments")]
    pub attachments: Option<crate::datadogV2::model::RelationshipToIncidentAttachment>,
    /// Relationship to user.
    #[serde(
        rename = "commander_user",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub commander_user: Option<Option<crate::datadogV2::model::NullableRelationshipToUser>>,
    /// Relationship to user.
    #[serde(rename = "created_by_user")]
    pub created_by_user: Option<crate::datadogV2::model::RelationshipToUser>,
    /// Relationship to impacts.
    #[serde(rename = "impacts")]
    pub impacts: Option<crate::datadogV2::model::RelationshipToIncidentImpacts>,
    /// A relationship reference for multiple integration metadata objects.
    #[serde(rename = "integrations")]
    pub integrations: Option<crate::datadogV2::model::RelationshipToIncidentIntegrationMetadatas>,
    /// Relationship to user.
    #[serde(rename = "last_modified_by_user")]
    pub last_modified_by_user: Option<crate::datadogV2::model::RelationshipToUser>,
    /// Relationship to incident responders.
    #[serde(rename = "responders")]
    pub responders: Option<crate::datadogV2::model::RelationshipToIncidentResponders>,
    /// Relationship to incident user defined fields.
    #[serde(rename = "user_defined_fields")]
    pub user_defined_fields:
        Option<crate::datadogV2::model::RelationshipToIncidentUserDefinedFields>,
}

impl IncidentResponseRelationships {
    pub fn new() -> IncidentResponseRelationships {
        IncidentResponseRelationships {
            attachments: None,
            commander_user: None,
            created_by_user: None,
            impacts: None,
            integrations: None,
            last_modified_by_user: None,
            responders: None,
            user_defined_fields: None,
        }
    }

    pub fn attachments(
        &mut self,
        value: crate::datadogV2::model::RelationshipToIncidentAttachment,
    ) -> &mut Self {
        self.attachments = Some(value);
        self
    }

    pub fn commander_user(
        &mut self,
        value: Option<crate::datadogV2::model::NullableRelationshipToUser>,
    ) -> &mut Self {
        self.commander_user = Some(value);
        self
    }

    pub fn created_by_user(
        &mut self,
        value: crate::datadogV2::model::RelationshipToUser,
    ) -> &mut Self {
        self.created_by_user = Some(value);
        self
    }

    pub fn impacts(
        &mut self,
        value: crate::datadogV2::model::RelationshipToIncidentImpacts,
    ) -> &mut Self {
        self.impacts = Some(value);
        self
    }

    pub fn integrations(
        &mut self,
        value: crate::datadogV2::model::RelationshipToIncidentIntegrationMetadatas,
    ) -> &mut Self {
        self.integrations = Some(value);
        self
    }

    pub fn last_modified_by_user(
        &mut self,
        value: crate::datadogV2::model::RelationshipToUser,
    ) -> &mut Self {
        self.last_modified_by_user = Some(value);
        self
    }

    pub fn responders(
        &mut self,
        value: crate::datadogV2::model::RelationshipToIncidentResponders,
    ) -> &mut Self {
        self.responders = Some(value);
        self
    }

    pub fn user_defined_fields(
        &mut self,
        value: crate::datadogV2::model::RelationshipToIncidentUserDefinedFields,
    ) -> &mut Self {
        self.user_defined_fields = Some(value);
        self
    }
}

impl Default for IncidentResponseRelationships {
    fn default() -> Self {
        Self::new()
    }
}
