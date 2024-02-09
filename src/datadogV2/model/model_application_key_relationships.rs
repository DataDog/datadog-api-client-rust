// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Resources related to the application key.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationKeyRelationships {
    /// Relationship to user.
    #[serde(rename = "owned_by")]
    pub owned_by: Option<crate::datadogV2::model::RelationshipToUser>,
}

impl ApplicationKeyRelationships {
    pub fn new() -> ApplicationKeyRelationships {
        ApplicationKeyRelationships { owned_by: None }
    }

    pub fn owned_by(&mut self, value: crate::datadogV2::model::RelationshipToUser) -> &mut Self {
        self.owned_by = Some(value);
        self
    }
}

impl Default for ApplicationKeyRelationships {
    fn default() -> Self {
        Self::new()
    }
}
