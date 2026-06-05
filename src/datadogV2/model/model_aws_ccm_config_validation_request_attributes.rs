// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for an AWS CCM config validation request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSCcmConfigValidationRequestAttributes {
    /// Your AWS Account ID without dashes.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// Name of the S3 bucket where the Cost and Usage Report is stored.
    #[serde(rename = "bucket_name")]
    pub bucket_name: String,
    /// AWS region of the S3 bucket.
    #[serde(rename = "bucket_region")]
    pub bucket_region: String,
    /// Name of the Cost and Usage Report.
    #[serde(rename = "report_name")]
    pub report_name: String,
    /// S3 prefix where the Cost and Usage Report is stored.
    #[serde(rename = "report_prefix")]
    pub report_prefix: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSCcmConfigValidationRequestAttributes {
    pub fn new(
        account_id: String,
        bucket_name: String,
        bucket_region: String,
        report_name: String,
    ) -> AWSCcmConfigValidationRequestAttributes {
        AWSCcmConfigValidationRequestAttributes {
            account_id,
            bucket_name,
            bucket_region,
            report_name,
            report_prefix: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn report_prefix(mut self, value: String) -> Self {
        self.report_prefix = Some(value);
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

impl<'de> Deserialize<'de> for AWSCcmConfigValidationRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSCcmConfigValidationRequestAttributesVisitor;
        impl<'a> Visitor<'a> for AWSCcmConfigValidationRequestAttributesVisitor {
            type Value = AWSCcmConfigValidationRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut bucket_name: Option<String> = None;
                let mut bucket_region: Option<String> = None;
                let mut report_name: Option<String> = None;
                let mut report_prefix: Option<String> = None;
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
                        "bucket_region" => {
                            bucket_region =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "report_name" => {
                            report_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "report_prefix" => {
                            if v.is_null() {
                                continue;
                            }
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
                let bucket_region =
                    bucket_region.ok_or_else(|| M::Error::missing_field("bucket_region"))?;
                let report_name =
                    report_name.ok_or_else(|| M::Error::missing_field("report_name"))?;

                let content = AWSCcmConfigValidationRequestAttributes {
                    account_id,
                    bucket_name,
                    bucket_region,
                    report_name,
                    report_prefix,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSCcmConfigValidationRequestAttributesVisitor)
    }
}
