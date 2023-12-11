// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CloudWorkloadSecurityAgentRuleType {
    #[serde(rename = "agent_rule")]
    AGENT_RULE,
}

impl ToString for CloudWorkloadSecurityAgentRuleType {
    fn to_string(&self) -> String {
        match self {
            Self::AGENT_RULE => String::from("agent_rule"),
        }
    }
}

impl Default for CloudWorkloadSecurityAgentRuleType {
    fn default() -> CloudWorkloadSecurityAgentRuleType {
        Self::AGENT_RULE
    }
}
