// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The incident service's attributes from a response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentServiceResponseAttributes {
    /// Timestamp of when the incident service was created.
    #[serde(rename = "created")]
    pub created: Option<String>,
    /// Timestamp of when the incident service was modified.
    #[serde(rename = "modified")]
    pub modified: Option<String>,
    /// Name of the incident service.
    #[serde(rename = "name")]
    pub name: Option<String>,
}

impl IncidentServiceResponseAttributes {
    pub fn new() -> IncidentServiceResponseAttributes {
        IncidentServiceResponseAttributes {
            created: None,
            modified: None,
            name: None,
        }
    }

    pub fn with_created(&mut self, value: String) -> &mut Self {
        self.created = Some(value);
        self
    }

    pub fn with_modified(&mut self, value: String) -> &mut Self {
        self.modified = Some(value);
        self
    }

    pub fn with_name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }
}
impl Default for IncidentServiceResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}
