// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationships of the group.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerGroupRelationships {
    /// A Sensitive Data Scanner configuration data.
    #[serde(rename = "configuration")]
    pub configuration: Option<crate::datadogV2::model::SensitiveDataScannerConfigurationData>,
    /// Rules included in the group.
    #[serde(rename = "rules")]
    pub rules: Option<crate::datadogV2::model::SensitiveDataScannerRuleData>,
}

impl SensitiveDataScannerGroupRelationships {
    pub fn new() -> SensitiveDataScannerGroupRelationships {
        SensitiveDataScannerGroupRelationships {
            configuration: None,
            rules: None,
        }
    }

    pub fn with_configuration(
        &mut self,
        value: crate::datadogV2::model::SensitiveDataScannerConfigurationData,
    ) -> &mut Self {
        self.configuration = Some(value);
        self
    }

    pub fn with_rules(
        &mut self,
        value: crate::datadogV2::model::SensitiveDataScannerRuleData,
    ) -> &mut Self {
        self.rules = Some(value);
        self
    }
}
impl Default for SensitiveDataScannerGroupRelationships {
    fn default() -> Self {
        Self::new()
    }
}
