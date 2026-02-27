// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for pushing experiment events including spans and metrics.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsExperimentEventsDataAttributesRequest {
    /// List of metrics to push for the experiment.
    #[serde(rename = "metrics")]
    pub metrics: Option<Vec<crate::datadogV2::model::LLMObsExperimentMetric>>,
    /// List of spans to push for the experiment.
    #[serde(rename = "spans")]
    pub spans: Option<Vec<crate::datadogV2::model::LLMObsExperimentSpan>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsExperimentEventsDataAttributesRequest {
    pub fn new() -> LLMObsExperimentEventsDataAttributesRequest {
        LLMObsExperimentEventsDataAttributesRequest {
            metrics: None,
            spans: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn metrics(mut self, value: Vec<crate::datadogV2::model::LLMObsExperimentMetric>) -> Self {
        self.metrics = Some(value);
        self
    }

    pub fn spans(mut self, value: Vec<crate::datadogV2::model::LLMObsExperimentSpan>) -> Self {
        self.spans = Some(value);
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

impl Default for LLMObsExperimentEventsDataAttributesRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsExperimentEventsDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsExperimentEventsDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for LLMObsExperimentEventsDataAttributesRequestVisitor {
            type Value = LLMObsExperimentEventsDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut metrics: Option<Vec<crate::datadogV2::model::LLMObsExperimentMetric>> =
                    None;
                let mut spans: Option<Vec<crate::datadogV2::model::LLMObsExperimentSpan>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            metrics = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "spans" => {
                            if v.is_null() {
                                continue;
                            }
                            spans = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LLMObsExperimentEventsDataAttributesRequest {
                    metrics,
                    spans,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsExperimentEventsDataAttributesRequestVisitor)
    }
}
