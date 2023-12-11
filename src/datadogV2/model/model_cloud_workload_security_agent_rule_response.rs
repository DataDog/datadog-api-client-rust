// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response object that includes an Agent rule.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CloudWorkloadSecurityAgentRuleResponse {
    /// Object for a single Agent rule.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleData>>,
}

impl CloudWorkloadSecurityAgentRuleResponse {
    pub fn new() -> CloudWorkloadSecurityAgentRuleResponse {
        CloudWorkloadSecurityAgentRuleResponse { data: None }
    }
}
