// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A list of all Datadog-AWS logs integrations available in your Datadog organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSLogsListResponse {
    /// Your AWS Account ID without dashes.
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// List of ARNs configured in your Datadog account.
    #[serde(rename = "lambdas")]
    pub lambdas: Option<Vec<crate::datadogV1::model::AWSLogsLambda>>,
    /// Array of services IDs.
    #[serde(rename = "services")]
    pub services: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSLogsListResponse {
    pub fn new() -> AWSLogsListResponse {
        AWSLogsListResponse {
            account_id: None,
            lambdas: None,
            services: None,
            _unparsed: false,
        }
    }

    pub fn account_id(mut self, value: String) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn lambdas(mut self, value: Vec<crate::datadogV1::model::AWSLogsLambda>) -> Self {
        self.lambdas = Some(value);
        self
    }

    pub fn services(mut self, value: Vec<String>) -> Self {
        self.services = Some(value);
        self
    }
}

impl Default for AWSLogsListResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSLogsListResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSLogsListResponseVisitor;
        impl<'a> Visitor<'a> for AWSLogsListResponseVisitor {
            type Value = AWSLogsListResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut lambdas: Option<Vec<crate::datadogV1::model::AWSLogsLambda>> = None;
                let mut services: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            if v.is_null() {
                                continue;
                            }
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lambdas" => {
                            if v.is_null() {
                                continue;
                            }
                            lambdas = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "services" => {
                            if v.is_null() {
                                continue;
                            }
                            services = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AWSLogsListResponse {
                    account_id,
                    lambdas,
                    services,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSLogsListResponseVisitor)
    }
}
