// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A Sensitive Data Scanner configuration data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerConfigurationData {
    /// A Sensitive Data Scanner configuration.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::SensitiveDataScannerConfiguration>,
}

impl SensitiveDataScannerConfigurationData {
    pub fn new() -> SensitiveDataScannerConfigurationData {
        SensitiveDataScannerConfigurationData { data: None }
    }

    pub fn data(
        &mut self,
        value: crate::datadogV2::model::SensitiveDataScannerConfiguration,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }
}

impl Default for SensitiveDataScannerConfigurationData {
    fn default() -> Self {
        Self::new()
    }
}
