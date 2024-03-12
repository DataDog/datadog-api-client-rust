// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Create group request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerGroupCreateRequest {
    /// Data related to the creation of a group.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::SensitiveDataScannerGroupCreate>,
    /// Meta payload containing information about the API.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::SensitiveDataScannerMetaVersionOnly>,
}

impl SensitiveDataScannerGroupCreateRequest {
    pub fn new() -> SensitiveDataScannerGroupCreateRequest {
        SensitiveDataScannerGroupCreateRequest {
            data: None,
            meta: None,
        }
    }

    pub fn data(mut self, value: crate::datadogV2::model::SensitiveDataScannerGroupCreate) -> Self {
        self.data = Some(value);
        self
    }

    pub fn meta(
        mut self,
        value: crate::datadogV2::model::SensitiveDataScannerMetaVersionOnly,
    ) -> Self {
        self.meta = Some(value);
        self
    }
}

impl Default for SensitiveDataScannerGroupCreateRequest {
    fn default() -> Self {
        Self::new()
    }
}
