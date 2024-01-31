// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data related to a Sensitive Data Scanner Group.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerGroupItem {
    /// ID of the group.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Sensitive Data Scanner group type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SensitiveDataScannerGroupType>,
}

impl SensitiveDataScannerGroupItem {
    pub fn new() -> SensitiveDataScannerGroupItem {
        SensitiveDataScannerGroupItem {
            id: None,
            type_: None,
        }
    }

    pub fn with_id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn with_type_(
        &mut self,
        value: crate::datadogV2::model::SensitiveDataScannerGroupType,
    ) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}
impl Default for SensitiveDataScannerGroupItem {
    fn default() -> Self {
        Self::new()
    }
}
