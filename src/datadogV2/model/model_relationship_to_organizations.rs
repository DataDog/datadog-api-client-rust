// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationship to organizations.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToOrganizations {
    /// Relationships to organization objects.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::RelationshipToOrganizationData>,
}

impl RelationshipToOrganizations {
    pub fn new(
        data: Vec<crate::datadogV2::model::RelationshipToOrganizationData>,
    ) -> RelationshipToOrganizations {
        RelationshipToOrganizations { data }
    }
}
