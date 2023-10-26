// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IPAllowlistUpdateRequest {
    /// IP allowlist data.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::IPAllowlistData>,
}

impl IPAllowlistUpdateRequest {
    /// Update the IP allowlist.
    pub fn new(data: crate::datadogV2::model::IPAllowlistData) -> IPAllowlistUpdateRequest {
        IPAllowlistUpdateRequest { data: Box::new(data) }
    }
}
