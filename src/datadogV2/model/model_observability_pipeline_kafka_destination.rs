// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `kafka` destination sends logs to Apache Kafka topics.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineKafkaDestination {
    /// Compression codec for Kafka messages.
    #[serde(rename = "compression")]
    pub compression:
        Option<crate::datadogV2::model::ObservabilityPipelineKafkaDestinationCompression>,
    /// Encoding format for log events.
    #[serde(rename = "encoding")]
    pub encoding: crate::datadogV2::model::ObservabilityPipelineKafkaDestinationEncoding,
    /// The field name to use for Kafka message headers.
    #[serde(rename = "headers_key")]
    pub headers_key: Option<String>,
    /// The unique identifier for this component.
    #[serde(rename = "id")]
    pub id: String,
    /// A list of component IDs whose output is used as the `input` for this component.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// The field name to use as the Kafka message key.
    #[serde(rename = "key_field")]
    pub key_field: Option<String>,
    /// Optional list of advanced Kafka producer configuration options, defined as key-value pairs.
    #[serde(rename = "librdkafka_options")]
    pub librdkafka_options:
        Option<Vec<crate::datadogV2::model::ObservabilityPipelineKafkaLibrdkafkaOption>>,
    /// Maximum time in milliseconds to wait for message delivery confirmation.
    #[serde(rename = "message_timeout_ms")]
    pub message_timeout_ms: Option<i64>,
    /// Duration in seconds for the rate limit window.
    #[serde(rename = "rate_limit_duration_secs")]
    pub rate_limit_duration_secs: Option<i64>,
    /// Maximum number of messages allowed per rate limit duration.
    #[serde(rename = "rate_limit_num")]
    pub rate_limit_num: Option<i64>,
    /// Specifies the SASL mechanism for authenticating with a Kafka cluster.
    #[serde(rename = "sasl")]
    pub sasl: Option<crate::datadogV2::model::ObservabilityPipelineKafkaSasl>,
    /// Socket timeout in milliseconds for network requests.
    #[serde(rename = "socket_timeout_ms")]
    pub socket_timeout_ms: Option<i64>,
    /// Configuration for enabling TLS encryption between the pipeline component and external services.
    #[serde(rename = "tls")]
    pub tls: Option<crate::datadogV2::model::ObservabilityPipelineTls>,
    /// The Kafka topic name to publish logs to.
    #[serde(rename = "topic")]
    pub topic: String,
    /// The destination type. The value should always be `kafka`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineKafkaDestinationType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineKafkaDestination {
    pub fn new(
        encoding: crate::datadogV2::model::ObservabilityPipelineKafkaDestinationEncoding,
        id: String,
        inputs: Vec<String>,
        topic: String,
        type_: crate::datadogV2::model::ObservabilityPipelineKafkaDestinationType,
    ) -> ObservabilityPipelineKafkaDestination {
        ObservabilityPipelineKafkaDestination {
            compression: None,
            encoding,
            headers_key: None,
            id,
            inputs,
            key_field: None,
            librdkafka_options: None,
            message_timeout_ms: None,
            rate_limit_duration_secs: None,
            rate_limit_num: None,
            sasl: None,
            socket_timeout_ms: None,
            tls: None,
            topic,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn compression(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineKafkaDestinationCompression,
    ) -> Self {
        self.compression = Some(value);
        self
    }

    pub fn headers_key(mut self, value: String) -> Self {
        self.headers_key = Some(value);
        self
    }

    pub fn key_field(mut self, value: String) -> Self {
        self.key_field = Some(value);
        self
    }

    pub fn librdkafka_options(
        mut self,
        value: Vec<crate::datadogV2::model::ObservabilityPipelineKafkaLibrdkafkaOption>,
    ) -> Self {
        self.librdkafka_options = Some(value);
        self
    }

    pub fn message_timeout_ms(mut self, value: i64) -> Self {
        self.message_timeout_ms = Some(value);
        self
    }

    pub fn rate_limit_duration_secs(mut self, value: i64) -> Self {
        self.rate_limit_duration_secs = Some(value);
        self
    }

    pub fn rate_limit_num(mut self, value: i64) -> Self {
        self.rate_limit_num = Some(value);
        self
    }

    pub fn sasl(mut self, value: crate::datadogV2::model::ObservabilityPipelineKafkaSasl) -> Self {
        self.sasl = Some(value);
        self
    }

    pub fn socket_timeout_ms(mut self, value: i64) -> Self {
        self.socket_timeout_ms = Some(value);
        self
    }

    pub fn tls(mut self, value: crate::datadogV2::model::ObservabilityPipelineTls) -> Self {
        self.tls = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineKafkaDestination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineKafkaDestinationVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineKafkaDestinationVisitor {
            type Value = ObservabilityPipelineKafkaDestination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compression: Option<
                    crate::datadogV2::model::ObservabilityPipelineKafkaDestinationCompression,
                > = None;
                let mut encoding: Option<
                    crate::datadogV2::model::ObservabilityPipelineKafkaDestinationEncoding,
                > = None;
                let mut headers_key: Option<String> = None;
                let mut id: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut key_field: Option<String> = None;
                let mut librdkafka_options: Option<
                    Vec<crate::datadogV2::model::ObservabilityPipelineKafkaLibrdkafkaOption>,
                > = None;
                let mut message_timeout_ms: Option<i64> = None;
                let mut rate_limit_duration_secs: Option<i64> = None;
                let mut rate_limit_num: Option<i64> = None;
                let mut sasl: Option<crate::datadogV2::model::ObservabilityPipelineKafkaSasl> =
                    None;
                let mut socket_timeout_ms: Option<i64> = None;
                let mut tls: Option<crate::datadogV2::model::ObservabilityPipelineTls> = None;
                let mut topic: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineKafkaDestinationType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compression" => {
                            if v.is_null() {
                                continue;
                            }
                            compression =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _compression) = compression {
                                match _compression {
                                    crate::datadogV2::model::ObservabilityPipelineKafkaDestinationCompression::UnparsedObject(_compression) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "encoding" => {
                            encoding = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _encoding) = encoding {
                                match _encoding {
                                    crate::datadogV2::model::ObservabilityPipelineKafkaDestinationEncoding::UnparsedObject(_encoding) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "headers_key" => {
                            if v.is_null() {
                                continue;
                            }
                            headers_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputs" => {
                            inputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key_field" => {
                            if v.is_null() {
                                continue;
                            }
                            key_field = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "librdkafka_options" => {
                            if v.is_null() {
                                continue;
                            }
                            librdkafka_options =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message_timeout_ms" => {
                            if v.is_null() {
                                continue;
                            }
                            message_timeout_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rate_limit_duration_secs" => {
                            if v.is_null() {
                                continue;
                            }
                            rate_limit_duration_secs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rate_limit_num" => {
                            if v.is_null() {
                                continue;
                            }
                            rate_limit_num =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sasl" => {
                            if v.is_null() {
                                continue;
                            }
                            sasl = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "socket_timeout_ms" => {
                            if v.is_null() {
                                continue;
                            }
                            socket_timeout_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tls" => {
                            if v.is_null() {
                                continue;
                            }
                            tls = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "topic" => {
                            topic = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineKafkaDestinationType::UnparsedObject(_type_) => {
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
                let encoding = encoding.ok_or_else(|| M::Error::missing_field("encoding"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;
                let topic = topic.ok_or_else(|| M::Error::missing_field("topic"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineKafkaDestination {
                    compression,
                    encoding,
                    headers_key,
                    id,
                    inputs,
                    key_field,
                    librdkafka_options,
                    message_timeout_ms,
                    rate_limit_duration_secs,
                    rate_limit_num,
                    sasl,
                    socket_timeout_ms,
                    tls,
                    topic,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineKafkaDestinationVisitor)
    }
}
