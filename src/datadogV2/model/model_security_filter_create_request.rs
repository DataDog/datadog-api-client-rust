// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Request object that includes the security filter that you would like to create.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityFilterCreateRequest {
    /// Object for a single security filter.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::SecurityFilterCreateData,
}

impl SecurityFilterCreateRequest {
    pub fn new(
        data: crate::datadogV2::model::SecurityFilterCreateData,
    ) -> SecurityFilterCreateRequest {
        SecurityFilterCreateRequest { data }
    }
}
