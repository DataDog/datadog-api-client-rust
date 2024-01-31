// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Create rule request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerRuleCreateRequest {
    /// Data related to the creation of a rule.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::SensitiveDataScannerRuleCreate,
    /// Meta payload containing information about the API.
    #[serde(rename = "meta")]
    pub meta: crate::datadogV2::model::SensitiveDataScannerMetaVersionOnly,
}

impl SensitiveDataScannerRuleCreateRequest {
    pub fn new(
        data: crate::datadogV2::model::SensitiveDataScannerRuleCreate,
        meta: crate::datadogV2::model::SensitiveDataScannerMetaVersionOnly,
    ) -> SensitiveDataScannerRuleCreateRequest {
        SensitiveDataScannerRuleCreateRequest { data, meta }
    }
}
