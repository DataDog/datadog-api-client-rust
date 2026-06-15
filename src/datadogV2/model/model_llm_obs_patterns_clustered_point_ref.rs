// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A clustered point attached inline to a topic. The metric fields are populated
/// only when the request includes `include_metrics=true`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsPatternsClusteredPointRef {
    /// Duration of the source span in nanoseconds. Included only when metrics are requested.
    #[serde(rename = "duration")]
    pub duration: Option<f64>,
    /// Estimated total cost of the source span. Included only when metrics are requested.
    #[serde(rename = "estimated_total_cost")]
    pub estimated_total_cost: Option<f64>,
    /// Evaluation results for the source span keyed by evaluation name. Included
    /// only when metrics are requested.
    #[serde(rename = "evaluation")]
    pub evaluation: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Number of input tokens of the source span. Included only when metrics are requested.
    #[serde(rename = "input_tokens")]
    pub input_tokens: Option<f64>,
    /// Number of output tokens of the source span. Included only when metrics are requested.
    #[serde(rename = "output_tokens")]
    pub output_tokens: Option<f64>,
    /// Identifier of the source span.
    #[serde(rename = "span_id")]
    pub span_id: String,
    /// Status of the source span. Included only when metrics are requested.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// Total number of tokens of the source span. Included only when metrics are requested.
    #[serde(rename = "total_tokens")]
    pub total_tokens: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsPatternsClusteredPointRef {
    pub fn new(span_id: String) -> LLMObsPatternsClusteredPointRef {
        LLMObsPatternsClusteredPointRef {
            duration: None,
            estimated_total_cost: None,
            evaluation: None,
            input_tokens: None,
            output_tokens: None,
            span_id,
            status: None,
            total_tokens: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn duration(mut self, value: f64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn estimated_total_cost(mut self, value: f64) -> Self {
        self.estimated_total_cost = Some(value);
        self
    }

    pub fn evaluation(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.evaluation = Some(value);
        self
    }

    pub fn input_tokens(mut self, value: f64) -> Self {
        self.input_tokens = Some(value);
        self
    }

    pub fn output_tokens(mut self, value: f64) -> Self {
        self.output_tokens = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
        self
    }

    pub fn total_tokens(mut self, value: f64) -> Self {
        self.total_tokens = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsPatternsClusteredPointRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsPatternsClusteredPointRefVisitor;
        impl<'a> Visitor<'a> for LLMObsPatternsClusteredPointRefVisitor {
            type Value = LLMObsPatternsClusteredPointRef;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut duration: Option<f64> = None;
                let mut estimated_total_cost: Option<f64> = None;
                let mut evaluation: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut input_tokens: Option<f64> = None;
                let mut output_tokens: Option<f64> = None;
                let mut span_id: Option<String> = None;
                let mut status: Option<String> = None;
                let mut total_tokens: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "duration" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "estimated_total_cost" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            estimated_total_cost =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "evaluation" => {
                            if v.is_null() {
                                continue;
                            }
                            evaluation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "input_tokens" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            input_tokens =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "output_tokens" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            output_tokens =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "span_id" => {
                            span_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_tokens" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            total_tokens =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let span_id = span_id.ok_or_else(|| M::Error::missing_field("span_id"))?;

                let content = LLMObsPatternsClusteredPointRef {
                    duration,
                    estimated_total_cost,
                    evaluation,
                    input_tokens,
                    output_tokens,
                    span_id,
                    status,
                    total_tokens,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsPatternsClusteredPointRefVisitor)
    }
}
