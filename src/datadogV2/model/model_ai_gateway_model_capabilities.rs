// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Model features reported by the enrichment source. Omitted properties are unknown.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayModelCapabilities {
    /// Whether attachment input is supported.
    #[serde(rename = "attachment")]
    pub attachment: Option<bool>,
    /// Whether reasoning can be interleaved with tool use.
    #[serde(rename = "interleaved_reasoning")]
    pub interleaved_reasoning: Option<bool>,
    /// Whether reasoning is supported.
    #[serde(rename = "reasoning")]
    pub reasoning: Option<bool>,
    /// Whether structured output is supported.
    #[serde(rename = "structured_output")]
    pub structured_output: Option<bool>,
    /// Whether temperature can be configured.
    #[serde(rename = "temperature")]
    pub temperature: Option<bool>,
    /// Whether tool calls are supported.
    #[serde(rename = "tool_call")]
    pub tool_call: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayModelCapabilities {
    pub fn new() -> AiGatewayModelCapabilities {
        AiGatewayModelCapabilities {
            attachment: None,
            interleaved_reasoning: None,
            reasoning: None,
            structured_output: None,
            temperature: None,
            tool_call: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn attachment(mut self, value: bool) -> Self {
        self.attachment = Some(value);
        self
    }

    pub fn interleaved_reasoning(mut self, value: bool) -> Self {
        self.interleaved_reasoning = Some(value);
        self
    }

    pub fn reasoning(mut self, value: bool) -> Self {
        self.reasoning = Some(value);
        self
    }

    pub fn structured_output(mut self, value: bool) -> Self {
        self.structured_output = Some(value);
        self
    }

    pub fn temperature(mut self, value: bool) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn tool_call(mut self, value: bool) -> Self {
        self.tool_call = Some(value);
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

impl Default for AiGatewayModelCapabilities {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AiGatewayModelCapabilities {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayModelCapabilitiesVisitor;
        impl<'a> Visitor<'a> for AiGatewayModelCapabilitiesVisitor {
            type Value = AiGatewayModelCapabilities;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attachment: Option<bool> = None;
                let mut interleaved_reasoning: Option<bool> = None;
                let mut reasoning: Option<bool> = None;
                let mut structured_output: Option<bool> = None;
                let mut temperature: Option<bool> = None;
                let mut tool_call: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attachment" => {
                            if v.is_null() {
                                continue;
                            }
                            attachment = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "interleaved_reasoning" => {
                            if v.is_null() {
                                continue;
                            }
                            interleaved_reasoning =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reasoning" => {
                            if v.is_null() {
                                continue;
                            }
                            reasoning = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "structured_output" => {
                            if v.is_null() {
                                continue;
                            }
                            structured_output =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "temperature" => {
                            if v.is_null() {
                                continue;
                            }
                            temperature =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tool_call" => {
                            if v.is_null() {
                                continue;
                            }
                            tool_call = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AiGatewayModelCapabilities {
                    attachment,
                    interleaved_reasoning,
                    reasoning,
                    structured_output,
                    temperature,
                    tool_call,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayModelCapabilitiesVisitor)
    }
}
