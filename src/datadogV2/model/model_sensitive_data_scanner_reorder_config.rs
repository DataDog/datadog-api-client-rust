// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data related to the reordering of scanning groups.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerReorderConfig {
    /// ID of the configuration.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Relationships of the configuration.
    #[serde(rename = "relationships")]
    pub relationships:
        Option<crate::datadogV2::model::SensitiveDataScannerConfigurationRelationships>,
    /// Sensitive Data Scanner configuration type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SensitiveDataScannerConfigurationType>,
}

impl SensitiveDataScannerReorderConfig {
    pub fn new() -> SensitiveDataScannerReorderConfig {
        SensitiveDataScannerReorderConfig {
            id: None,
            relationships: None,
            type_: None,
        }
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn relationships(
        mut self,
        value: crate::datadogV2::model::SensitiveDataScannerConfigurationRelationships,
    ) -> Self {
        self.relationships = Some(value);
        self
    }

    pub fn type_(
        mut self,
        value: crate::datadogV2::model::SensitiveDataScannerConfigurationType,
    ) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for SensitiveDataScannerReorderConfig {
    fn default() -> Self {
        Self::new()
    }
}
