// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Specifies the SASL mechanism for authenticating with a Kafka cluster.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineKafkaSasl {
    /// SASL mechanism used for Kafka authentication.
    #[serde(rename = "mechanism")]
    pub mechanism: Option<crate::datadogV2::model::ObservabilityPipelineKafkaSaslMechanism>,
    /// Name of the environment variable or secret that holds the SASL password.
    #[serde(rename = "password_key")]
    pub password_key: Option<String>,
    /// Name of the environment variable or secret that holds the SASL username.
    #[serde(rename = "username_key")]
    pub username_key: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ObservabilityPipelineKafkaSasl {
    pub fn new() -> ObservabilityPipelineKafkaSasl {
        ObservabilityPipelineKafkaSasl {
            mechanism: None,
            password_key: None,
            username_key: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn mechanism(mut self, value: crate::datadogV2::model::ObservabilityPipelineKafkaSaslMechanism) -> Self {
        self.mechanism = Some(value);
        self
    }
    
    pub fn password_key(mut self, value: String) -> Self {
        self.password_key = Some(value);
        self
    }
    
    pub fn username_key(mut self, value: String) -> Self {
        self.username_key = Some(value);
        self
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}


impl Default for ObservabilityPipelineKafkaSasl {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ObservabilityPipelineKafkaSasl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineKafkaSaslVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineKafkaSaslVisitor {
            type Value = ObservabilityPipelineKafkaSasl;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut mechanism: Option<crate::datadogV2::model::ObservabilityPipelineKafkaSaslMechanism> = None;
                let mut password_key: Option<String> = None;
                let mut username_key: Option<String> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "mechanism" => {
                            if v.is_null() {
                                continue;
                            }
                            mechanism = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mechanism) = mechanism {
                                match _mechanism {
                                    crate::datadogV2::model::ObservabilityPipelineKafkaSaslMechanism::UnparsedObject(_mechanism) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "password_key" => {
                            if v.is_null() {
                                continue;
                            }
                            password_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "username_key" => {
                            if v.is_null() {
                                continue;
                            }
                            username_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }

                let content = ObservabilityPipelineKafkaSasl {
                    mechanism,
                    password_key,
                    username_key,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineKafkaSaslVisitor)
    }
}
