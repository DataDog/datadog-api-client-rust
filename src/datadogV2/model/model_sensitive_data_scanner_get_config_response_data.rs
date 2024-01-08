// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response data related to the scanning groups.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerGetConfigResponseData {
    /// Attributes of the Sensitive Data configuration.
    #[serde(rename = "attributes")]
    pub attributes: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// ID of the configuration.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Relationships of the configuration.
    #[serde(rename = "relationships")]
    pub relationships:
        Option<Box<crate::datadogV2::model::SensitiveDataScannerConfigurationRelationships>>,
    /// Sensitive Data Scanner configuration type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SensitiveDataScannerConfigurationType>,
}

impl SensitiveDataScannerGetConfigResponseData {
    pub fn new() -> SensitiveDataScannerGetConfigResponseData {
        SensitiveDataScannerGetConfigResponseData {
            attributes: None,
            id: None,
            relationships: None,
            type_: None,
        }
    }
}
impl Default for SensitiveDataScannerGetConfigResponseData {
    fn default() -> Self {
        Self::new()
    }
}
