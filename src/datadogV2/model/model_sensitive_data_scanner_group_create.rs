// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data related to the creation of a group.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerGroupCreate {
    /// Attributes of the Sensitive Data Scanner group.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::SensitiveDataScannerGroupAttributes,
    /// Relationships of the group.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::SensitiveDataScannerGroupRelationships>,
    /// Sensitive Data Scanner group type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::SensitiveDataScannerGroupType,
}

impl SensitiveDataScannerGroupCreate {
    pub fn new(
        attributes: crate::datadogV2::model::SensitiveDataScannerGroupAttributes,
        type_: crate::datadogV2::model::SensitiveDataScannerGroupType,
    ) -> SensitiveDataScannerGroupCreate {
        SensitiveDataScannerGroupCreate {
            attributes,
            relationships: None,
            type_,
        }
    }

    pub fn relationships(
        &mut self,
        value: crate::datadogV2::model::SensitiveDataScannerGroupRelationships,
    ) -> &mut Self {
        self.relationships = Some(value);
        self
    }
}
