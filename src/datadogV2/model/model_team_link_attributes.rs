// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Team link attributes
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamLinkAttributes {
    /// The link's label
    #[serde(rename = "label")]
    pub label: String,
    /// The link's position, used to sort links for the team
    #[serde(rename = "position")]
    pub position: Option<i32>,
    /// ID of the team the link is associated with
    #[serde(rename = "team_id")]
    pub team_id: Option<String>,
    /// The URL for the link
    #[serde(rename = "url")]
    pub url: String,
}

impl TeamLinkAttributes {
    pub fn new(label: String, url: String) -> TeamLinkAttributes {
        TeamLinkAttributes {
            label,
            position: None,
            team_id: None,
            url,
        }
    }
}
