// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Delete group response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerGroupDeleteResponse {
    /// Meta payload containing information about the API.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::SensitiveDataScannerMetaVersionOnly>,
}

impl SensitiveDataScannerGroupDeleteResponse {
    pub fn new() -> SensitiveDataScannerGroupDeleteResponse {
        SensitiveDataScannerGroupDeleteResponse { meta: None }
    }

    pub fn meta(
        &mut self,
        value: crate::datadogV2::model::SensitiveDataScannerMetaVersionOnly,
    ) -> &mut Self {
        self.meta = Some(value);
        self
    }
}

impl Default for SensitiveDataScannerGroupDeleteResponse {
    fn default() -> Self {
        Self::new()
    }
}
