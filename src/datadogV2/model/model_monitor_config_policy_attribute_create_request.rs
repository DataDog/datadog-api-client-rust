// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Policy and policy type for a monitor configuration policy.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorConfigPolicyAttributeCreateRequest {
    /// Configuration for the policy.
    #[serde(rename = "policy")]
    pub policy: Box<crate::datadogV2::model::MonitorConfigPolicyPolicyCreateRequest>,
    /// The monitor configuration policy type.
    #[serde(rename = "policy_type")]
    pub policy_type: crate::datadogV2::model::MonitorConfigPolicyType,
}

impl MonitorConfigPolicyAttributeCreateRequest {
    pub fn new(
        policy: Box<crate::datadogV2::model::MonitorConfigPolicyPolicyCreateRequest>,
        policy_type: crate::datadogV2::model::MonitorConfigPolicyType,
    ) -> MonitorConfigPolicyAttributeCreateRequest {
        MonitorConfigPolicyAttributeCreateRequest {
            policy,
            policy_type,
        }
    }
}