// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Facet data for user attributes of an incident.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentSearchResponseUserFacetData {
    /// Count of the facet value appearing in search results.
    #[serde(rename = "count")]
    pub count: Option<i32>,
    /// Email of the user.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// Handle of the user.
    #[serde(rename = "handle")]
    pub handle: Option<String>,
    /// Name of the user.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// ID of the user.
    #[serde(rename = "uuid")]
    pub uuid: Option<String>,
}

impl IncidentSearchResponseUserFacetData {
    pub fn new() -> IncidentSearchResponseUserFacetData {
        IncidentSearchResponseUserFacetData {
            count: None,
            email: None,
            handle: None,
            name: None,
            uuid: None,
        }
    }

    pub fn with_count(&mut self, value: i32) -> &mut Self {
        self.count = Some(value);
        self
    }

    pub fn with_email(&mut self, value: String) -> &mut Self {
        self.email = Some(value);
        self
    }

    pub fn with_handle(&mut self, value: String) -> &mut Self {
        self.handle = Some(value);
        self
    }

    pub fn with_name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn with_uuid(&mut self, value: String) -> &mut Self {
        self.uuid = Some(value);
        self
    }
}
impl Default for IncidentSearchResponseUserFacetData {
    fn default() -> Self {
        Self::new()
    }
}
