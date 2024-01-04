// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Rule details.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudConfigurationRegoRule {
    /// The policy written in `rego`, see: https://www.openpolicyagent.org/docs/latest/policy-language/
    #[serde(rename = "policy")]
    pub policy: String,
    /// List of resource types that will be evaluated upon. Must have at least one element.
    #[serde(rename = "resourceTypes")]
    pub resource_types: Vec<String>,
}

impl CloudConfigurationRegoRule {
    pub fn new(policy: String, resource_types: Vec<String>) -> CloudConfigurationRegoRule {
        CloudConfigurationRegoRule {
            policy,
            resource_types,
        }
    }
}
