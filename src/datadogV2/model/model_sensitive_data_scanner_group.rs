// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A scanning group.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerGroup {
    /// ID of the group.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Sensitive Data Scanner group type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SensitiveDataScannerGroupType>,
}

impl SensitiveDataScannerGroup {
    pub fn new() -> SensitiveDataScannerGroup {
        SensitiveDataScannerGroup {
            id: None,
            type_: None,
        }
    }
}
