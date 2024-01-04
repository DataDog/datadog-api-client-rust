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
}
