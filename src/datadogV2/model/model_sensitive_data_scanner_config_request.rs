// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Group reorder request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerConfigRequest {
    /// Data related to the reordering of scanning groups.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::SensitiveDataScannerReorderConfig>,
    /// Meta payload containing information about the API.
    #[serde(rename = "meta")]
    pub meta: Box<crate::datadogV2::model::SensitiveDataScannerMetaVersionOnly>,
}

impl SensitiveDataScannerConfigRequest {
    pub fn new(
        data: Box<crate::datadogV2::model::SensitiveDataScannerReorderConfig>,
        meta: Box<crate::datadogV2::model::SensitiveDataScannerMetaVersionOnly>,
    ) -> SensitiveDataScannerConfigRequest {
        SensitiveDataScannerConfigRequest { data, meta }
    }
}