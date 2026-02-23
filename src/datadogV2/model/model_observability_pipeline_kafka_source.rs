// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `kafka` source ingests data from Apache Kafka topics.
///
/// **Supported pipeline types:** logs
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineKafkaSource {
    /// Name of the environment variable or secret that holds the Kafka bootstrap servers list.
    #[serde(rename = "bootstrap_servers_key")]
    pub bootstrap_servers_key: Option<String>,
    /// Consumer group ID used by the Kafka client.
    #[serde(rename = "group_id")]
    pub group_id: String,
    /// The unique identifier for this component. Used in other parts of the pipeline to reference this component (for example, as the `input` to downstream components).
    #[serde(rename = "id")]
    pub id: String,
    /// Optional list of advanced Kafka client configuration options, defined as key-value pairs.
    #[serde(rename = "librdkafka_options")]
    pub librdkafka_options:
        Option<Vec<crate::datadogV2::model::ObservabilityPipelineKafkaLibrdkafkaOption>>,
    /// Specifies the SASL mechanism for authenticating with a Kafka cluster.
    #[serde(rename = "sasl")]
    pub sasl: Option<crate::datadogV2::model::ObservabilityPipelineKafkaSasl>,
    /// Configuration for enabling TLS encryption between the pipeline component and external services.
    #[serde(rename = "tls")]
    pub tls: Option<crate::datadogV2::model::ObservabilityPipelineTls>,
    /// A list of Kafka topic names to subscribe to. The source ingests messages from each topic specified.
    #[serde(rename = "topics")]
    pub topics: Vec<String>,
    /// The source type. The value should always be `kafka`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineKafkaSourceType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineKafkaSource {
    pub fn new(
        group_id: String,
        id: String,
        topics: Vec<String>,
        type_: crate::datadogV2::model::ObservabilityPipelineKafkaSourceType,
    ) -> ObservabilityPipelineKafkaSource {
        ObservabilityPipelineKafkaSource {
            bootstrap_servers_key: None,
            group_id,
            id,
            librdkafka_options: None,
            sasl: None,
            tls: None,
            topics,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn bootstrap_servers_key(mut self, value: String) -> Self {
        self.bootstrap_servers_key = Some(value);
        self
    }

    pub fn librdkafka_options(
        mut self,
        value: Vec<crate::datadogV2::model::ObservabilityPipelineKafkaLibrdkafkaOption>,
    ) -> Self {
        self.librdkafka_options = Some(value);
        self
    }

    pub fn sasl(mut self, value: crate::datadogV2::model::ObservabilityPipelineKafkaSasl) -> Self {
        self.sasl = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineKafkaSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineKafkaSourceVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineKafkaSourceVisitor {
            type Value = ObservabilityPipelineKafkaSource;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bootstrap_servers_key: Option<String> = None;
                let mut group_id: Option<String> = None;
                let mut id: Option<String> = None;
                let mut librdkafka_options: Option<
                    Vec<crate::datadogV2::model::ObservabilityPipelineKafkaLibrdkafkaOption>,
                > = None;
                let mut sasl: Option<crate::datadogV2::model::ObservabilityPipelineKafkaSasl> =
                    None;
                let mut tls: Option<crate::datadogV2::model::ObservabilityPipelineTls> = None;
                let mut topics: Option<Vec<String>> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineKafkaSourceType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bootstrap_servers_key" => {
                            if v.is_null() {
                                continue;
                            }
                            bootstrap_servers_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_id" => {
                            group_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "librdkafka_options" => {
                            if v.is_null() {
                                continue;
                            }
                            librdkafka_options =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sasl" => {
                            if v.is_null() {
                                continue;
                            }
                            sasl = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tls" => {
                            if v.is_null() {
                                continue;
                            }
                            tls = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "topics" => {
                            topics = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineKafkaSourceType::UnparsedObject(_type_) => {
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
                let group_id = group_id.ok_or_else(|| M::Error::missing_field("group_id"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let topics = topics.ok_or_else(|| M::Error::missing_field("topics"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineKafkaSource {
                    bootstrap_servers_key,
                    group_id,
                    id,
                    librdkafka_options,
                    sasl,
                    tls,
                    topics,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineKafkaSourceVisitor)
    }
}
