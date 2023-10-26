// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CloudWorkloadSecurityAgentRuleUpdateRequest {
    /// Object for a single Agent rule.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleUpdateData>,
}

impl CloudWorkloadSecurityAgentRuleUpdateRequest {
    /// Request object that includes the Agent rule with the attributes to update.
    pub fn new(
        data: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleUpdateData,
    ) -> CloudWorkloadSecurityAgentRuleUpdateRequest {
        CloudWorkloadSecurityAgentRuleUpdateRequest { data: Box::new(data) }
    }
}
