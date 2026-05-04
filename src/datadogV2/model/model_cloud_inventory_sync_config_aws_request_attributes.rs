// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// AWS settings for the S3 bucket Storage Management reads inventory reports from.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudInventorySyncConfigAWSRequestAttributes {
    /// AWS account ID that owns the inventory bucket.
    #[serde(rename = "aws_account_id")]
    pub aws_account_id: String,
    /// Name of the S3 bucket containing inventory files.
    #[serde(rename = "destination_bucket_name")]
    pub destination_bucket_name: String,
    /// AWS Region of the inventory bucket.
    #[serde(rename = "destination_bucket_region")]
    pub destination_bucket_region: String,
    /// Object key prefix where inventory reports are written. Omit or set to / when reports are written at the bucket root.
    #[serde(rename = "destination_prefix")]
    pub destination_prefix: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudInventorySyncConfigAWSRequestAttributes {
    pub fn new(
        aws_account_id: String,
        destination_bucket_name: String,
        destination_bucket_region: String,
    ) -> CloudInventorySyncConfigAWSRequestAttributes {
        CloudInventorySyncConfigAWSRequestAttributes {
            aws_account_id,
            destination_bucket_name,
            destination_bucket_region,
            destination_prefix: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn destination_prefix(mut self, value: String) -> Self {
        self.destination_prefix = Some(value);
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

impl<'de> Deserialize<'de> for CloudInventorySyncConfigAWSRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudInventorySyncConfigAWSRequestAttributesVisitor;
        impl<'a> Visitor<'a> for CloudInventorySyncConfigAWSRequestAttributesVisitor {
            type Value = CloudInventorySyncConfigAWSRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aws_account_id: Option<String> = None;
                let mut destination_bucket_name: Option<String> = None;
                let mut destination_bucket_region: Option<String> = None;
                let mut destination_prefix: Option<String> = None;
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
                        "destination_bucket_name" => {
                            destination_bucket_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "destination_bucket_region" => {
                            destination_bucket_region =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "destination_prefix" => {
                            if v.is_null() {
                                continue;
                            }
                            destination_prefix =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let destination_bucket_name = destination_bucket_name
                    .ok_or_else(|| M::Error::missing_field("destination_bucket_name"))?;
                let destination_bucket_region = destination_bucket_region
                    .ok_or_else(|| M::Error::missing_field("destination_bucket_region"))?;

                let content = CloudInventorySyncConfigAWSRequestAttributes {
                    aws_account_id,
                    destination_bucket_name,
                    destination_bucket_region,
                    destination_prefix,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudInventorySyncConfigAWSRequestAttributesVisitor)
    }
}
