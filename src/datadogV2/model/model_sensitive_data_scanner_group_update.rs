// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data related to the update of a group.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerGroupUpdate {
    /// Attributes of the Sensitive Data Scanner group.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::SensitiveDataScannerGroupAttributes>>,
    /// ID of the group.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Relationships of the group.
    #[serde(rename = "relationships")]
    pub relationships: Option<Box<crate::datadogV2::model::SensitiveDataScannerGroupRelationships>>,
    /// Sensitive Data Scanner group type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SensitiveDataScannerGroupType>,
}

impl SensitiveDataScannerGroupUpdate {
    pub fn new() -> SensitiveDataScannerGroupUpdate {
        SensitiveDataScannerGroupUpdate {
            attributes: None,
            id: None,
            relationships: None,
            type_: None,
        }
    }
}
impl Default for SensitiveDataScannerGroupUpdate {
    fn default() -> Self {
        Self::new()
    }
}
