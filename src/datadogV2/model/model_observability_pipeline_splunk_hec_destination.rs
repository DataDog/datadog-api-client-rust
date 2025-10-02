// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `splunk_hec` destination forwards logs to Splunk using the HTTP Event Collector (HEC).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineSplunkHecDestination {
    /// If `true`, Splunk tries to extract timestamps from incoming log events.
    /// If `false`, Splunk assigns the time the event was received.
    #[serde(rename = "auto_extract_timestamp")]
    pub auto_extract_timestamp: Option<bool>,
    /// Encoding format for log events.
    #[serde(rename = "encoding")]
    pub encoding:
        Option<crate::datadogV2::model::ObservabilityPipelineSplunkHecDestinationEncoding>,
    /// The unique identifier for this component. Used to reference this component in other parts of the pipeline (e.g., as input to downstream components).
    #[serde(rename = "id")]
    pub id: String,
    /// Optional name of the Splunk index where logs are written.
    #[serde(rename = "index")]
    pub index: Option<String>,
    /// A list of component IDs whose output is used as the `input` for this component.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// The Splunk sourcetype to assign to log events.
    #[serde(rename = "sourcetype")]
    pub sourcetype: Option<String>,
    /// The destination type. Always `splunk_hec`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineSplunkHecDestinationType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineSplunkHecDestination {
    pub fn new(
        id: String,
        inputs: Vec<String>,
        type_: crate::datadogV2::model::ObservabilityPipelineSplunkHecDestinationType,
    ) -> ObservabilityPipelineSplunkHecDestination {
        ObservabilityPipelineSplunkHecDestination {
            auto_extract_timestamp: None,
            encoding: None,
            id,
            index: None,
            inputs,
            sourcetype: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn auto_extract_timestamp(mut self, value: bool) -> Self {
        self.auto_extract_timestamp = Some(value);
        self
    }

    pub fn encoding(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineSplunkHecDestinationEncoding,
    ) -> Self {
        self.encoding = Some(value);
        self
    }

    pub fn index(mut self, value: String) -> Self {
        self.index = Some(value);
        self
    }

    pub fn sourcetype(mut self, value: String) -> Self {
        self.sourcetype = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineSplunkHecDestination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineSplunkHecDestinationVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineSplunkHecDestinationVisitor {
            type Value = ObservabilityPipelineSplunkHecDestination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auto_extract_timestamp: Option<bool> = None;
                let mut encoding: Option<
                    crate::datadogV2::model::ObservabilityPipelineSplunkHecDestinationEncoding,
                > = None;
                let mut id: Option<String> = None;
                let mut index: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut sourcetype: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineSplunkHecDestinationType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auto_extract_timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            auto_extract_timestamp =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "encoding" => {
                            if v.is_null() {
                                continue;
                            }
                            encoding = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _encoding) = encoding {
                                match _encoding {
                                    crate::datadogV2::model::ObservabilityPipelineSplunkHecDestinationEncoding::UnparsedObject(_encoding) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "index" => {
                            if v.is_null() {
                                continue;
                            }
                            index = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputs" => {
                            inputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sourcetype" => {
                            if v.is_null() {
                                continue;
                            }
                            sourcetype = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineSplunkHecDestinationType::UnparsedObject(_type_) => {
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
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineSplunkHecDestination {
                    auto_extract_timestamp,
                    encoding,
                    id,
                    index,
                    inputs,
                    sourcetype,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineSplunkHecDestinationVisitor)
    }
}
