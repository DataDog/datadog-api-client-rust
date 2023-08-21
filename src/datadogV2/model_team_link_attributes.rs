// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamLinkAttributes {
    /// The link's label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: String,
    /// The link's position, used to sort links for the team
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: i32,
    /// ID of the team the link is associated with
    #[serde(rename = "team_id", skip_serializing_if = "Option::is_none")]
    pub team_id: String,
    /// The URL for the link
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: String,
}

