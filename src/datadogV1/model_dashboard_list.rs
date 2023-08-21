// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardList {
    /// Object describing the creator of the shared element.
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Creator,
    /// Date of creation of the dashboard list.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: String,
    /// The number of dashboards in the list.
    #[serde(rename = "dashboard_count", skip_serializing_if = "Option::is_none")]
    pub dashboard_count: i64,
    /// The ID of the dashboard list.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: i64,
    /// Whether or not the list is in the favorites.
    #[serde(rename = "is_favorite", skip_serializing_if = "Option::is_none")]
    pub is_favorite: bool,
    /// Date of last edition of the dashboard list.
    #[serde(rename = "modified", skip_serializing_if = "Option::is_none")]
    pub modified: String,
    /// The name of the dashboard list.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// The type of dashboard list.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: String,
}

