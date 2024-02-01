// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationships of the configuration.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerConfigurationRelationships {
    /// List of groups, ordered.
    #[serde(rename = "groups")]
    pub groups: Option<crate::datadogV2::model::SensitiveDataScannerGroupList>,
}

impl SensitiveDataScannerConfigurationRelationships {
    pub fn new() -> SensitiveDataScannerConfigurationRelationships {
        SensitiveDataScannerConfigurationRelationships { groups: None }
    }

    pub fn groups(
        &mut self,
        value: crate::datadogV2::model::SensitiveDataScannerGroupList,
    ) -> &mut Self {
        self.groups = Some(value);
        self
    }
}

impl Default for SensitiveDataScannerConfigurationRelationships {
    fn default() -> Self {
        Self::new()
    }
}
