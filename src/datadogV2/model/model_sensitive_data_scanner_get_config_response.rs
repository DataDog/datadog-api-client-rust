// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Get all groups response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerGetConfigResponse {
    /// Response data related to the scanning groups.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::SensitiveDataScannerGetConfigResponseData>,
    /// Included objects from relationships.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::SensitiveDataScannerGetConfigIncludedItem>>,
    /// Meta response containing information about the API.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::SensitiveDataScannerMeta>,
}

impl SensitiveDataScannerGetConfigResponse {
    pub fn new() -> SensitiveDataScannerGetConfigResponse {
        SensitiveDataScannerGetConfigResponse {
            data: None,
            included: None,
            meta: None,
        }
    }

    pub fn data(
        mut self,
        value: crate::datadogV2::model::SensitiveDataScannerGetConfigResponseData,
    ) -> Self {
        self.data = Some(value);
        self
    }

    pub fn included(
        mut self,
        value: Vec<crate::datadogV2::model::SensitiveDataScannerGetConfigIncludedItem>,
    ) -> Self {
        self.included = Some(value);
        self
    }

    pub fn meta(mut self, value: crate::datadogV2::model::SensitiveDataScannerMeta) -> Self {
        self.meta = Some(value);
        self
    }
}

impl Default for SensitiveDataScannerGetConfigResponse {
    fn default() -> Self {
        Self::new()
    }
}
