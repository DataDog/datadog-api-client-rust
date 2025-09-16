// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for An AWS CUR config.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AwsCURConfigAttributes {
    /// The account filtering configuration object.
    #[serde(rename = "account_filters")]
    pub account_filters: Option<crate::datadogV2::model::AccountFilteringConfig>,
    /// The AWS account ID.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The AWS bucket name used to store the Cost and Usage Report.
    #[serde(rename = "bucket_name")]
    pub bucket_name: String,
    /// The region the bucket is located in.
    #[serde(rename = "bucket_region")]
    pub bucket_region: String,
    /// The timestamp when the AWS CUR config was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// The error messages for the AWS CUR config.
    #[serde(rename = "error_messages")]
    pub error_messages: Option<Vec<String>>,
    /// The number of months the report has been backfilled.
    #[deprecated]
    #[serde(rename = "months")]
    pub months: Option<i32>,
    /// The name of the Cost and Usage Report.
    #[serde(rename = "report_name")]
    pub report_name: String,
    /// The report prefix used for the Cost and Usage Report.
    #[serde(rename = "report_prefix")]
    pub report_prefix: String,
    /// The status of the AWS CUR.
    #[serde(rename = "status")]
    pub status: String,
    /// The timestamp when the AWS CUR config status was updated.
    #[serde(rename = "status_updated_at")]
    pub status_updated_at: Option<String>,
    /// The timestamp when the AWS CUR config status was updated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AwsCURConfigAttributes {
    pub fn new(
        account_id: String,
        bucket_name: String,
        bucket_region: String,
        report_name: String,
        report_prefix: String,
        status: String,
    ) -> AwsCURConfigAttributes {
        #[allow(deprecated)]
        AwsCURConfigAttributes {
            account_filters: None,
            account_id,
            bucket_name,
            bucket_region,
            created_at: None,
            error_messages: None,
            months: None,
            report_name,
            report_prefix,
            status,
            status_updated_at: None,
            updated_at: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn account_filters(
        mut self,
        value: crate::datadogV2::model::AccountFilteringConfig,
    ) -> Self {
        self.account_filters = Some(value);
        self
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

impl<'de> Deserialize<'de> for AwsCURConfigAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AwsCURConfigAttributesVisitor;
        impl<'a> Visitor<'a> for AwsCURConfigAttributesVisitor {
            type Value = AwsCURConfigAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_filters: Option<crate::datadogV2::model::AccountFilteringConfig> =
                    None;
                let mut account_id: Option<String> = None;
                let mut bucket_name: Option<String> = None;
                let mut bucket_region: Option<String> = None;
                let mut created_at: Option<String> = None;
                let mut error_messages: Option<Vec<String>> = None;
                let mut months: Option<i32> = None;
                let mut report_name: Option<String> = None;
                let mut report_prefix: Option<String> = None;
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
                        "account_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            account_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "account_id" => {
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bucket_name" => {
                            bucket_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bucket_region" => {
                            bucket_region =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_messages" => {
                            if v.is_null() {
                                continue;
                            }
                            error_messages =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "months" => {
                            if v.is_null() {
                                continue;
                            }
                            months = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "report_name" => {
                            report_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "report_prefix" => {
                            report_prefix =
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
                let bucket_region =
                    bucket_region.ok_or_else(|| M::Error::missing_field("bucket_region"))?;
                let report_name =
                    report_name.ok_or_else(|| M::Error::missing_field("report_name"))?;
                let report_prefix =
                    report_prefix.ok_or_else(|| M::Error::missing_field("report_prefix"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                #[allow(deprecated)]
                let content = AwsCURConfigAttributes {
                    account_filters,
                    account_id,
                    bucket_name,
                    bucket_region,
                    created_at,
                    error_messages,
                    months,
                    report_name,
                    report_prefix,
                    status,
                    status_updated_at,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AwsCURConfigAttributesVisitor)
    }
}
