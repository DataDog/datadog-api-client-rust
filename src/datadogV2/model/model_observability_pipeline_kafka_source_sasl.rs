// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Specifies the SASL mechanism for authenticating with a Kafka cluster.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineKafkaSourceSasl {
    /// SASL mechanism used for Kafka authentication.
    #[serde(rename = "mechanism")]
    pub mechanism:
        Option<crate::datadogV2::model::ObservabilityPipelinePipelineKafkaSourceSaslMechanism>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineKafkaSourceSasl {
    pub fn new() -> ObservabilityPipelineKafkaSourceSasl {
        ObservabilityPipelineKafkaSourceSasl {
            mechanism: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn mechanism(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelinePipelineKafkaSourceSaslMechanism,
    ) -> Self {
        self.mechanism = Some(value);
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

impl Default for ObservabilityPipelineKafkaSourceSasl {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ObservabilityPipelineKafkaSourceSasl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineKafkaSourceSaslVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineKafkaSourceSaslVisitor {
            type Value = ObservabilityPipelineKafkaSourceSasl;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut mechanism: Option<
                    crate::datadogV2::model::ObservabilityPipelinePipelineKafkaSourceSaslMechanism,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                                    crate::datadogV2::model::ObservabilityPipelinePipelineKafkaSourceSaslMechanism::UnparsedObject(_mechanism) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ObservabilityPipelineKafkaSourceSasl {
                    mechanism,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineKafkaSourceSaslVisitor)
    }
}
