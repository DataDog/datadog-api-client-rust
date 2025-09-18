// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for Google Cloud Usage Cost config post request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GCPUsageCostConfigPostRequestAttributes {
    /// The Google Cloud account ID.
    #[serde(rename = "billing_account_id")]
    pub billing_account_id: String,
    /// The Google Cloud bucket name used to store the Usage Cost export.
    #[serde(rename = "bucket_name")]
    pub bucket_name: String,
    /// The export dataset name used for the Google Cloud Usage Cost report.
    #[serde(rename = "export_dataset_name")]
    pub export_dataset_name: String,
    /// The export prefix used for the Google Cloud Usage Cost report.
    #[serde(rename = "export_prefix")]
    pub export_prefix: Option<String>,
    /// The name of the Google Cloud Usage Cost report.
    #[serde(rename = "export_project_name")]
    pub export_project_name: String,
    /// The unique Google Cloud service account email.
    #[serde(rename = "service_account")]
    pub service_account: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GCPUsageCostConfigPostRequestAttributes {
    pub fn new(
        billing_account_id: String,
        bucket_name: String,
        export_dataset_name: String,
        export_project_name: String,
        service_account: String,
    ) -> GCPUsageCostConfigPostRequestAttributes {
        GCPUsageCostConfigPostRequestAttributes {
            billing_account_id,
            bucket_name,
            export_dataset_name,
            export_prefix: None,
            export_project_name,
            service_account,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn export_prefix(mut self, value: String) -> Self {
        self.export_prefix = Some(value);
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

impl<'de> Deserialize<'de> for GCPUsageCostConfigPostRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GCPUsageCostConfigPostRequestAttributesVisitor;
        impl<'a> Visitor<'a> for GCPUsageCostConfigPostRequestAttributesVisitor {
            type Value = GCPUsageCostConfigPostRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut billing_account_id: Option<String> = None;
                let mut bucket_name: Option<String> = None;
                let mut export_dataset_name: Option<String> = None;
                let mut export_prefix: Option<String> = None;
                let mut export_project_name: Option<String> = None;
                let mut service_account: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "billing_account_id" => {
                            billing_account_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bucket_name" => {
                            bucket_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "export_dataset_name" => {
                            export_dataset_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "export_prefix" => {
                            if v.is_null() {
                                continue;
                            }
                            export_prefix =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "export_project_name" => {
                            export_project_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service_account" => {
                            service_account =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let billing_account_id = billing_account_id
                    .ok_or_else(|| M::Error::missing_field("billing_account_id"))?;
                let bucket_name =
                    bucket_name.ok_or_else(|| M::Error::missing_field("bucket_name"))?;
                let export_dataset_name = export_dataset_name
                    .ok_or_else(|| M::Error::missing_field("export_dataset_name"))?;
                let export_project_name = export_project_name
                    .ok_or_else(|| M::Error::missing_field("export_project_name"))?;
                let service_account =
                    service_account.ok_or_else(|| M::Error::missing_field("service_account"))?;

                let content = GCPUsageCostConfigPostRequestAttributes {
                    billing_account_id,
                    bucket_name,
                    export_dataset_name,
                    export_prefix,
                    export_project_name,
                    service_account,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GCPUsageCostConfigPostRequestAttributesVisitor)
    }
}
