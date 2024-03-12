// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A Scanning Group included item.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerGroupIncludedItem {
    /// Attributes of the Sensitive Data Scanner group.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::SensitiveDataScannerGroupAttributes>,
    /// ID of the group.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Relationships of the group.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::SensitiveDataScannerGroupRelationships>,
    /// Sensitive Data Scanner group type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SensitiveDataScannerGroupType>,
}

impl SensitiveDataScannerGroupIncludedItem {
    pub fn new() -> SensitiveDataScannerGroupIncludedItem {
        SensitiveDataScannerGroupIncludedItem {
            attributes: None,
            id: None,
            relationships: None,
            type_: None,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV2::model::SensitiveDataScannerGroupAttributes,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn relationships(
        mut self,
        value: crate::datadogV2::model::SensitiveDataScannerGroupRelationships,
    ) -> Self {
        self.relationships = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::SensitiveDataScannerGroupType) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for SensitiveDataScannerGroupIncludedItem {
    fn default() -> Self {
        Self::new()
    }
}
