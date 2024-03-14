// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Azure config.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn created_at(mut self, value: String) -> Self {
        self.created_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn error_messages(mut self, value: Vec<String>) -> Self {
        self.error_messages = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn months(mut self, value: i32) -> Self {
        self.months = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn status_updated_at(mut self, value: String) -> Self {
        self.status_updated_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn updated_at(mut self, value: String) -> Self {
        self.updated_at = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for AzureUCConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AzureUCConfigVisitor;
        impl<'a> Visitor<'a> for AzureUCConfigVisitor {
            type Value = AzureUCConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut client_id: Option<String> = None;
                let mut created_at: Option<String> = None;
                let mut dataset_type: Option<String> = None;
                let mut error_messages: Option<Vec<String>> = None;
                let mut export_name: Option<String> = None;
                let mut export_path: Option<String> = None;
                let mut id: Option<i64> = None;
                let mut months: Option<i32> = None;
                let mut scope: Option<String> = None;
                let mut status: Option<String> = None;
                let mut status_updated_at: Option<String> = None;
                let mut storage_account: Option<String> = None;
                let mut storage_container: Option<String> = None;
                let mut updated_at: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_id" => {
                            client_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dataset_type" => {
                            dataset_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_messages" => {
                            if v.is_null() {
                                continue;
                            }
                            error_messages =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "export_name" => {
                            export_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "export_path" => {
                            export_path =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "months" => {
                            if v.is_null() {
                                continue;
                            }
                            months = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status_updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            status_updated_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "storage_account" => {
                            storage_account =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "storage_container" => {
                            storage_container =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let account_id = account_id.ok_or_else(|| M::Error::missing_field("account_id"))?;
                let client_id = client_id.ok_or_else(|| M::Error::missing_field("client_id"))?;
                let dataset_type =
                    dataset_type.ok_or_else(|| M::Error::missing_field("dataset_type"))?;
                let export_name =
                    export_name.ok_or_else(|| M::Error::missing_field("export_name"))?;
                let export_path =
                    export_path.ok_or_else(|| M::Error::missing_field("export_path"))?;
                let scope = scope.ok_or_else(|| M::Error::missing_field("scope"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let storage_account =
                    storage_account.ok_or_else(|| M::Error::missing_field("storage_account"))?;
                let storage_container = storage_container
                    .ok_or_else(|| M::Error::missing_field("storage_container"))?;

                #[allow(deprecated)]
                let content = AzureUCConfig {
                    account_id,
                    client_id,
                    created_at,
                    dataset_type,
                    error_messages,
                    export_name,
                    export_path,
                    id,
                    months,
                    scope,
                    status,
                    status_updated_at,
                    storage_account,
                    storage_container,
                    updated_at,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AzureUCConfigVisitor)
    }
}
