// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Update group request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerGroupUpdateRequest {
    /// Data related to the update of a group.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::SensitiveDataScannerGroupUpdate,
    /// Meta payload containing information about the API.
    #[serde(rename = "meta")]
    pub meta: crate::datadogV2::model::SensitiveDataScannerMetaVersionOnly,
}

impl SensitiveDataScannerGroupUpdateRequest {
    pub fn new(
        data: crate::datadogV2::model::SensitiveDataScannerGroupUpdate,
        meta: crate::datadogV2::model::SensitiveDataScannerMetaVersionOnly,
    ) -> SensitiveDataScannerGroupUpdateRequest {
        SensitiveDataScannerGroupUpdateRequest { data, meta }
    }
}
