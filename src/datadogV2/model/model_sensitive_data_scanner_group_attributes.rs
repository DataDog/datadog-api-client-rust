// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of the Sensitive Data Scanner group.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerGroupAttributes {
    /// Description of the group.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Filter for the Scanning Group.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::SensitiveDataScannerFilter>,
    /// Whether or not the group is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the group.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// List of products the scanning group applies.
    #[serde(rename = "product_list")]
    pub product_list: Option<Vec<crate::datadogV2::model::SensitiveDataScannerProduct>>,
}

impl SensitiveDataScannerGroupAttributes {
    pub fn new() -> SensitiveDataScannerGroupAttributes {
        SensitiveDataScannerGroupAttributes {
            description: None,
            filter: None,
            is_enabled: None,
            name: None,
            product_list: None,
        }
    }

    pub fn description(&mut self, value: String) -> &mut Self {
        self.description = Some(value);
        self
    }

    pub fn filter(
        &mut self,
        value: crate::datadogV2::model::SensitiveDataScannerFilter,
    ) -> &mut Self {
        self.filter = Some(value);
        self
    }

    pub fn is_enabled(&mut self, value: bool) -> &mut Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn product_list(
        &mut self,
        value: Vec<crate::datadogV2::model::SensitiveDataScannerProduct>,
    ) -> &mut Self {
        self.product_list = Some(value);
        self
    }
}

impl Default for SensitiveDataScannerGroupAttributes {
    fn default() -> Self {
        Self::new()
    }
}
