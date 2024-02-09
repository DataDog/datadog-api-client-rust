// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationship to an organization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToOrganization {
    /// Relationship to organization object.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::RelationshipToOrganizationData,
}

impl RelationshipToOrganization {
    pub fn new(
        data: crate::datadogV2::model::RelationshipToOrganizationData,
    ) -> RelationshipToOrganization {
        RelationshipToOrganization { data }
    }
}
