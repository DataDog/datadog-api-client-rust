// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A list of  SLO correction objects.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOCorrectionListResponse {
    /// The list of of SLO corrections objects.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV1::model::SLOCorrection>>,
    /// Object describing meta attributes of response.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV1::model::ResponseMetaAttributes>,
}

impl SLOCorrectionListResponse {
    pub fn new() -> SLOCorrectionListResponse {
        SLOCorrectionListResponse {
            data: None,
            meta: None,
        }
    }

    pub fn data(&mut self, value: Vec<crate::datadogV1::model::SLOCorrection>) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn meta(&mut self, value: crate::datadogV1::model::ResponseMetaAttributes) -> &mut Self {
        self.meta = Some(value);
        self
    }
}

impl Default for SLOCorrectionListResponse {
    fn default() -> Self {
        Self::new()
    }
}
