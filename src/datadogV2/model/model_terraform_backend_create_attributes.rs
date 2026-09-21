// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings for a new Terraform backend sync configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TerraformBackendCreateAttributes {
    /// AWS account ID that owns the S3 buckets.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// Backend type to synchronize.
    #[serde(rename = "backend_type")]
    pub backend_type: crate::datadogV2::model::TerraformBackendKind,
    /// Complete set of S3 bucket names to synchronize. Names must be nonempty and unique.
    #[serde(rename = "bucket_names")]
    pub bucket_names: Vec<String>,
    /// AWS region containing the S3 buckets.
    #[serde(rename = "region")]
    pub region: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TerraformBackendCreateAttributes {
    pub fn new(
        account_id: String,
        backend_type: crate::datadogV2::model::TerraformBackendKind,
        bucket_names: Vec<String>,
        region: String,
    ) -> TerraformBackendCreateAttributes {
        TerraformBackendCreateAttributes {
            account_id,
            backend_type,
            bucket_names,
            region,
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

impl<'de> Deserialize<'de> for TerraformBackendCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TerraformBackendCreateAttributesVisitor;
        impl<'a> Visitor<'a> for TerraformBackendCreateAttributesVisitor {
            type Value = TerraformBackendCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut backend_type: Option<crate::datadogV2::model::TerraformBackendKind> = None;
                let mut bucket_names: Option<Vec<String>> = None;
                let mut region: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "backend_type" => {
                            backend_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _backend_type) = backend_type {
                                match _backend_type {
                                    crate::datadogV2::model::TerraformBackendKind::UnparsedObject(_backend_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "bucket_names" => {
                            bucket_names =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "region" => {
                            region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let account_id = account_id.ok_or_else(|| M::Error::missing_field("account_id"))?;
                let backend_type =
                    backend_type.ok_or_else(|| M::Error::missing_field("backend_type"))?;
                let bucket_names =
                    bucket_names.ok_or_else(|| M::Error::missing_field("bucket_names"))?;
                let region = region.ok_or_else(|| M::Error::missing_field("region"))?;

                let content = TerraformBackendCreateAttributes {
                    account_id,
                    backend_type,
                    bucket_names,
                    region,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TerraformBackendCreateAttributesVisitor)
    }
}
