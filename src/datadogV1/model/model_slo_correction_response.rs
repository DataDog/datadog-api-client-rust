// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The response object of an SLO correction.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SLOCorrectionResponse {
    /// The response object of a list of SLO corrections.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV1::model::SLOCorrection>>,
}

impl SLOCorrectionResponse {
    pub fn new() -> SLOCorrectionResponse {
        SLOCorrectionResponse { data: None }
    }
}
