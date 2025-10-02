// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `amazon_s3` destination sends your logs in Datadog-rehydratable format to an Amazon S3 bucket for archiving.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineAmazonS3Destination {
    /// AWS authentication credentials used for accessing AWS services such as S3.
    /// If omitted, the system’s default credentials are used (for example, the IAM role and environment variables).
    #[serde(rename = "auth")]
    pub auth: Option<crate::datadogV2::model::ObservabilityPipelineAwsAuth>,
    /// S3 bucket name.
    #[serde(rename = "bucket")]
    pub bucket: String,
    /// Unique identifier for the destination component.
    #[serde(rename = "id")]
    pub id: String,
    /// A list of component IDs whose output is used as the `input` for this component.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// Optional prefix for object keys.
    #[serde(rename = "key_prefix")]
    pub key_prefix: Option<String>,
    /// AWS region of the S3 bucket.
    #[serde(rename = "region")]
    pub region: String,
    /// S3 storage class.
    #[serde(rename = "storage_class")]
    pub storage_class:
        crate::datadogV2::model::ObservabilityPipelineAmazonS3DestinationStorageClass,
    /// Configuration for enabling TLS encryption between the pipeline component and external services.
    #[serde(rename = "tls")]
    pub tls: Option<crate::datadogV2::model::ObservabilityPipelineTls>,
    /// The destination type. Always `amazon_s3`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineAmazonS3DestinationType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineAmazonS3Destination {
    pub fn new(
        bucket: String,
        id: String,
        inputs: Vec<String>,
        region: String,
        storage_class: crate::datadogV2::model::ObservabilityPipelineAmazonS3DestinationStorageClass,
        type_: crate::datadogV2::model::ObservabilityPipelineAmazonS3DestinationType,
    ) -> ObservabilityPipelineAmazonS3Destination {
        ObservabilityPipelineAmazonS3Destination {
            auth: None,
            bucket,
            id,
            inputs,
            key_prefix: None,
            region,
            storage_class,
            tls: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn auth(mut self, value: crate::datadogV2::model::ObservabilityPipelineAwsAuth) -> Self {
        self.auth = Some(value);
        self
    }

    pub fn key_prefix(mut self, value: String) -> Self {
        self.key_prefix = Some(value);
        self
    }

    pub fn tls(mut self, value: crate::datadogV2::model::ObservabilityPipelineTls) -> Self {
        self.tls = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineAmazonS3Destination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineAmazonS3DestinationVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineAmazonS3DestinationVisitor {
            type Value = ObservabilityPipelineAmazonS3Destination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth: Option<crate::datadogV2::model::ObservabilityPipelineAwsAuth> = None;
                let mut bucket: Option<String> = None;
                let mut id: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut key_prefix: Option<String> = None;
                let mut region: Option<String> = None;
                let mut storage_class: Option<
                    crate::datadogV2::model::ObservabilityPipelineAmazonS3DestinationStorageClass,
                > = None;
                let mut tls: Option<crate::datadogV2::model::ObservabilityPipelineTls> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineAmazonS3DestinationType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth" => {
                            if v.is_null() {
                                continue;
                            }
                            auth = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bucket" => {
                            bucket = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputs" => {
                            inputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key_prefix" => {
                            if v.is_null() {
                                continue;
                            }
                            key_prefix = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "region" => {
                            region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "storage_class" => {
                            storage_class =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _storage_class) = storage_class {
                                match _storage_class {
                                    crate::datadogV2::model::ObservabilityPipelineAmazonS3DestinationStorageClass::UnparsedObject(_storage_class) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "tls" => {
                            if v.is_null() {
                                continue;
                            }
                            tls = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineAmazonS3DestinationType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let bucket = bucket.ok_or_else(|| M::Error::missing_field("bucket"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;
                let region = region.ok_or_else(|| M::Error::missing_field("region"))?;
                let storage_class =
                    storage_class.ok_or_else(|| M::Error::missing_field("storage_class"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineAmazonS3Destination {
                    auth,
                    bucket,
                    id,
                    inputs,
                    key_prefix,
                    region,
                    storage_class,
                    tls,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineAmazonS3DestinationVisitor)
    }
}
