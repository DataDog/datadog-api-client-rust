// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RestrictionPolicyUpdateRequest {
    /// Restriction policy object.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::RestrictionPolicy>,
}

impl RestrictionPolicyUpdateRequest {
    /// Update request for a restriction policy.
    pub fn new(data: crate::datadogV2::model::RestrictionPolicy) -> RestrictionPolicyUpdateRequest {
        RestrictionPolicyUpdateRequest { data: Box::new(data) }
    }
}
