// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `UCConfigPairDataAttributesConfigsItems` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UCConfigPairDataAttributesConfigsItems {
    /// The `items` `account_id`.
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// The `items` `client_id`.
    #[serde(rename = "client_id")]
    pub client_id: Option<String>,
    /// The `items` `created_at`.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// The `items` `dataset_type`.
    #[serde(rename = "dataset_type")]
    pub dataset_type: Option<String>,
    /// The `items` `error_messages`.
    #[serde(
        rename = "error_messages",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub error_messages: Option<Option<Vec<String>>>,
    /// The `items` `export_name`.
    #[serde(rename = "export_name")]
    pub export_name: Option<String>,
    /// The `items` `export_path`.
    #[serde(rename = "export_path")]
    pub export_path: Option<String>,
    /// The `items` `id`.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The `items` `months`.
    #[serde(rename = "months")]
    pub months: Option<i64>,
    /// The `items` `scope`.
    #[serde(rename = "scope")]
    pub scope: Option<String>,
    /// The `items` `status`.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// The `items` `status_updated_at`.
    #[serde(rename = "status_updated_at")]
    pub status_updated_at: Option<String>,
    /// The `items` `storage_account`.
    #[serde(rename = "storage_account")]
    pub storage_account: Option<String>,
    /// The `items` `storage_container`.
    #[serde(rename = "storage_container")]
    pub storage_container: Option<String>,
    /// The `items` `updated_at`.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UCConfigPairDataAttributesConfigsItems {
    pub fn new() -> UCConfigPairDataAttributesConfigsItems {
        UCConfigPairDataAttributesConfigsItems {
            account_id: None,
            client_id: None,
            created_at: None,
            dataset_type: None,
            error_messages: None,
            export_name: None,
            export_path: None,
            id: None,
            months: None,
            scope: None,
            status: None,
            status_updated_at: None,
            storage_account: None,
            storage_container: None,
            updated_at: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account_id(mut self, value: String) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn client_id(mut self, value: String) -> Self {
        self.client_id = Some(value);
        self
    }

    pub fn created_at(mut self, value: String) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn dataset_type(mut self, value: String) -> Self {
        self.dataset_type = Some(value);
        self
    }

    pub fn error_messages(mut self, value: Option<Vec<String>>) -> Self {
        self.error_messages = Some(value);
        self
    }

    pub fn export_name(mut self, value: String) -> Self {
        self.export_name = Some(value);
        self
    }

    pub fn export_path(mut self, value: String) -> Self {
        self.export_path = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn months(mut self, value: i64) -> Self {
        self.months = Some(value);
        self
    }

    pub fn scope(mut self, value: String) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
        self
    }

    pub fn status_updated_at(mut self, value: String) -> Self {
        self.status_updated_at = Some(value);
        self
    }

    pub fn storage_account(mut self, value: String) -> Self {
        self.storage_account = Some(value);
        self
    }

    pub fn storage_container(mut self, value: String) -> Self {
        self.storage_container = Some(value);
        self
    }

    pub fn updated_at(mut self, value: String) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for UCConfigPairDataAttributesConfigsItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UCConfigPairDataAttributesConfigsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UCConfigPairDataAttributesConfigsItemsVisitor;
        impl<'a> Visitor<'a> for UCConfigPairDataAttributesConfigsItemsVisitor {
            type Value = UCConfigPairDataAttributesConfigsItems;

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
                let mut error_messages: Option<Option<Vec<String>>> = None;
                let mut export_name: Option<String> = None;
                let mut export_path: Option<String> = None;
                let mut id: Option<String> = None;
                let mut months: Option<i64> = None;
                let mut scope: Option<String> = None;
                let mut status: Option<String> = None;
                let mut status_updated_at: Option<String> = None;
                let mut storage_account: Option<String> = None;
                let mut storage_container: Option<String> = None;
                let mut updated_at: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            if v.is_null() {
                                continue;
                            }
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_id" => {
                            if v.is_null() {
                                continue;
                            }
                            client_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dataset_type" => {
                            if v.is_null() {
                                continue;
                            }
                            dataset_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_messages" => {
                            error_messages =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "export_name" => {
                            if v.is_null() {
                                continue;
                            }
                            export_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "export_path" => {
                            if v.is_null() {
                                continue;
                            }
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
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
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
                            if v.is_null() {
                                continue;
                            }
                            storage_account =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "storage_container" => {
                            if v.is_null() {
                                continue;
                            }
                            storage_container =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = UCConfigPairDataAttributesConfigsItems {
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
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UCConfigPairDataAttributesConfigsItemsVisitor)
    }
}
