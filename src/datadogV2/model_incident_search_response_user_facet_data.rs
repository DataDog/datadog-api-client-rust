// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentSearchResponseUserFacetData {
    /// Count of the facet value appearing in search results.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: i32,
    /// Email of the user.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: String,
    /// Handle of the user.
    #[serde(rename = "handle", skip_serializing_if = "Option::is_none")]
    pub handle: String,
    /// Name of the user.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// ID of the user.
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: String,
}

