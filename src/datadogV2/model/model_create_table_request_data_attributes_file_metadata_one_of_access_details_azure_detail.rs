// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetail` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetail {
    /// The Azure client ID.
    #[serde(rename = "azure_client_id")]
    pub azure_client_id: String,
    /// The name of the Azure container.
    #[serde(rename = "azure_container_name")]
    pub azure_container_name: String,
    /// The name of the Azure storage account.
    #[serde(rename = "azure_storage_account_name")]
    pub azure_storage_account_name: String,
    /// The ID of the Azure tenant.
    #[serde(rename = "azure_tenant_id")]
    pub azure_tenant_id: String,
    /// The relative file path from the Azure container root to the CSV file.
    #[serde(rename = "file_path")]
    pub file_path: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetail {
    pub fn new(
        azure_client_id: String,
        azure_container_name: String,
        azure_storage_account_name: String,
        azure_tenant_id: String,
        file_path: String,
    ) -> CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetail {
        CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetail {
            azure_client_id,
            azure_container_name,
            azure_storage_account_name,
            azure_tenant_id,
            file_path,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de>
    for CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetail
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetailVisitor;
        impl<'a> Visitor<'a>
            for CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetailVisitor
        {
            type Value = CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetail;

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
                            azure_client_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "azure_container_name" => {
                            azure_container_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "azure_storage_account_name" => {
                            azure_storage_account_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "azure_tenant_id" => {
                            azure_tenant_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "file_path" => {
                            file_path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let azure_client_id =
                    azure_client_id.ok_or_else(|| M::Error::missing_field("azure_client_id"))?;
                let azure_container_name = azure_container_name
                    .ok_or_else(|| M::Error::missing_field("azure_container_name"))?;
                let azure_storage_account_name = azure_storage_account_name
                    .ok_or_else(|| M::Error::missing_field("azure_storage_account_name"))?;
                let azure_tenant_id =
                    azure_tenant_id.ok_or_else(|| M::Error::missing_field("azure_tenant_id"))?;
                let file_path = file_path.ok_or_else(|| M::Error::missing_field("file_path"))?;

                let content =
                    CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetail {
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
            CreateTableRequestDataAttributesFileMetadataOneOfAccessDetailsAzureDetailVisitor,
        )
    }
}
