// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for a Storage Management configuration. Fields other than id may be empty in the response immediately after a create or update; subsequent reads return the full configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudInventorySyncConfigAttributes {
    /// AWS account ID for the inventory bucket.
    #[serde(rename = "aws_account_id")]
    pub aws_account_id: String,
    /// AWS S3 bucket name for inventory files.
    #[serde(rename = "aws_bucket_name")]
    pub aws_bucket_name: String,
    /// AWS Region for the inventory bucket.
    #[serde(rename = "aws_region")]
    pub aws_region: String,
    /// Azure AD application (client) ID.
    #[serde(rename = "azure_client_id")]
    pub azure_client_id: String,
    /// Azure blob container name.
    #[serde(rename = "azure_container_name")]
    pub azure_container_name: String,
    /// Azure storage account name.
    #[serde(rename = "azure_storage_account_name")]
    pub azure_storage_account_name: String,
    /// Azure AD tenant ID.
    #[serde(rename = "azure_tenant_id")]
    pub azure_tenant_id: String,
    /// Cloud provider for this sync configuration (`aws`, `gcp`, or `azure`). For requests, must match the provider block supplied under `attributes`.
    #[serde(rename = "cloud_provider")]
    pub cloud_provider: crate::datadogV2::model::CloudInventoryCloudProviderId,
    /// Human-readable error detail when sync is unhealthy.
    #[serde(rename = "error")]
    pub error: String,
    /// Machine-readable error code when sync is unhealthy.
    #[serde(rename = "error_code")]
    pub error_code: String,
    /// GCS bucket name for inventory files Datadog reads.
    #[serde(rename = "gcp_bucket_name")]
    pub gcp_bucket_name: String,
    /// GCP project ID.
    #[serde(rename = "gcp_project_id")]
    pub gcp_project_id: String,
    /// Service account email for bucket access.
    #[serde(rename = "gcp_service_account_email")]
    pub gcp_service_account_email: String,
    /// Object key prefix where inventory reports are written. Returns / when reports are written at the bucket root.
    #[serde(rename = "prefix")]
    pub prefix: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudInventorySyncConfigAttributes {
    pub fn new(
        aws_account_id: String,
        aws_bucket_name: String,
        aws_region: String,
        azure_client_id: String,
        azure_container_name: String,
        azure_storage_account_name: String,
        azure_tenant_id: String,
        cloud_provider: crate::datadogV2::model::CloudInventoryCloudProviderId,
        error: String,
        error_code: String,
        gcp_bucket_name: String,
        gcp_project_id: String,
        gcp_service_account_email: String,
        prefix: String,
    ) -> CloudInventorySyncConfigAttributes {
        CloudInventorySyncConfigAttributes {
            aws_account_id,
            aws_bucket_name,
            aws_region,
            azure_client_id,
            azure_container_name,
            azure_storage_account_name,
            azure_tenant_id,
            cloud_provider,
            error,
            error_code,
            gcp_bucket_name,
            gcp_project_id,
            gcp_service_account_email,
            prefix,
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

impl<'de> Deserialize<'de> for CloudInventorySyncConfigAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudInventorySyncConfigAttributesVisitor;
        impl<'a> Visitor<'a> for CloudInventorySyncConfigAttributesVisitor {
            type Value = CloudInventorySyncConfigAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aws_account_id: Option<String> = None;
                let mut aws_bucket_name: Option<String> = None;
                let mut aws_region: Option<String> = None;
                let mut azure_client_id: Option<String> = None;
                let mut azure_container_name: Option<String> = None;
                let mut azure_storage_account_name: Option<String> = None;
                let mut azure_tenant_id: Option<String> = None;
                let mut cloud_provider: Option<
                    crate::datadogV2::model::CloudInventoryCloudProviderId,
                > = None;
                let mut error: Option<String> = None;
                let mut error_code: Option<String> = None;
                let mut gcp_bucket_name: Option<String> = None;
                let mut gcp_project_id: Option<String> = None;
                let mut gcp_service_account_email: Option<String> = None;
                let mut prefix: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aws_account_id" => {
                            aws_account_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "aws_bucket_name" => {
                            aws_bucket_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "aws_region" => {
                            aws_region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        "cloud_provider" => {
                            cloud_provider =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _cloud_provider) = cloud_provider {
                                match _cloud_provider {
                                    crate::datadogV2::model::CloudInventoryCloudProviderId::UnparsedObject(_cloud_provider) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "error" => {
                            error = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_code" => {
                            error_code = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gcp_bucket_name" => {
                            gcp_bucket_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gcp_project_id" => {
                            gcp_project_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gcp_service_account_email" => {
                            gcp_service_account_email =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prefix" => {
                            prefix = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let aws_account_id =
                    aws_account_id.ok_or_else(|| M::Error::missing_field("aws_account_id"))?;
                let aws_bucket_name =
                    aws_bucket_name.ok_or_else(|| M::Error::missing_field("aws_bucket_name"))?;
                let aws_region = aws_region.ok_or_else(|| M::Error::missing_field("aws_region"))?;
                let azure_client_id =
                    azure_client_id.ok_or_else(|| M::Error::missing_field("azure_client_id"))?;
                let azure_container_name = azure_container_name
                    .ok_or_else(|| M::Error::missing_field("azure_container_name"))?;
                let azure_storage_account_name = azure_storage_account_name
                    .ok_or_else(|| M::Error::missing_field("azure_storage_account_name"))?;
                let azure_tenant_id =
                    azure_tenant_id.ok_or_else(|| M::Error::missing_field("azure_tenant_id"))?;
                let cloud_provider =
                    cloud_provider.ok_or_else(|| M::Error::missing_field("cloud_provider"))?;
                let error = error.ok_or_else(|| M::Error::missing_field("error"))?;
                let error_code = error_code.ok_or_else(|| M::Error::missing_field("error_code"))?;
                let gcp_bucket_name =
                    gcp_bucket_name.ok_or_else(|| M::Error::missing_field("gcp_bucket_name"))?;
                let gcp_project_id =
                    gcp_project_id.ok_or_else(|| M::Error::missing_field("gcp_project_id"))?;
                let gcp_service_account_email = gcp_service_account_email
                    .ok_or_else(|| M::Error::missing_field("gcp_service_account_email"))?;
                let prefix = prefix.ok_or_else(|| M::Error::missing_field("prefix"))?;

                let content = CloudInventorySyncConfigAttributes {
                    aws_account_id,
                    aws_bucket_name,
                    aws_region,
                    azure_client_id,
                    azure_container_name,
                    azure_storage_account_name,
                    azure_tenant_id,
                    cloud_provider,
                    error,
                    error_code,
                    gcp_bucket_name,
                    gcp_project_id,
                    gcp_service_account_email,
                    prefix,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudInventorySyncConfigAttributesVisitor)
    }
}
