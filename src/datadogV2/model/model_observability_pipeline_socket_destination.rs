// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `socket` destination sends logs over TCP or UDP to a remote server.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineSocketDestination {
    /// Encoding format for log events.
    #[serde(rename = "encoding")]
    pub encoding: crate::datadogV2::model::ObservabilityPipelineSocketDestinationEncoding,
    /// Framing method configuration.
    #[serde(rename = "framing")]
    pub framing: crate::datadogV2::model::ObservabilityPipelineSocketDestinationFraming,
    /// The unique identifier for this component.
    #[serde(rename = "id")]
    pub id: String,
    /// A list of component IDs whose output is used as the `input` for this component.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// Protocol used to send logs.
    #[serde(rename = "mode")]
    pub mode: crate::datadogV2::model::ObservabilityPipelineSocketDestinationMode,
    /// Configuration for enabling TLS encryption between the pipeline component and external services.
    #[serde(rename = "tls")]
    pub tls: Option<crate::datadogV2::model::ObservabilityPipelineTls>,
    /// The destination type. The value should always be `socket`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineSocketDestinationType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineSocketDestination {
    pub fn new(
        encoding: crate::datadogV2::model::ObservabilityPipelineSocketDestinationEncoding,
        framing: crate::datadogV2::model::ObservabilityPipelineSocketDestinationFraming,
        id: String,
        inputs: Vec<String>,
        mode: crate::datadogV2::model::ObservabilityPipelineSocketDestinationMode,
        type_: crate::datadogV2::model::ObservabilityPipelineSocketDestinationType,
    ) -> ObservabilityPipelineSocketDestination {
        ObservabilityPipelineSocketDestination {
            encoding,
            framing,
            id,
            inputs,
            mode,
            tls: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
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

impl<'de> Deserialize<'de> for ObservabilityPipelineSocketDestination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineSocketDestinationVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineSocketDestinationVisitor {
            type Value = ObservabilityPipelineSocketDestination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut encoding: Option<
                    crate::datadogV2::model::ObservabilityPipelineSocketDestinationEncoding,
                > = None;
                let mut framing: Option<
                    crate::datadogV2::model::ObservabilityPipelineSocketDestinationFraming,
                > = None;
                let mut id: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut mode: Option<
                    crate::datadogV2::model::ObservabilityPipelineSocketDestinationMode,
                > = None;
                let mut tls: Option<crate::datadogV2::model::ObservabilityPipelineTls> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineSocketDestinationType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "encoding" => {
                            encoding = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _encoding) = encoding {
                                match _encoding {
                                    crate::datadogV2::model::ObservabilityPipelineSocketDestinationEncoding::UnparsedObject(_encoding) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "framing" => {
                            framing = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _framing) = framing {
                                match _framing {
                                    crate::datadogV2::model::ObservabilityPipelineSocketDestinationFraming::UnparsedObject(_framing) => {
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
                        "mode" => {
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mode) = mode {
                                match _mode {
                                    crate::datadogV2::model::ObservabilityPipelineSocketDestinationMode::UnparsedObject(_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "tls" => {
                            if v.is_null() {
                                continue;
                            }
                            tls = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineSocketDestinationType::UnparsedObject(_type_) => {
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
                let framing = framing.ok_or_else(|| M::Error::missing_field("framing"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;
                let mode = mode.ok_or_else(|| M::Error::missing_field("mode"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineSocketDestination {
                    encoding,
                    framing,
                    id,
                    inputs,
                    mode,
                    tls,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineSocketDestinationVisitor)
    }
}
