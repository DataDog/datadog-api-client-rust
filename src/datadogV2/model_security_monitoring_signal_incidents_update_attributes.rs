// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalIncidentsUpdateAttributes {
    /// Array of incidents that are associated with this signal.
    #[serde(rename = "incident_ids", skip_serializing_if = "Option::is_none")]
    pub incident_ids: Vec<i64>,
    /// Version of the updated signal. If server side version is higher, update will be rejected.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: i64,
}

