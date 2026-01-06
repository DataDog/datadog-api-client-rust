// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `google_pubsub` destination publishes logs to a Google Cloud Pub/Sub topic.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineGooglePubSubDestination {
    /// GCP credentials used to authenticate with Google Cloud Storage.
    #[serde(rename = "auth")]
    pub auth: Option<crate::datadogV2::model::ObservabilityPipelineGcpAuth>,
    /// Configuration for buffer settings on destination components.
    #[serde(rename = "buffer")]
    pub buffer: Option<crate::datadogV2::model::ObservabilityPipelineBufferOptions>,
    /// Encoding format for log events.
    #[serde(rename = "encoding")]
    pub encoding: crate::datadogV2::model::ObservabilityPipelineGooglePubSubDestinationEncoding,
    /// The unique identifier for this component.
    #[serde(rename = "id")]
    pub id: String,
    /// A list of component IDs whose output is used as the `input` for this component.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// The GCP project ID that owns the Pub/Sub topic.
    #[serde(rename = "project")]
    pub project: String,
    /// Configuration for enabling TLS encryption between the pipeline component and external services.
    #[serde(rename = "tls")]
    pub tls: Option<crate::datadogV2::model::ObservabilityPipelineTls>,
    /// The Pub/Sub topic name to publish logs to.
    #[serde(rename = "topic")]
    pub topic: String,
    /// The destination type. The value should always be `google_pubsub`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineGooglePubSubDestinationType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineGooglePubSubDestination {
    pub fn new(
        encoding: crate::datadogV2::model::ObservabilityPipelineGooglePubSubDestinationEncoding,
        id: String,
        inputs: Vec<String>,
        project: String,
        topic: String,
        type_: crate::datadogV2::model::ObservabilityPipelineGooglePubSubDestinationType,
    ) -> ObservabilityPipelineGooglePubSubDestination {
        ObservabilityPipelineGooglePubSubDestination {
            auth: None,
            buffer: None,
            encoding,
            id,
            inputs,
            project,
            tls: None,
            topic,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn auth(mut self, value: crate::datadogV2::model::ObservabilityPipelineGcpAuth) -> Self {
        self.auth = Some(value);
        self
    }

    pub fn buffer(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineBufferOptions,
    ) -> Self {
        self.buffer = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineGooglePubSubDestination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineGooglePubSubDestinationVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineGooglePubSubDestinationVisitor {
            type Value = ObservabilityPipelineGooglePubSubDestination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth: Option<crate::datadogV2::model::ObservabilityPipelineGcpAuth> = None;
                let mut buffer: Option<
                    crate::datadogV2::model::ObservabilityPipelineBufferOptions,
                > = None;
                let mut encoding: Option<
                    crate::datadogV2::model::ObservabilityPipelineGooglePubSubDestinationEncoding,
                > = None;
                let mut id: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut project: Option<String> = None;
                let mut tls: Option<crate::datadogV2::model::ObservabilityPipelineTls> = None;
                let mut topic: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineGooglePubSubDestinationType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth" => {
                            if v.is_null() {
                                continue;
                            }
                            auth = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "buffer" => {
                            if v.is_null() {
                                continue;
                            }
                            buffer = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _buffer) = buffer {
                                match _buffer {
                                    crate::datadogV2::model::ObservabilityPipelineBufferOptions::UnparsedObject(_buffer) => {
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
                                    crate::datadogV2::model::ObservabilityPipelineGooglePubSubDestinationEncoding::UnparsedObject(_encoding) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputs" => {
                            inputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project" => {
                            project = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV2::model::ObservabilityPipelineGooglePubSubDestinationType::UnparsedObject(_type_) => {
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
                let project = project.ok_or_else(|| M::Error::missing_field("project"))?;
                let topic = topic.ok_or_else(|| M::Error::missing_field("topic"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineGooglePubSubDestination {
                    auth,
                    buffer,
                    encoding,
                    id,
                    inputs,
                    project,
                    tls,
                    topic,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineGooglePubSubDestinationVisitor)
    }
}
