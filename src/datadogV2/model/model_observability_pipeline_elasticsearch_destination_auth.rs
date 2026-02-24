// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Authentication settings for the Elasticsearch destination.
/// When `strategy` is `basic`, use `username_key` and `password_key` to reference credentials stored in environment variables or secrets.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineElasticsearchDestinationAuth {
    /// Name of the environment variable or secret that holds the Elasticsearch password (used when `strategy` is `basic`).
    #[serde(rename = "password_key")]
    pub password_key: Option<String>,
    /// The authentication strategy to use.
    #[serde(rename = "strategy")]
    pub strategy:
        crate::datadogV2::model::ObservabilityPipelineAmazonOpenSearchDestinationAuthStrategy,
    /// Name of the environment variable or secret that holds the Elasticsearch username (used when `strategy` is `basic`).
    #[serde(rename = "username_key")]
    pub username_key: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineElasticsearchDestinationAuth {
    pub fn new(
        strategy: crate::datadogV2::model::ObservabilityPipelineAmazonOpenSearchDestinationAuthStrategy,
    ) -> ObservabilityPipelineElasticsearchDestinationAuth {
        ObservabilityPipelineElasticsearchDestinationAuth {
            password_key: None,
            strategy,
            username_key: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn password_key(mut self, value: String) -> Self {
        self.password_key = Some(value);
        self
    }

    pub fn username_key(mut self, value: String) -> Self {
        self.username_key = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineElasticsearchDestinationAuth {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineElasticsearchDestinationAuthVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineElasticsearchDestinationAuthVisitor {
            type Value = ObservabilityPipelineElasticsearchDestinationAuth;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut password_key: Option<String> = None;
                let mut strategy: Option<crate::datadogV2::model::ObservabilityPipelineAmazonOpenSearchDestinationAuthStrategy> = None;
                let mut username_key: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "password_key" => {
                            if v.is_null() {
                                continue;
                            }
                            password_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "strategy" => {
                            strategy = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _strategy) = strategy {
                                match _strategy {
                                    crate::datadogV2::model::ObservabilityPipelineAmazonOpenSearchDestinationAuthStrategy::UnparsedObject(_strategy) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "username_key" => {
                            if v.is_null() {
                                continue;
                            }
                            username_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let strategy = strategy.ok_or_else(|| M::Error::missing_field("strategy"))?;

                let content = ObservabilityPipelineElasticsearchDestinationAuth {
                    password_key,
                    strategy,
                    username_key,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineElasticsearchDestinationAuthVisitor)
    }
}
