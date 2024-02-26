// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Request for editing a monitor configuration policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorConfigPolicyEditRequest {
    /// A monitor configuration policy data.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::MonitorConfigPolicyEditData,
}

impl MonitorConfigPolicyEditRequest {
    pub fn new(
        data: crate::datadogV2::model::MonitorConfigPolicyEditData,
    ) -> MonitorConfigPolicyEditRequest {
        MonitorConfigPolicyEditRequest { data }
    }
}
