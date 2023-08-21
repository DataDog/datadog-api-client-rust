// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudWorkloadSecurityAgentRuleCreateData {
    /// Create a new Cloud Workload Security Agent rule.
    #[serde(rename = "attributes")]
    pub attributes: CloudWorkloadSecurityAgentRuleCreateAttributes,
    /// The type of the resource. The value should always be `agent_rule`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: CloudWorkloadSecurityAgentRuleType,
}

