// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationships of the user object.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRelationships {
    /// Relationship to roles.
    #[serde(rename = "roles")]
    pub roles: Option<Box<crate::datadogV2::model::RelationshipToRoles>>,
}

impl UserRelationships {
    pub fn new() -> UserRelationships {
        UserRelationships { roles: None }
    }
}