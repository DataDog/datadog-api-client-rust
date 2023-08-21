// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorSearchResponse {
    /// The counts of monitors per different criteria.
    #[serde(rename = "counts", skip_serializing_if = "Option::is_none")]
    pub counts: MonitorSearchResponseCounts,
    /// Metadata about the response.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: MonitorSearchResponseMetadata,
    /// The list of found monitors.
    #[serde(rename = "monitors", skip_serializing_if = "Option::is_none")]
    pub monitors: Vec<MonitorSearchResult>,
}

