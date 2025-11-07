// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Azure Blob Storage access configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetail {
    /// Azure service principal (application) client ID with permissions to read from the container.
    #[serde(rename = "azure_client_id")]
    pub azure_client_id: Option<String>,
    /// Azure Blob Storage container containing the CSV file.
    #[serde(rename = "azure_container_name")]
    pub azure_container_name: Option<String>,
    /// Azure storage account where the container is located.
    #[serde(rename = "azure_storage_account_name")]
    pub azure_storage_account_name: Option<String>,
    /// Azure Active Directory tenant ID.
    #[serde(rename = "azure_tenant_id")]
    pub azure_tenant_id: Option<String>,
    /// The relative file path from the Azure container root to the CSV file.
    #[serde(rename = "file_path")]
    pub file_path: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetail {
    pub fn new() -> PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetail {
        PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetail {
            azure_client_id: None,
            azure_container_name: None,
            azure_storage_account_name: None,
            azure_tenant_id: None,
            file_path: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn azure_client_id(mut self, value: String) -> Self {
        self.azure_client_id = Some(value);
        self
    }

    pub fn azure_container_name(mut self, value: String) -> Self {
        self.azure_container_name = Some(value);
        self
    }

    pub fn azure_storage_account_name(mut self, value: String) -> Self {
        self.azure_storage_account_name = Some(value);
        self
    }

    pub fn azure_tenant_id(mut self, value: String) -> Self {
        self.azure_tenant_id = Some(value);
        self
    }

    pub fn file_path(mut self, value: String) -> Self {
        self.file_path = Some(value);
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

impl Default for PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetail {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de>
    for PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetail
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetailVisitor;
        impl<'a> Visitor<'a>
            for PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetailVisitor
        {
            type Value = PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetail;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut azure_client_id: Option<String> = None;
                let mut azure_container_name: Option<String> = None;
                let mut azure_storage_account_name: Option<String> = None;
                let mut azure_tenant_id: Option<String> = None;
                let mut file_path: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "azure_client_id" => {
                            if v.is_null() {
                                continue;
                            }
                            azure_client_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "azure_container_name" => {
                            if v.is_null() {
                                continue;
                            }
                            azure_container_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "azure_storage_account_name" => {
                            if v.is_null() {
                                continue;
                            }
                            azure_storage_account_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "azure_tenant_id" => {
                            if v.is_null() {
                                continue;
                            }
                            azure_tenant_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "file_path" => {
                            if v.is_null() {
                                continue;
                            }
                            file_path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content =
                    PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetail {
                        azure_client_id,
                        azure_container_name,
                        azure_storage_account_name,
                        azure_tenant_id,
                        file_path,
                        additional_properties,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            PatchTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetailVisitor,
        )
    }
}
