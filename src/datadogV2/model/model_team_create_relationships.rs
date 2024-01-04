// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationships formed with the team on creation
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamCreateRelationships {
    /// Relationship to users.
    #[serde(rename = "users")]
    pub users: Option<Box<crate::datadogV2::model::RelationshipToUsers>>,
}

impl TeamCreateRelationships {
    pub fn new() -> TeamCreateRelationships {
        TeamCreateRelationships { users: None }
    }
}