// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalIncidentsUpdateData {
    /// Attributes describing the new list of related signals for a security signal.
    #[serde(rename = "attributes")]
    pub attributes: SecurityMonitoringSignalIncidentsUpdateAttributes,
}

