// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Token limits reported by the enrichment source.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayModelLimits {
    /// Total model context window in tokens.
    #[serde(rename = "context_window_tokens")]
    pub context_window_tokens: Option<i64>,
    /// Maximum input tokens when reported separately.
    #[serde(rename = "max_input_tokens")]
    pub max_input_tokens: Option<i64>,
    /// Maximum output tokens.
    #[serde(rename = "max_output_tokens")]
    pub max_output_tokens: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayModelLimits {
    pub fn new() -> AiGatewayModelLimits {
        AiGatewayModelLimits {
            context_window_tokens: None,
            max_input_tokens: None,
            max_output_tokens: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn context_window_tokens(mut self, value: i64) -> Self {
        self.context_window_tokens = Some(value);
        self
    }

    pub fn max_input_tokens(mut self, value: i64) -> Self {
        self.max_input_tokens = Some(value);
        self
    }

    pub fn max_output_tokens(mut self, value: i64) -> Self {
        self.max_output_tokens = Some(value);
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

impl Default for AiGatewayModelLimits {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AiGatewayModelLimits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayModelLimitsVisitor;
        impl<'a> Visitor<'a> for AiGatewayModelLimitsVisitor {
            type Value = AiGatewayModelLimits;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut context_window_tokens: Option<i64> = None;
                let mut max_input_tokens: Option<i64> = None;
                let mut max_output_tokens: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "context_window_tokens" => {
                            if v.is_null() {
                                continue;
                            }
                            context_window_tokens =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "max_input_tokens" => {
                            if v.is_null() {
                                continue;
                            }
                            max_input_tokens =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "max_output_tokens" => {
                            if v.is_null() {
                                continue;
                            }
                            max_output_tokens =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AiGatewayModelLimits {
                    context_window_tokens,
                    max_input_tokens,
                    max_output_tokens,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayModelLimitsVisitor)
    }
}
