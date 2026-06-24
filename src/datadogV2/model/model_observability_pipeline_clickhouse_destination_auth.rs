// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// HTTP Basic Authentication credentials for the ClickHouse destination.
/// When `strategy` is `basic`, provide `username_key` and `password_key` that reference environment variables or secrets containing the credentials.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineClickhouseDestinationAuth {
    /// Name of the environment variable or secret that contains the ClickHouse password.
    #[serde(rename = "password_key")]
    pub password_key: Option<String>,
    /// The authentication strategy for ClickHouse HTTP requests. Only `basic` is supported.
    #[serde(rename = "strategy")]
    pub strategy: crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationAuthStrategy,
    /// Name of the environment variable or secret that contains the ClickHouse username.
    #[serde(rename = "username_key")]
    pub username_key: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineClickhouseDestinationAuth {
    pub fn new(
        strategy: crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationAuthStrategy,
    ) -> ObservabilityPipelineClickhouseDestinationAuth {
        ObservabilityPipelineClickhouseDestinationAuth {
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

impl<'de> Deserialize<'de> for ObservabilityPipelineClickhouseDestinationAuth {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineClickhouseDestinationAuthVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineClickhouseDestinationAuthVisitor {
            type Value = ObservabilityPipelineClickhouseDestinationAuth;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut password_key: Option<String> = None;
                let mut strategy: Option<
                    crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationAuthStrategy,
                > = None;
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
                                    crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationAuthStrategy::UnparsedObject(_strategy) => {
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

                let content = ObservabilityPipelineClickhouseDestinationAuth {
                    password_key,
                    strategy,
                    username_key,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineClickhouseDestinationAuthVisitor)
    }
}
