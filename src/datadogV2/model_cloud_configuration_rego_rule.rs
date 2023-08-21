// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudConfigurationRegoRule {
    /// The policy written in `rego`, see: https://www.openpolicyagent.org/docs/latest/policy-language/
    #[serde(rename = "policy", skip_serializing_if = "Option::is_none")]
    pub policy: String,
    /// List of resource types that will be evaluated upon. Must have at least one element.
    #[serde(rename = "resourceTypes", skip_serializing_if = "Option::is_none")]
    pub resource_types: Vec<String>,
}

