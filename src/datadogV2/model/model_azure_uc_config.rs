// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Azure config.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureUCConfig {
    /// The tenant ID of the azure account.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The client ID of the Azure account.
    #[serde(rename = "client_id")]
    pub client_id: String,
    /// The timestamp when the Azure config was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// The dataset type of the Azure config.
    #[serde(rename = "dataset_type")]
    pub dataset_type: String,
    /// The error messages for the Azure config.
    #[serde(rename = "error_messages")]
    pub error_messages: Option<Vec<String>>,
    /// The name of the configured Azure Export.
    #[serde(rename = "export_name")]
    pub export_name: String,
    /// The path where the Azure Export is saved.
    #[serde(rename = "export_path")]
    pub export_path: String,
    /// The ID of the Azure config.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// The number of months the report has been backfilled.
    #[deprecated]
    #[serde(rename = "months")]
    pub months: Option<i32>,
    /// The scope of your observed subscription.
    #[serde(rename = "scope")]
    pub scope: String,
    /// The status of the Azure config.
    #[serde(rename = "status")]
    pub status: String,
    /// The timestamp when the Azure config status was last updated.
    #[serde(rename = "status_updated_at")]
    pub status_updated_at: Option<String>,
    /// The name of the storage account where the Azure Export is saved.
    #[serde(rename = "storage_account")]
    pub storage_account: String,
    /// The name of the storage container where the Azure Export is saved.
    #[serde(rename = "storage_container")]
    pub storage_container: String,
    /// The timestamp when the Azure config was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
}

impl AzureUCConfig {
    pub fn new(
        account_id: String,
        client_id: String,
        dataset_type: String,
        export_name: String,
        export_path: String,
        scope: String,
        status: String,
        storage_account: String,
        storage_container: String,
    ) -> AzureUCConfig {
        #[allow(deprecated)]
        AzureUCConfig {
            account_id,
            client_id,
            created_at: None,
            dataset_type,
            error_messages: None,
            export_name,
            export_path,
            id: None,
            months: None,
            scope,
            status,
            status_updated_at: None,
            storage_account,
            storage_container,
            updated_at: None,
        }
    }

    #[allow(deprecated)]
    pub fn created_at(&mut self, value: String) -> &mut Self {
        self.created_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn error_messages(&mut self, value: Vec<String>) -> &mut Self {
        self.error_messages = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn id(&mut self, value: i64) -> &mut Self {
        self.id = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn months(&mut self, value: i32) -> &mut Self {
        self.months = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn status_updated_at(&mut self, value: String) -> &mut Self {
        self.status_updated_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn updated_at(&mut self, value: String) -> &mut Self {
        self.updated_at = Some(value);
        self
    }
}
