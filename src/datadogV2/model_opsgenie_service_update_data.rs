// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpsgenieServiceUpdateData {
    /// The Opsgenie service attributes for an update request.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: OpsgenieServiceUpdateAttributes,
    /// The ID of the Opsgenie service.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Opsgenie service resource type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: OpsgenieServiceType,
}

