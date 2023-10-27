// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CloudWorkloadSecurityAgentRuleCreatorAttributes {
    /// The handle of the user.
    #[serde(rename = "handle")]
    pub handle: Option<String>,
    /// The name of the user.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option")]
    pub name: Option<Option<String>>,
}

impl CloudWorkloadSecurityAgentRuleCreatorAttributes {
    /// The attributes of the user who created the Agent rule.
    pub fn new() -> CloudWorkloadSecurityAgentRuleCreatorAttributes {
        CloudWorkloadSecurityAgentRuleCreatorAttributes {
            handle: None,
            name: None,
        }
    }
}
