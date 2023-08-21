// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalStateUpdateData {
    /// Attributes describing the change of state of a security signal.
    #[serde(rename = "attributes")]
    pub attributes: SecurityMonitoringSignalStateUpdateAttributes,
    /// The unique ID of the security signal.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: interface{},
    /// The type of event.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SecurityMonitoringSignalMetadataType,
}

