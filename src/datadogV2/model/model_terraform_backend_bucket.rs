// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Synchronization status for an S3 bucket.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TerraformBackendBucket {
    /// Name of the source S3 bucket.
    #[serde(rename = "bucket_name")]
    pub bucket_name: String,
    /// Error from the most recent failed synchronization, or an empty string otherwise.
    #[serde(rename = "last_sync_error")]
    pub last_sync_error: String,
    /// Most recent synchronization outcome, or pending if no outcome has been recorded.
    #[serde(rename = "last_sync_status")]
    pub last_sync_status: crate::datadogV2::model::TerraformBackendSyncStatus,
    /// Time of the most recent synchronization outcome, or null if none has been recorded.
    #[serialize_always]
    #[serde(rename = "last_sync_time")]
    pub last_sync_time: Option<chrono::DateTime<chrono::Utc>>,
    /// Identifier of the recurring synchronization job.
    #[serde(rename = "recurring_blob_sync_id")]
    pub recurring_blob_sync_id: String,
    /// Number of synchronized Terraform state files in the bucket.
    #[serde(rename = "statefile_count")]
    pub statefile_count: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TerraformBackendBucket {
    pub fn new(
        bucket_name: String,
        last_sync_error: String,
        last_sync_status: crate::datadogV2::model::TerraformBackendSyncStatus,
        last_sync_time: Option<chrono::DateTime<chrono::Utc>>,
        recurring_blob_sync_id: String,
        statefile_count: i64,
    ) -> TerraformBackendBucket {
        TerraformBackendBucket {
            bucket_name,
            last_sync_error,
            last_sync_status,
            last_sync_time,
            recurring_blob_sync_id,
            statefile_count,
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

impl<'de> Deserialize<'de> for TerraformBackendBucket {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TerraformBackendBucketVisitor;
        impl<'a> Visitor<'a> for TerraformBackendBucketVisitor {
            type Value = TerraformBackendBucket;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bucket_name: Option<String> = None;
                let mut last_sync_error: Option<String> = None;
                let mut last_sync_status: Option<
                    crate::datadogV2::model::TerraformBackendSyncStatus,
                > = None;
                let mut last_sync_time: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut recurring_blob_sync_id: Option<String> = None;
                let mut statefile_count: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bucket_name" => {
                            bucket_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_sync_error" => {
                            last_sync_error =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_sync_status" => {
                            last_sync_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _last_sync_status) = last_sync_status {
                                match _last_sync_status {
                                    crate::datadogV2::model::TerraformBackendSyncStatus::UnparsedObject(_last_sync_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "last_sync_time" => {
                            last_sync_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "recurring_blob_sync_id" => {
                            recurring_blob_sync_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "statefile_count" => {
                            statefile_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let bucket_name =
                    bucket_name.ok_or_else(|| M::Error::missing_field("bucket_name"))?;
                let last_sync_error =
                    last_sync_error.ok_or_else(|| M::Error::missing_field("last_sync_error"))?;
                let last_sync_status =
                    last_sync_status.ok_or_else(|| M::Error::missing_field("last_sync_status"))?;
                let last_sync_time =
                    last_sync_time.ok_or_else(|| M::Error::missing_field("last_sync_time"))?;
                let recurring_blob_sync_id = recurring_blob_sync_id
                    .ok_or_else(|| M::Error::missing_field("recurring_blob_sync_id"))?;
                let statefile_count =
                    statefile_count.ok_or_else(|| M::Error::missing_field("statefile_count"))?;

                let content = TerraformBackendBucket {
                    bucket_name,
                    last_sync_error,
                    last_sync_status,
                    last_sync_time,
                    recurring_blob_sync_id,
                    statefile_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TerraformBackendBucketVisitor)
    }
}
