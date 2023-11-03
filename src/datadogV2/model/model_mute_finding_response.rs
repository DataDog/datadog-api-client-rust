// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The expected response schema.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MuteFindingResponse {
    /// Data object containing the updated finding.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::MuteFindingResponseData>,
}

impl MuteFindingResponse {
    pub fn new(data: crate::datadogV2::model::MuteFindingResponseData) -> MuteFindingResponse {
        MuteFindingResponse {
            data: Box::new(data),
        }
    }
}
