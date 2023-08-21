// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamAttributes {
    /// Creation date of the team
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: String,
    /// Free-form markdown description/content for the team's homepage
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    /// The team's identifier
    #[serde(rename = "handle", skip_serializing_if = "Option::is_none")]
    pub handle: String,
    /// The number of links belonging to the team
    #[serde(rename = "link_count", skip_serializing_if = "Option::is_none")]
    pub link_count: i32,
    /// Modification date of the team
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: String,
    /// The name of the team
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// A brief summary of the team, derived from the `description`
    #[serde(rename = "summary", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub summary: Option<String>,
    /// The number of users belonging to the team
    #[serde(rename = "user_count", skip_serializing_if = "Option::is_none")]
    pub user_count: i32,
}

