// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Request object that includes the Agent rule to create.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CloudWorkloadSecurityAgentRuleCreateRequest {
    /// Object for a single Agent rule.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleCreateData>,
}

impl CloudWorkloadSecurityAgentRuleCreateRequest {
    pub fn new(
        data: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleCreateData,
    ) -> CloudWorkloadSecurityAgentRuleCreateRequest {
        CloudWorkloadSecurityAgentRuleCreateRequest {
            data: Box::new(data),
        }
    }
}
