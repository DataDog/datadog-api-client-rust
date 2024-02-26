// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data related to the update of a rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerRuleUpdate {
    /// Attributes of the Sensitive Data Scanner rule.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::SensitiveDataScannerRuleAttributes>,
    /// ID of the rule.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Relationships of a scanning rule.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::SensitiveDataScannerRuleRelationships>,
    /// Sensitive Data Scanner rule type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SensitiveDataScannerRuleType>,
}

impl SensitiveDataScannerRuleUpdate {
    pub fn new() -> SensitiveDataScannerRuleUpdate {
        SensitiveDataScannerRuleUpdate {
            attributes: None,
            id: None,
            relationships: None,
            type_: None,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::SensitiveDataScannerRuleAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn relationships(
        &mut self,
        value: crate::datadogV2::model::SensitiveDataScannerRuleRelationships,
    ) -> &mut Self {
        self.relationships = Some(value);
        self
    }

    pub fn type_(
        &mut self,
        value: crate::datadogV2::model::SensitiveDataScannerRuleType,
    ) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for SensitiveDataScannerRuleUpdate {
    fn default() -> Self {
        Self::new()
    }
}
