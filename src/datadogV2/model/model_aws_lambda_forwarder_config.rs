// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Log Autosubscription configuration for Datadog Forwarder Lambda functions. Automatically set up triggers for existing
/// and new logs for some services, ensuring no logs from new resources are missed and saving time spent on manual configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSLambdaForwarderConfig {
    /// List of Datadog Lambda Log Forwarder ARNs in your AWS account. Defaults to `[]`.
    #[serde(rename = "lambdas")]
    pub lambdas: Option<Vec<String>>,
    /// List of service IDs set to enable automatic log collection. Discover the list of available services with the
    /// [Get list of AWS log ready services](<https://docs.datadoghq.com/api/latest/aws-logs-integration/#get-list-of-aws-log-ready-services>) endpoint.
    #[serde(rename = "sources")]
    pub sources: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSLambdaForwarderConfig {
    pub fn new() -> AWSLambdaForwarderConfig {
        AWSLambdaForwarderConfig {
            lambdas: None,
            sources: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn lambdas(mut self, value: Vec<String>) -> Self {
        self.lambdas = Some(value);
        self
    }

    pub fn sources(mut self, value: Vec<String>) -> Self {
        self.sources = Some(value);
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

impl Default for AWSLambdaForwarderConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSLambdaForwarderConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSLambdaForwarderConfigVisitor;
        impl<'a> Visitor<'a> for AWSLambdaForwarderConfigVisitor {
            type Value = AWSLambdaForwarderConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut lambdas: Option<Vec<String>> = None;
                let mut sources: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "lambdas" => {
                            if v.is_null() {
                                continue;
                            }
                            lambdas = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sources" => {
                            if v.is_null() {
                                continue;
                            }
                            sources = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AWSLambdaForwarderConfig {
                    lambdas,
                    sources,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSLambdaForwarderConfigVisitor)
    }
}
