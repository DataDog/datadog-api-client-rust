// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecretRuleDataAttributesMatchValidation {
    #[serde(rename = "endpoint")]
    pub endpoint: Option<String>,
    #[serde(rename = "hosts")]
    pub hosts: Option<Vec<String>>,
    #[serde(rename = "http_method")]
    pub http_method: Option<String>,
    #[serde(rename = "invalid_http_status_code")]
    pub invalid_http_status_code: Option<Vec<crate::datadogV2::model::SecretRuleDataAttributesMatchValidationInvalidHttpStatusCodeItems>>,
    #[serde(rename = "request_headers")]
    pub request_headers: Option<std::collections::BTreeMap<String, String>>,
    #[serde(rename = "timeout_seconds")]
    pub timeout_seconds: Option<i64>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "valid_http_status_code")]
    pub valid_http_status_code: Option<Vec<crate::datadogV2::model::SecretRuleDataAttributesMatchValidationValidHttpStatusCodeItems>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl SecretRuleDataAttributesMatchValidation {
    pub fn new() -> SecretRuleDataAttributesMatchValidation {
        SecretRuleDataAttributesMatchValidation {
            endpoint: None,
            hosts: None,
            http_method: None,
            invalid_http_status_code: None,
            request_headers: None,
            timeout_seconds: None,
            type_: None,
            valid_http_status_code: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn endpoint(mut self, value: String) -> Self {
        self.endpoint = Some(value);
        self
    }

    pub fn hosts(mut self, value: Vec<String>) -> Self {
        self.hosts = Some(value);
        self
    }

    pub fn http_method(mut self, value: String) -> Self {
        self.http_method = Some(value);
        self
    }

    pub fn invalid_http_status_code(
        mut self,
        value: Vec<crate::datadogV2::model::SecretRuleDataAttributesMatchValidationInvalidHttpStatusCodeItems>,
    ) -> Self {
        self.invalid_http_status_code = Some(value);
        self
    }

    pub fn request_headers(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.request_headers = Some(value);
        self
    }

    pub fn timeout_seconds(mut self, value: i64) -> Self {
        self.timeout_seconds = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn valid_http_status_code(
        mut self,
        value: Vec<crate::datadogV2::model::SecretRuleDataAttributesMatchValidationValidHttpStatusCodeItems>,
    ) -> Self {
        self.valid_http_status_code = Some(value);
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

impl Default for SecretRuleDataAttributesMatchValidation {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecretRuleDataAttributesMatchValidation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecretRuleDataAttributesMatchValidationVisitor;
        impl<'a> Visitor<'a> for SecretRuleDataAttributesMatchValidationVisitor {
            type Value = SecretRuleDataAttributesMatchValidation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut endpoint: Option<String> = None;
                let mut hosts: Option<Vec<String>> = None;
                let mut http_method: Option<String> = None;
                let mut invalid_http_status_code: Option<Vec<crate::datadogV2::model::SecretRuleDataAttributesMatchValidationInvalidHttpStatusCodeItems>> = None;
                let mut request_headers: Option<std::collections::BTreeMap<String, String>> = None;
                let mut timeout_seconds: Option<i64> = None;
                let mut type_: Option<String> = None;
                let mut valid_http_status_code: Option<Vec<crate::datadogV2::model::SecretRuleDataAttributesMatchValidationValidHttpStatusCodeItems>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "endpoint" => {
                            if v.is_null() {
                                continue;
                            }
                            endpoint = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hosts" => {
                            if v.is_null() {
                                continue;
                            }
                            hosts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "http_method" => {
                            if v.is_null() {
                                continue;
                            }
                            http_method =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "invalid_http_status_code" => {
                            if v.is_null() {
                                continue;
                            }
                            invalid_http_status_code =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "request_headers" => {
                            if v.is_null() {
                                continue;
                            }
                            request_headers =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeout_seconds" => {
                            if v.is_null() {
                                continue;
                            }
                            timeout_seconds =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "valid_http_status_code" => {
                            if v.is_null() {
                                continue;
                            }
                            valid_http_status_code =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SecretRuleDataAttributesMatchValidation {
                    endpoint,
                    hosts,
                    http_method,
                    invalid_http_status_code,
                    request_headers,
                    timeout_seconds,
                    type_,
                    valid_http_status_code,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecretRuleDataAttributesMatchValidationVisitor)
    }
}
