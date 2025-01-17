// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for a GCP Usage Cost config.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GCPUsageCostConfigAttributes {
    /// The GCP account ID.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The GCP bucket name used to store the Usage Cost export.
    #[serde(rename = "bucket_name")]
    pub bucket_name: String,
    /// The timestamp when the GCP Usage Cost config was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// The export dataset name used for the GCP Usage Cost Report.
    #[serde(rename = "dataset")]
    pub dataset: String,
    /// The error messages for the GCP Usage Cost config.
    #[serde(rename = "error_messages")]
    pub error_messages: Option<Vec<String>>,
    /// The export prefix used for the GCP Usage Cost Report.
    #[serde(rename = "export_prefix")]
    pub export_prefix: String,
    /// The name of the GCP Usage Cost Report.
    #[serde(rename = "export_project_name")]
    pub export_project_name: String,
    /// The number of months the report has been backfilled.
    #[deprecated]
    #[serde(rename = "months")]
    pub months: Option<i32>,
    /// The `project_id` of the GCP Usage Cost report.
    #[serde(rename = "project_id")]
    pub project_id: Option<String>,
    /// The unique GCP service account email.
    #[serde(rename = "service_account")]
    pub service_account: String,
    /// The status of the GCP Usage Cost config.
    #[serde(rename = "status")]
    pub status: String,
    /// The timestamp when the GCP Usage Cost config status was updated.
    #[serde(rename = "status_updated_at")]
    pub status_updated_at: Option<String>,
    /// The timestamp when the GCP Usage Cost config status was updated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GCPUsageCostConfigAttributes {
    pub fn new(
        account_id: String,
        bucket_name: String,
        dataset: String,
        export_prefix: String,
        export_project_name: String,
        service_account: String,
        status: String,
    ) -> GCPUsageCostConfigAttributes {
        #[allow(deprecated)]
        GCPUsageCostConfigAttributes {
            account_id,
            bucket_name,
            created_at: None,
            dataset,
            error_messages: None,
            export_prefix,
            export_project_name,
            months: None,
            project_id: None,
            service_account,
            status,
            status_updated_at: None,
            updated_at: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn created_at(mut self, value: String) -> Self {
        self.created_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn error_messages(mut self, value: Vec<String>) -> Self {
        self.error_messages = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn months(mut self, value: i32) -> Self {
        self.months = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn project_id(mut self, value: String) -> Self {
        self.project_id = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn status_updated_at(mut self, value: String) -> Self {
        self.status_updated_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn updated_at(mut self, value: String) -> Self {
        self.updated_at = Some(value);
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

impl<'de> Deserialize<'de> for GCPUsageCostConfigAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GCPUsageCostConfigAttributesVisitor;
        impl<'a> Visitor<'a> for GCPUsageCostConfigAttributesVisitor {
            type Value = GCPUsageCostConfigAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut bucket_name: Option<String> = None;
                let mut created_at: Option<String> = None;
                let mut dataset: Option<String> = None;
                let mut error_messages: Option<Vec<String>> = None;
                let mut export_prefix: Option<String> = None;
                let mut export_project_name: Option<String> = None;
                let mut months: Option<i32> = None;
                let mut project_id: Option<String> = None;
                let mut service_account: Option<String> = None;
                let mut status: Option<String> = None;
                let mut status_updated_at: Option<String> = None;
                let mut updated_at: Option<String> = None;
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
                        "bucket_name" => {
                            bucket_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dataset" => {
                            dataset = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_messages" => {
                            if v.is_null() {
                                continue;
                            }
                            error_messages =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "export_prefix" => {
                            export_prefix =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "export_project_name" => {
                            export_project_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "months" => {
                            if v.is_null() {
                                continue;
                            }
                            months = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project_id" => {
                            if v.is_null() {
                                continue;
                            }
                            project_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service_account" => {
                            service_account =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status_updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            status_updated_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let account_id = account_id.ok_or_else(|| M::Error::missing_field("account_id"))?;
                let bucket_name =
                    bucket_name.ok_or_else(|| M::Error::missing_field("bucket_name"))?;
                let dataset = dataset.ok_or_else(|| M::Error::missing_field("dataset"))?;
                let export_prefix =
                    export_prefix.ok_or_else(|| M::Error::missing_field("export_prefix"))?;
                let export_project_name = export_project_name
                    .ok_or_else(|| M::Error::missing_field("export_project_name"))?;
                let service_account =
                    service_account.ok_or_else(|| M::Error::missing_field("service_account"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                #[allow(deprecated)]
                let content = GCPUsageCostConfigAttributes {
                    account_id,
                    bucket_name,
                    created_at,
                    dataset,
                    error_messages,
                    export_prefix,
                    export_project_name,
                    months,
                    project_id,
                    service_account,
                    status,
                    status_updated_at,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GCPUsageCostConfigAttributesVisitor)
    }
}
