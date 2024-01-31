// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An object that defines a correction to be applied to an SLO.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOCorrectionUpdateRequest {
    /// The data object associated with the SLO correction to be updated.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV1::model::SLOCorrectionUpdateData>,
}

impl SLOCorrectionUpdateRequest {
    pub fn new() -> SLOCorrectionUpdateRequest {
        SLOCorrectionUpdateRequest { data: None }
    }

    pub fn with_data(
        &mut self,
        value: crate::datadogV1::model::SLOCorrectionUpdateData,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }
}
impl Default for SLOCorrectionUpdateRequest {
    fn default() -> Self {
        Self::new()
    }
}
