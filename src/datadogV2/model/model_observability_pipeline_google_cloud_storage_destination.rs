// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `google_cloud_storage` destination stores logs in a Google Cloud Storage (GCS) bucket.
/// It requires a bucket name, Google Cloud authentication, and metadata fields.
///
/// **Supported pipeline types:** logs
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineGoogleCloudStorageDestination {
    /// Access control list setting for objects written to the bucket.
    #[serde(rename = "acl")]
    pub acl: Option<crate::datadogV2::model::ObservabilityPipelineGoogleCloudStorageDestinationAcl>,
    /// Google Cloud credentials used to authenticate with Google Cloud Storage.
    #[serde(rename = "auth")]
    pub auth: Option<crate::datadogV2::model::ObservabilityPipelineGcpAuth>,
    /// Name of the GCS bucket.
    #[serde(rename = "bucket")]
    pub bucket: String,
    /// Configuration for buffer settings on destination components.
    #[serde(rename = "buffer")]
    pub buffer: Option<crate::datadogV2::model::ObservabilityPipelineBufferOptions>,
    /// Unique identifier for the destination component.
    #[serde(rename = "id")]
    pub id: String,
    /// A list of component IDs whose output is used as the `input` for this component.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// Optional prefix for object keys within the GCS bucket.
    #[serde(rename = "key_prefix")]
    pub key_prefix: Option<String>,
    /// Custom metadata to attach to each object uploaded to the GCS bucket.
    #[serde(rename = "metadata")]
    pub metadata: Option<Vec<crate::datadogV2::model::ObservabilityPipelineMetadataEntry>>,
    /// Storage class used for objects stored in GCS.
    #[serde(rename = "storage_class")]
    pub storage_class:
        crate::datadogV2::model::ObservabilityPipelineGoogleCloudStorageDestinationStorageClass,
    /// The destination type. Always `google_cloud_storage`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineGoogleCloudStorageDestinationType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineGoogleCloudStorageDestination {
    pub fn new(
        bucket: String,
        id: String,
        inputs: Vec<String>,
        storage_class: crate::datadogV2::model::ObservabilityPipelineGoogleCloudStorageDestinationStorageClass,
        type_: crate::datadogV2::model::ObservabilityPipelineGoogleCloudStorageDestinationType,
    ) -> ObservabilityPipelineGoogleCloudStorageDestination {
        ObservabilityPipelineGoogleCloudStorageDestination {
            acl: None,
            auth: None,
            bucket,
            buffer: None,
            id,
            inputs,
            key_prefix: None,
            metadata: None,
            storage_class,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn acl(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineGoogleCloudStorageDestinationAcl,
    ) -> Self {
        self.acl = Some(value);
        self
    }

    pub fn auth(mut self, value: crate::datadogV2::model::ObservabilityPipelineGcpAuth) -> Self {
        self.auth = Some(value);
        self
    }

    pub fn buffer(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineBufferOptions,
    ) -> Self {
        self.buffer = Some(value);
        self
    }

    pub fn key_prefix(mut self, value: String) -> Self {
        self.key_prefix = Some(value);
        self
    }

    pub fn metadata(
        mut self,
        value: Vec<crate::datadogV2::model::ObservabilityPipelineMetadataEntry>,
    ) -> Self {
        self.metadata = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineGoogleCloudStorageDestination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineGoogleCloudStorageDestinationVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineGoogleCloudStorageDestinationVisitor {
            type Value = ObservabilityPipelineGoogleCloudStorageDestination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut acl: Option<
                    crate::datadogV2::model::ObservabilityPipelineGoogleCloudStorageDestinationAcl,
                > = None;
                let mut auth: Option<crate::datadogV2::model::ObservabilityPipelineGcpAuth> = None;
                let mut bucket: Option<String> = None;
                let mut buffer: Option<
                    crate::datadogV2::model::ObservabilityPipelineBufferOptions,
                > = None;
                let mut id: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut key_prefix: Option<String> = None;
                let mut metadata: Option<
                    Vec<crate::datadogV2::model::ObservabilityPipelineMetadataEntry>,
                > = None;
                let mut storage_class: Option<crate::datadogV2::model::ObservabilityPipelineGoogleCloudStorageDestinationStorageClass> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineGoogleCloudStorageDestinationType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "acl" => {
                            if v.is_null() {
                                continue;
                            }
                            acl = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _acl) = acl {
                                match _acl {
                                    crate::datadogV2::model::ObservabilityPipelineGoogleCloudStorageDestinationAcl::UnparsedObject(_acl) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "auth" => {
                            if v.is_null() {
                                continue;
                            }
                            auth = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bucket" => {
                            bucket = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "buffer" => {
                            if v.is_null() {
                                continue;
                            }
                            buffer = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _buffer) = buffer {
                                match _buffer {
                                    crate::datadogV2::model::ObservabilityPipelineBufferOptions::UnparsedObject(_buffer) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "storage_class" => {
                            storage_class =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _storage_class) = storage_class {
                                match _storage_class {
                                    crate::datadogV2::model::ObservabilityPipelineGoogleCloudStorageDestinationStorageClass::UnparsedObject(_storage_class) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineGoogleCloudStorageDestinationType::UnparsedObject(_type_) => {
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
                let storage_class =
                    storage_class.ok_or_else(|| M::Error::missing_field("storage_class"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineGoogleCloudStorageDestination {
                    acl,
                    auth,
                    bucket,
                    buffer,
                    id,
                    inputs,
                    key_prefix,
                    metadata,
                    storage_class,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineGoogleCloudStorageDestinationVisitor)
    }
}
