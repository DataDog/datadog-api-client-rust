// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The incident team's attributes from a response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentTeamResponseAttributes {
    /// Timestamp of when the incident team was created.
    #[serde(rename = "created")]
    pub created: Option<String>,
    /// Timestamp of when the incident team was modified.
    #[serde(rename = "modified")]
    pub modified: Option<String>,
    /// Name of the incident team.
    #[serde(rename = "name")]
    pub name: Option<String>,
}

impl IncidentTeamResponseAttributes {
    pub fn new() -> IncidentTeamResponseAttributes {
        IncidentTeamResponseAttributes {
            created: None,
            modified: None,
            name: None,
        }
    }

    pub fn created(&mut self, value: String) -> &mut Self {
        self.created = Some(value);
        self
    }

    pub fn modified(&mut self, value: String) -> &mut Self {
        self.modified = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }
}

impl Default for IncidentTeamResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}
