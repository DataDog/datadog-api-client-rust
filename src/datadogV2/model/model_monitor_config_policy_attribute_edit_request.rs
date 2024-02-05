// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Policy and policy type for a monitor configuration policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorConfigPolicyAttributeEditRequest {
    /// Configuration for the policy.
    #[serde(rename = "policy")]
    pub policy: crate::datadogV2::model::MonitorConfigPolicyPolicy,
    /// The monitor configuration policy type.
    #[serde(rename = "policy_type")]
    pub policy_type: crate::datadogV2::model::MonitorConfigPolicyType,
}

impl MonitorConfigPolicyAttributeEditRequest {
    pub fn new(
        policy: crate::datadogV2::model::MonitorConfigPolicyPolicy,
        policy_type: crate::datadogV2::model::MonitorConfigPolicyType,
    ) -> MonitorConfigPolicyAttributeEditRequest {
        MonitorConfigPolicyAttributeEditRequest {
            policy,
            policy_type,
        }
    }
}
