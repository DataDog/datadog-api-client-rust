// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeRelationships {
    /// The user who created the downtime.
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: DowntimeRelationshipsCreatedBy,
    /// The monitor identified by the downtime.
    #[serde(rename = "monitor", skip_serializing_if = "Option::is_none")]
    pub monitor: DowntimeRelationshipsMonitor,
}

