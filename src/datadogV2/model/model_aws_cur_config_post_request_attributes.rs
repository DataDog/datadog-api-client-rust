// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for AWS CUR config Post Request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AwsCURConfigPostRequestAttributes {
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
    pub bucket_region: Option<String>,
    /// The month of the report.
    #[serde(rename = "months")]
    pub months: Option<i32>,
    /// The name of the Cost and Usage Report.
    #[serde(rename = "report_name")]
    pub report_name: String,
    /// The report prefix used for the Cost and Usage Report.
    #[serde(rename = "report_prefix")]
    pub report_prefix: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AwsCURConfigPostRequestAttributes {
    pub fn new(
        account_id: String,
        bucket_name: String,
        report_name: String,
        report_prefix: String,
    ) -> AwsCURConfigPostRequestAttributes {
        AwsCURConfigPostRequestAttributes {
            account_filters: None,
            account_id,
            bucket_name,
            bucket_region: None,
            months: None,
            report_name,
            report_prefix,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account_filters(
        mut self,
        value: crate::datadogV2::model::AccountFilteringConfig,
    ) -> Self {
        self.account_filters = Some(value);
        self
    }

    pub fn bucket_region(mut self, value: String) -> Self {
        self.bucket_region = Some(value);
        self
    }

    pub fn months(mut self, value: i32) -> Self {
        self.months = Some(value);
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

impl<'de> Deserialize<'de> for AwsCURConfigPostRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AwsCURConfigPostRequestAttributesVisitor;
        impl<'a> Visitor<'a> for AwsCURConfigPostRequestAttributesVisitor {
            type Value = AwsCURConfigPostRequestAttributes;

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
                let mut months: Option<i32> = None;
                let mut report_name: Option<String> = None;
                let mut report_prefix: Option<String> = None;
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
                            if v.is_null() {
                                continue;
                            }
                            bucket_region =
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
                let report_name =
                    report_name.ok_or_else(|| M::Error::missing_field("report_name"))?;
                let report_prefix =
                    report_prefix.ok_or_else(|| M::Error::missing_field("report_prefix"))?;

                let content = AwsCURConfigPostRequestAttributes {
                    account_filters,
                    account_id,
                    bucket_name,
                    bucket_region,
                    months,
                    report_name,
                    report_prefix,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AwsCURConfigPostRequestAttributesVisitor)
    }
}
