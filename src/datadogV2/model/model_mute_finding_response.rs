// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MuteFindingResponse {
    /// Data object containing the updated finding.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::MuteFindingResponseData>,
}

impl MuteFindingResponse {
    /// The expected response schema.
    pub fn new(data: crate::datadogV2::model::MuteFindingResponseData) -> MuteFindingResponse {
        MuteFindingResponse { data: Box::new(data) }
    }
}
