// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A monitor configuration policy data.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorConfigPolicyCreateData {
    /// Policy and policy type for a monitor configuration policy.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::MonitorConfigPolicyAttributeCreateRequest>,
    /// Monitor configuration policy resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::MonitorConfigPolicyResourceType,
}

impl MonitorConfigPolicyCreateData {
    pub fn new(
        attributes: Box<crate::datadogV2::model::MonitorConfigPolicyAttributeCreateRequest>,
        type_: crate::datadogV2::model::MonitorConfigPolicyResourceType,
    ) -> MonitorConfigPolicyCreateData {
        MonitorConfigPolicyCreateData { attributes, type_ }
    }
}
