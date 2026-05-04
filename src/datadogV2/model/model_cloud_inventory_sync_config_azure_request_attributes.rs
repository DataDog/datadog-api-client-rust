// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Azure settings for the storage account and container with inventory data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudInventorySyncConfigAzureRequestAttributes {
    /// Azure AD application (client) ID used for access.
    #[serde(rename = "client_id")]
    pub client_id: String,
    /// Blob container name.
    #[serde(rename = "container")]
    pub container: String,
    /// Resource group containing the storage account.
    #[serde(rename = "resource_group")]
    pub resource_group: String,
    /// Storage account name.
    #[serde(rename = "storage_account")]
    pub storage_account: String,
    /// Azure subscription ID.
    #[serde(rename = "subscription_id")]
    pub subscription_id: String,
    /// Azure AD tenant ID.
    #[serde(rename = "tenant_id")]
    pub tenant_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudInventorySyncConfigAzureRequestAttributes {
    pub fn new(
        client_id: String,
        container: String,
        resource_group: String,
        storage_account: String,
        subscription_id: String,
        tenant_id: String,
    ) -> CloudInventorySyncConfigAzureRequestAttributes {
        CloudInventorySyncConfigAzureRequestAttributes {
            client_id,
            container,
            resource_group,
            storage_account,
            subscription_id,
            tenant_id,
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

impl<'de> Deserialize<'de> for CloudInventorySyncConfigAzureRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudInventorySyncConfigAzureRequestAttributesVisitor;
        impl<'a> Visitor<'a> for CloudInventorySyncConfigAzureRequestAttributesVisitor {
            type Value = CloudInventorySyncConfigAzureRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut client_id: Option<String> = None;
                let mut container: Option<String> = None;
                let mut resource_group: Option<String> = None;
                let mut storage_account: Option<String> = None;
                let mut subscription_id: Option<String> = None;
                let mut tenant_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "client_id" => {
                            client_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "container" => {
                            container = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_group" => {
                            resource_group =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "storage_account" => {
                            storage_account =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subscription_id" => {
                            subscription_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tenant_id" => {
                            tenant_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let client_id = client_id.ok_or_else(|| M::Error::missing_field("client_id"))?;
                let container = container.ok_or_else(|| M::Error::missing_field("container"))?;
                let resource_group =
                    resource_group.ok_or_else(|| M::Error::missing_field("resource_group"))?;
                let storage_account =
                    storage_account.ok_or_else(|| M::Error::missing_field("storage_account"))?;
                let subscription_id =
                    subscription_id.ok_or_else(|| M::Error::missing_field("subscription_id"))?;
                let tenant_id = tenant_id.ok_or_else(|| M::Error::missing_field("tenant_id"))?;

                let content = CloudInventorySyncConfigAzureRequestAttributes {
                    client_id,
                    container,
                    resource_group,
                    storage_account,
                    subscription_id,
                    tenant_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudInventorySyncConfigAzureRequestAttributesVisitor)
    }
}
