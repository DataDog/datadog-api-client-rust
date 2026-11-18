// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings of the Cloud Cost Management dataflow. Only the fields provided are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NebiusCloudCostCollectionIntegrationDataflowSettingsRequest {
    /// ID of the Nebius Object Storage (S3-compatible) access key used to read the cost export bucket. Required when this dataflow is enabled.
    #[serde(rename = "access_key_id")]
    pub access_key_id: Option<String>,
    /// Secret access key for the access key ID. Required when this dataflow is enabled. Omit it on update to keep the stored one; an empty string is rejected with a `422` response.
    #[serde(rename = "access_key_secret")]
    pub access_key_secret: Option<String>,
    /// Name of the Nebius Object Storage bucket holding your cost export. Required when this dataflow is enabled; a request that enables it without the bucket fields and access keys is rejected with a `422` response.
    #[serde(rename = "bucket_name")]
    pub bucket_name: Option<String>,
    /// Prefix path within the bucket that locates the cost export, combining the export name and path prefix.
    #[serde(rename = "bucket_prefix_path")]
    pub bucket_prefix_path: Option<String>,
    /// Region of the Nebius Object Storage bucket, such as `eu-north1`. Required when this dataflow is enabled.
    #[serde(rename = "bucket_region")]
    pub bucket_region: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NebiusCloudCostCollectionIntegrationDataflowSettingsRequest {
    pub fn new() -> NebiusCloudCostCollectionIntegrationDataflowSettingsRequest {
        NebiusCloudCostCollectionIntegrationDataflowSettingsRequest {
            access_key_id: None,
            access_key_secret: None,
            bucket_name: None,
            bucket_prefix_path: None,
            bucket_region: None,
            _unparsed: false,
        }
    }

    pub fn access_key_id(mut self, value: String) -> Self {
        self.access_key_id = Some(value);
        self
    }

    pub fn access_key_secret(mut self, value: String) -> Self {
        self.access_key_secret = Some(value);
        self
    }

    pub fn bucket_name(mut self, value: String) -> Self {
        self.bucket_name = Some(value);
        self
    }

    pub fn bucket_prefix_path(mut self, value: String) -> Self {
        self.bucket_prefix_path = Some(value);
        self
    }

    pub fn bucket_region(mut self, value: String) -> Self {
        self.bucket_region = Some(value);
        self
    }
}

impl Default for NebiusCloudCostCollectionIntegrationDataflowSettingsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for NebiusCloudCostCollectionIntegrationDataflowSettingsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NebiusCloudCostCollectionIntegrationDataflowSettingsRequestVisitor;
        impl<'a> Visitor<'a> for NebiusCloudCostCollectionIntegrationDataflowSettingsRequestVisitor {
            type Value = NebiusCloudCostCollectionIntegrationDataflowSettingsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut access_key_id: Option<String> = None;
                let mut access_key_secret: Option<String> = None;
                let mut bucket_name: Option<String> = None;
                let mut bucket_prefix_path: Option<String> = None;
                let mut bucket_region: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "access_key_id" => {
                            if v.is_null() {
                                continue;
                            }
                            access_key_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "access_key_secret" => {
                            if v.is_null() {
                                continue;
                            }
                            access_key_secret =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bucket_name" => {
                            if v.is_null() {
                                continue;
                            }
                            bucket_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bucket_prefix_path" => {
                            if v.is_null() {
                                continue;
                            }
                            bucket_prefix_path =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bucket_region" => {
                            if v.is_null() {
                                continue;
                            }
                            bucket_region =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = NebiusCloudCostCollectionIntegrationDataflowSettingsRequest {
                    access_key_id,
                    access_key_secret,
                    bucket_name,
                    bucket_prefix_path,
                    bucket_region,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(NebiusCloudCostCollectionIntegrationDataflowSettingsRequestVisitor)
    }
}
