// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Terraform backend sync configuration and bucket statuses.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TerraformBackendAttributes {
    /// AWS account ID that owns the S3 buckets.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// Backend type to synchronize.
    #[serde(rename = "backend_type")]
    pub backend_type: crate::datadogV2::model::TerraformBackendKind,
    /// Source buckets and their synchronization statuses.
    #[serde(rename = "buckets")]
    pub buckets: Vec<crate::datadogV2::model::TerraformBackendBucket>,
    /// Datadog organization ID.
    #[serde(rename = "org_id")]
    pub org_id: String,
    /// AWS region containing the S3 buckets.
    #[serde(rename = "region")]
    pub region: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TerraformBackendAttributes {
    pub fn new(
        account_id: String,
        backend_type: crate::datadogV2::model::TerraformBackendKind,
        buckets: Vec<crate::datadogV2::model::TerraformBackendBucket>,
        org_id: String,
        region: String,
    ) -> TerraformBackendAttributes {
        TerraformBackendAttributes {
            account_id,
            backend_type,
            buckets,
            org_id,
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

impl<'de> Deserialize<'de> for TerraformBackendAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TerraformBackendAttributesVisitor;
        impl<'a> Visitor<'a> for TerraformBackendAttributesVisitor {
            type Value = TerraformBackendAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut backend_type: Option<crate::datadogV2::model::TerraformBackendKind> = None;
                let mut buckets: Option<Vec<crate::datadogV2::model::TerraformBackendBucket>> =
                    None;
                let mut org_id: Option<String> = None;
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
                        "buckets" => {
                            buckets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let buckets = buckets.ok_or_else(|| M::Error::missing_field("buckets"))?;
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;
                let region = region.ok_or_else(|| M::Error::missing_field("region"))?;

                let content = TerraformBackendAttributes {
                    account_id,
                    backend_type,
                    buckets,
                    org_id,
                    region,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TerraformBackendAttributesVisitor)
    }
}
