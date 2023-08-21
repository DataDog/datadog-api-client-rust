// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorConfigPolicyAttributeResponse {
    /// Configuration for the policy.
    #[serde(rename = "policy", skip_serializing_if = "Option::is_none")]
    pub policy: MonitorConfigPolicyPolicy,
    /// The monitor configuration policy type.
    #[serde(rename = "policy_type", skip_serializing_if = "Option::is_none")]
    pub policy_type: MonitorConfigPolicyType,
}

