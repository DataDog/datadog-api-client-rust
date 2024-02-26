// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object for a single Agent rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudWorkloadSecurityAgentRuleUpdateData {
    /// Update an existing Cloud Workload Security Agent rule.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleUpdateAttributes,
    /// The type of the resource. The value should always be `agent_rule`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleType,
}

impl CloudWorkloadSecurityAgentRuleUpdateData {
    pub fn new(
        attributes: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleUpdateAttributes,
        type_: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleType,
    ) -> CloudWorkloadSecurityAgentRuleUpdateData {
        CloudWorkloadSecurityAgentRuleUpdateData { attributes, type_ }
    }
}
