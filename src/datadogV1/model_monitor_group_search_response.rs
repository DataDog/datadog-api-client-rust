// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorGroupSearchResponse {
    /// The counts of monitor groups per different criteria.
    #[serde(rename = "counts", skip_serializing_if = "Option::is_none")]
    pub counts: MonitorGroupSearchResponseCounts,
    /// The list of found monitor groups.
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Vec<MonitorGroupSearchResult>,
    /// Metadata about the response.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: MonitorSearchResponseMetadata,
}

