// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Provider-specific configuration. Include the object that matches `data.id` (`aws`, `gcp`, or `azure`).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpsertCloudInventorySyncConfigRequestAttributes {
    /// AWS settings for the customer bucket that stores inventory reports.
    #[serde(rename = "aws")]
    pub aws: Option<crate::datadogV2::model::CloudInventorySyncConfigAWSRequestAttributes>,
    /// Azure settings for the storage account and container with inventory data.
    #[serde(rename = "azure")]
    pub azure: Option<crate::datadogV2::model::CloudInventorySyncConfigAzureRequestAttributes>,
    /// GCP settings for buckets involved in inventory reporting.
    #[serde(rename = "gcp")]
    pub gcp: Option<crate::datadogV2::model::CloudInventorySyncConfigGCPRequestAttributes>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UpsertCloudInventorySyncConfigRequestAttributes {
    pub fn new() -> UpsertCloudInventorySyncConfigRequestAttributes {
        UpsertCloudInventorySyncConfigRequestAttributes {
            aws: None,
            azure: None,
            gcp: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn aws(
        mut self,
        value: crate::datadogV2::model::CloudInventorySyncConfigAWSRequestAttributes,
    ) -> Self {
        self.aws = Some(value);
        self
    }

    pub fn azure(
        mut self,
        value: crate::datadogV2::model::CloudInventorySyncConfigAzureRequestAttributes,
    ) -> Self {
        self.azure = Some(value);
        self
    }

    pub fn gcp(
        mut self,
        value: crate::datadogV2::model::CloudInventorySyncConfigGCPRequestAttributes,
    ) -> Self {
        self.gcp = Some(value);
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

impl Default for UpsertCloudInventorySyncConfigRequestAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UpsertCloudInventorySyncConfigRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpsertCloudInventorySyncConfigRequestAttributesVisitor;
        impl<'a> Visitor<'a> for UpsertCloudInventorySyncConfigRequestAttributesVisitor {
            type Value = UpsertCloudInventorySyncConfigRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aws: Option<
                    crate::datadogV2::model::CloudInventorySyncConfigAWSRequestAttributes,
                > = None;
                let mut azure: Option<
                    crate::datadogV2::model::CloudInventorySyncConfigAzureRequestAttributes,
                > = None;
                let mut gcp: Option<
                    crate::datadogV2::model::CloudInventorySyncConfigGCPRequestAttributes,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aws" => {
                            if v.is_null() {
                                continue;
                            }
                            aws = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "azure" => {
                            if v.is_null() {
                                continue;
                            }
                            azure = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gcp" => {
                            if v.is_null() {
                                continue;
                            }
                            gcp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = UpsertCloudInventorySyncConfigRequestAttributes {
                    aws,
                    azure,
                    gcp,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UpsertCloudInventorySyncConfigRequestAttributesVisitor)
    }
}
