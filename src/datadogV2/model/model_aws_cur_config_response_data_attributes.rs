// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `AwsCurConfigResponseDataAttributes` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AwsCurConfigResponseDataAttributes {
    /// The definition of `AwsCurConfigResponseDataAttributesAccountFilters` object.
    #[serde(rename = "account_filters")]
    pub account_filters:
        Option<crate::datadogV2::model::AwsCurConfigResponseDataAttributesAccountFilters>,
    /// The `attributes` `account_id`.
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// The `attributes` `bucket_name`.
    #[serde(rename = "bucket_name")]
    pub bucket_name: Option<String>,
    /// The `attributes` `bucket_region`.
    #[serde(rename = "bucket_region")]
    pub bucket_region: Option<String>,
    /// The `attributes` `created_at`.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// The `attributes` `error_messages`.
    #[serde(
        rename = "error_messages",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub error_messages: Option<Option<Vec<String>>>,
    /// The `attributes` `months`.
    #[serde(rename = "months")]
    pub months: Option<i64>,
    /// The `attributes` `report_name`.
    #[serde(rename = "report_name")]
    pub report_name: Option<String>,
    /// The `attributes` `report_prefix`.
    #[serde(rename = "report_prefix")]
    pub report_prefix: Option<String>,
    /// The `attributes` `status`.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// The `attributes` `status_updated_at`.
    #[serde(rename = "status_updated_at")]
    pub status_updated_at: Option<String>,
    /// The `attributes` `updated_at`.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AwsCurConfigResponseDataAttributes {
    pub fn new() -> AwsCurConfigResponseDataAttributes {
        AwsCurConfigResponseDataAttributes {
            account_filters: None,
            account_id: None,
            bucket_name: None,
            bucket_region: None,
            created_at: None,
            error_messages: None,
            months: None,
            report_name: None,
            report_prefix: None,
            status: None,
            status_updated_at: None,
            updated_at: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account_filters(
        mut self,
        value: crate::datadogV2::model::AwsCurConfigResponseDataAttributesAccountFilters,
    ) -> Self {
        self.account_filters = Some(value);
        self
    }

    pub fn account_id(mut self, value: String) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn bucket_name(mut self, value: String) -> Self {
        self.bucket_name = Some(value);
        self
    }

    pub fn bucket_region(mut self, value: String) -> Self {
        self.bucket_region = Some(value);
        self
    }

    pub fn created_at(mut self, value: String) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn error_messages(mut self, value: Option<Vec<String>>) -> Self {
        self.error_messages = Some(value);
        self
    }

    pub fn months(mut self, value: i64) -> Self {
        self.months = Some(value);
        self
    }

    pub fn report_name(mut self, value: String) -> Self {
        self.report_name = Some(value);
        self
    }

    pub fn report_prefix(mut self, value: String) -> Self {
        self.report_prefix = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
        self
    }

    pub fn status_updated_at(mut self, value: String) -> Self {
        self.status_updated_at = Some(value);
        self
    }

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

impl Default for AwsCurConfigResponseDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AwsCurConfigResponseDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AwsCurConfigResponseDataAttributesVisitor;
        impl<'a> Visitor<'a> for AwsCurConfigResponseDataAttributesVisitor {
            type Value = AwsCurConfigResponseDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_filters: Option<
                    crate::datadogV2::model::AwsCurConfigResponseDataAttributesAccountFilters,
                > = None;
                let mut account_id: Option<String> = None;
                let mut bucket_name: Option<String> = None;
                let mut bucket_region: Option<String> = None;
                let mut created_at: Option<String> = None;
                let mut error_messages: Option<Option<Vec<String>>> = None;
                let mut months: Option<i64> = None;
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
                            if v.is_null() {
                                continue;
                            }
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bucket_name" => {
                            if v.is_null() {
                                continue;
                            }
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
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_messages" => {
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
                            if v.is_null() {
                                continue;
                            }
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
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
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

                let content = AwsCurConfigResponseDataAttributes {
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

        deserializer.deserialize_any(AwsCurConfigResponseDataAttributesVisitor)
    }
}
