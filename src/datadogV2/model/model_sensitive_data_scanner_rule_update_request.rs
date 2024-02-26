// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Update rule request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerRuleUpdateRequest {
    /// Data related to the update of a rule.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::SensitiveDataScannerRuleUpdate,
    /// Meta payload containing information about the API.
    #[serde(rename = "meta")]
    pub meta: crate::datadogV2::model::SensitiveDataScannerMetaVersionOnly,
}

impl SensitiveDataScannerRuleUpdateRequest {
    pub fn new(
        data: crate::datadogV2::model::SensitiveDataScannerRuleUpdate,
        meta: crate::datadogV2::model::SensitiveDataScannerMetaVersionOnly,
    ) -> SensitiveDataScannerRuleUpdateRequest {
        SensitiveDataScannerRuleUpdateRequest { data, meta }
    }
}
