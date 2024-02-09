// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationship to user.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToUser {
    /// Relationship to user object.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::RelationshipToUserData,
}

impl RelationshipToUser {
    pub fn new(data: crate::datadogV2::model::RelationshipToUserData) -> RelationshipToUser {
        RelationshipToUser { data }
    }
}
