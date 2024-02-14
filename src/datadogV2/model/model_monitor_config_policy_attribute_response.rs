// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Policy and policy type for a monitor configuration policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorConfigPolicyAttributeResponse {
    /// Configuration for the policy.
    #[serde(rename = "policy")]
    pub policy: Option<crate::datadogV2::model::MonitorConfigPolicyPolicy>,
    /// The monitor configuration policy type.
    #[serde(rename = "policy_type")]
    pub policy_type: Option<crate::datadogV2::model::MonitorConfigPolicyType>,
}

impl MonitorConfigPolicyAttributeResponse {
    pub fn new() -> MonitorConfigPolicyAttributeResponse {
        MonitorConfigPolicyAttributeResponse {
            policy: None,
            policy_type: None,
        }
    }

    pub fn policy(
        &mut self,
        value: crate::datadogV2::model::MonitorConfigPolicyPolicy,
    ) -> &mut Self {
        self.policy = Some(value);
        self
    }

    pub fn policy_type(
        &mut self,
        value: crate::datadogV2::model::MonitorConfigPolicyType,
    ) -> &mut Self {
        self.policy_type = Some(value);
        self
    }
}

impl Default for MonitorConfigPolicyAttributeResponse {
    fn default() -> Self {
        Self::new()
    }
}
