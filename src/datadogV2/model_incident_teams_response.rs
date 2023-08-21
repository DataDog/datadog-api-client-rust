// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentTeamsResponse {
    /// An array of incident teams.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Vec<IncidentTeamResponseData>,
    /// Included related resources which the user requested.
    #[serde(rename = "included", skip_serializing_if = "Option::is_none")]
    pub included: Vec<IncidentTeamIncludedItems>,
    /// The metadata object containing pagination metadata.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: IncidentResponseMeta,
}

