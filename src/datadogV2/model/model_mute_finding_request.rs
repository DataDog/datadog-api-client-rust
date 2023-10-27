// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MuteFindingRequest {
    /// Data object containing the new mute properties of the finding.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::MuteFindingRequestData>,
}

impl MuteFindingRequest {
    /// The new mute finding request.
    pub fn new(data: crate::datadogV2::model::MuteFindingRequestData) -> MuteFindingRequest {
        MuteFindingRequest { data: Box::new(data) }
    }
}
