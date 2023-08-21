// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorConfigPolicyEditData {
    /// Policy and policy type for a monitor configuration policy.
    #[serde(rename = "attributes")]
    pub attributes: MonitorConfigPolicyAttributeEditRequest,
    /// ID of this monitor configuration policy.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Monitor configuration policy resource type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: MonitorConfigPolicyResourceType,
}

