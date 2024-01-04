// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Resources related to the API key.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct APIKeyRelationships {
    /// Relationship to user.
    #[serde(rename = "created_by")]
    pub created_by: Option<Box<crate::datadogV2::model::RelationshipToUser>>,
    /// Relationship to user.
    #[serde(rename = "modified_by")]
    pub modified_by: Option<Box<crate::datadogV2::model::RelationshipToUser>>,
}

impl APIKeyRelationships {
    pub fn new() -> APIKeyRelationships {
        APIKeyRelationships {
            created_by: None,
            modified_by: None,
        }
    }
}