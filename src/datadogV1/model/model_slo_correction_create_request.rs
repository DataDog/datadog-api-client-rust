// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An object that defines a correction to be applied to an SLO.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SLOCorrectionCreateRequest {
    /// The data object associated with the SLO correction to be created.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV1::model::SLOCorrectionCreateData>>,
}

impl SLOCorrectionCreateRequest {
    pub fn new() -> SLOCorrectionCreateRequest {
        SLOCorrectionCreateRequest { data: None }
    }
}
