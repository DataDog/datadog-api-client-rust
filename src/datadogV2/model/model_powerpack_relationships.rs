// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Powerpack relationship object.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerpackRelationships {
    /// Relationship to user.
    #[serde(rename = "author")]
    pub author: Option<crate::datadogV2::model::RelationshipToUser>,
}

impl PowerpackRelationships {
    pub fn new() -> PowerpackRelationships {
        PowerpackRelationships { author: None }
    }

    pub fn with_author(&mut self, value: crate::datadogV2::model::RelationshipToUser) -> &mut Self {
        self.author = Some(value);
        self
    }
}
impl Default for PowerpackRelationships {
    fn default() -> Self {
        Self::new()
    }
}
