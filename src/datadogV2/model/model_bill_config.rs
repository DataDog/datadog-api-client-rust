// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Bill config.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BillConfig {
    /// The name of the configured Azure Export.
    #[serde(rename = "export_name")]
    pub export_name: String,
    /// The path where the Azure Export is saved.
    #[serde(rename = "export_path")]
    pub export_path: String,
    /// The name of the storage account where the Azure Export is saved.
    #[serde(rename = "storage_account")]
    pub storage_account: String,
    /// The name of the storage container where the Azure Export is saved.
    #[serde(rename = "storage_container")]
    pub storage_container: String,
}

impl BillConfig {
    pub fn new(
        export_name: String,
        export_path: String,
        storage_account: String,
        storage_container: String,
    ) -> BillConfig {
        BillConfig {
            export_name,
            export_path,
            storage_account,
            storage_container,
        }
    }
}
