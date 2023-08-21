// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamsResponse {
    /// Teams response data
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Vec<Team>,
    /// Resources related to the team
    #[serde(rename = "included", skip_serializing_if = "Option::is_none")]
    pub included: Vec<TeamIncluded>,
    /// Teams response links.
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: TeamsResponseLinks,
    /// Teams response metadata.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: TeamsResponseMeta,
}

