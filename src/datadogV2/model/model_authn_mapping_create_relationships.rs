// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationship of AuthN Mapping create object to Role.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthNMappingCreateRelationships {
    /// Relationship to role.
    #[serde(rename = "role")]
    pub role: Option<crate::datadogV2::model::RelationshipToRole>,
}

impl AuthNMappingCreateRelationships {
    pub fn new() -> AuthNMappingCreateRelationships {
        AuthNMappingCreateRelationships { role: None }
    }

    pub fn role(&mut self, value: crate::datadogV2::model::RelationshipToRole) -> &mut Self {
        self.role = Some(value);
        self
    }
}

impl Default for AuthNMappingCreateRelationships {
    fn default() -> Self {
        Self::new()
    }
}