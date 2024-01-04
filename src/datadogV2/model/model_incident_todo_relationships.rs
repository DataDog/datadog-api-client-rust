// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The incident's relationships from a response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentTodoRelationships {
    /// Relationship to user.
    #[serde(rename = "created_by_user")]
    pub created_by_user: Option<Box<crate::datadogV2::model::RelationshipToUser>>,
    /// Relationship to user.
    #[serde(rename = "last_modified_by_user")]
    pub last_modified_by_user: Option<Box<crate::datadogV2::model::RelationshipToUser>>,
}

impl IncidentTodoRelationships {
    pub fn new() -> IncidentTodoRelationships {
        IncidentTodoRelationships {
            created_by_user: None,
            last_modified_by_user: None,
        }
    }
}