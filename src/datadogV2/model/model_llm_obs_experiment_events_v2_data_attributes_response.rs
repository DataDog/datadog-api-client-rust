// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an experiment events response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsExperimentEventsV2DataAttributesResponse {
    /// Experiment spans, each enriched with their associated evaluation metrics.
    #[serde(rename = "spans")]
    pub spans: Vec<crate::datadogV2::model::LLMObsExperimentSpanWithEvals>,
    /// Experiment-level summary evaluation metrics (not tied to individual spans).
    #[serde(rename = "summary_metrics")]
    pub summary_metrics: Vec<crate::datadogV2::model::LLMObsExperimentEvalMetricEvent>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsExperimentEventsV2DataAttributesResponse {
    pub fn new(
        spans: Vec<crate::datadogV2::model::LLMObsExperimentSpanWithEvals>,
        summary_metrics: Vec<crate::datadogV2::model::LLMObsExperimentEvalMetricEvent>,
    ) -> LLMObsExperimentEventsV2DataAttributesResponse {
        LLMObsExperimentEventsV2DataAttributesResponse {
            spans,
            summary_metrics,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for LLMObsExperimentEventsV2DataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsExperimentEventsV2DataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for LLMObsExperimentEventsV2DataAttributesResponseVisitor {
            type Value = LLMObsExperimentEventsV2DataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut spans: Option<Vec<crate::datadogV2::model::LLMObsExperimentSpanWithEvals>> =
                    None;
                let mut summary_metrics: Option<
                    Vec<crate::datadogV2::model::LLMObsExperimentEvalMetricEvent>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "spans" => {
                            spans = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "summary_metrics" => {
                            summary_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let spans = spans.ok_or_else(|| M::Error::missing_field("spans"))?;
                let summary_metrics =
                    summary_metrics.ok_or_else(|| M::Error::missing_field("summary_metrics"))?;

                let content = LLMObsExperimentEventsV2DataAttributesResponse {
                    spans,
                    summary_metrics,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsExperimentEventsV2DataAttributesResponseVisitor)
    }
}
