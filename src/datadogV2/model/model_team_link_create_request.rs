// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Team link create request
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamLinkCreateRequest {
    /// Team link create
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::TeamLinkCreate,
}

impl TeamLinkCreateRequest {
    pub fn new(data: crate::datadogV2::model::TeamLinkCreate) -> TeamLinkCreateRequest {
        TeamLinkCreateRequest { data }
    }
}
