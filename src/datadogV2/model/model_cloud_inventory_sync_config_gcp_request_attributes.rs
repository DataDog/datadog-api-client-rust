// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// GCP settings for buckets involved in inventory reporting.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudInventorySyncConfigGCPRequestAttributes {
    /// GCS bucket name where Datadog reads inventory reports.
    #[serde(rename = "destination_bucket_name")]
    pub destination_bucket_name: String,
    /// GCP project ID for the inventory destination bucket.
    #[serde(rename = "project_id")]
    pub project_id: String,
    /// Service account email used to read the destination bucket.
    #[serde(rename = "service_account_email")]
    pub service_account_email: String,
    /// GCS bucket name that inventory reports are generated for.
    #[serde(rename = "source_bucket_name")]
    pub source_bucket_name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudInventorySyncConfigGCPRequestAttributes {
    pub fn new(
        destination_bucket_name: String,
        project_id: String,
        service_account_email: String,
        source_bucket_name: String,
    ) -> CloudInventorySyncConfigGCPRequestAttributes {
        CloudInventorySyncConfigGCPRequestAttributes {
            destination_bucket_name,
            project_id,
            service_account_email,
            source_bucket_name,
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

impl<'de> Deserialize<'de> for CloudInventorySyncConfigGCPRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudInventorySyncConfigGCPRequestAttributesVisitor;
        impl<'a> Visitor<'a> for CloudInventorySyncConfigGCPRequestAttributesVisitor {
            type Value = CloudInventorySyncConfigGCPRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut destination_bucket_name: Option<String> = None;
                let mut project_id: Option<String> = None;
                let mut service_account_email: Option<String> = None;
                let mut source_bucket_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "destination_bucket_name" => {
                            destination_bucket_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project_id" => {
                            project_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service_account_email" => {
                            service_account_email =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source_bucket_name" => {
                            source_bucket_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let destination_bucket_name = destination_bucket_name
                    .ok_or_else(|| M::Error::missing_field("destination_bucket_name"))?;
                let project_id = project_id.ok_or_else(|| M::Error::missing_field("project_id"))?;
                let service_account_email = service_account_email
                    .ok_or_else(|| M::Error::missing_field("service_account_email"))?;
                let source_bucket_name = source_bucket_name
                    .ok_or_else(|| M::Error::missing_field("source_bucket_name"))?;

                let content = CloudInventorySyncConfigGCPRequestAttributes {
                    destination_bucket_name,
                    project_id,
                    service_account_email,
                    source_bucket_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudInventorySyncConfigGCPRequestAttributesVisitor)
    }
}
