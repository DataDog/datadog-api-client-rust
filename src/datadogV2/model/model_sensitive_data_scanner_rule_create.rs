// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data related to the creation of a rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerRuleCreate {
    /// Attributes of the Sensitive Data Scanner rule.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::SensitiveDataScannerRuleAttributes,
    /// Relationships of a scanning rule.
    #[serde(rename = "relationships")]
    pub relationships: crate::datadogV2::model::SensitiveDataScannerRuleRelationships,
    /// Sensitive Data Scanner rule type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::SensitiveDataScannerRuleType,
}

impl SensitiveDataScannerRuleCreate {
    pub fn new(
        attributes: crate::datadogV2::model::SensitiveDataScannerRuleAttributes,
        relationships: crate::datadogV2::model::SensitiveDataScannerRuleRelationships,
        type_: crate::datadogV2::model::SensitiveDataScannerRuleType,
    ) -> SensitiveDataScannerRuleCreate {
        SensitiveDataScannerRuleCreate {
            attributes,
            relationships,
            type_,
        }
    }
}
