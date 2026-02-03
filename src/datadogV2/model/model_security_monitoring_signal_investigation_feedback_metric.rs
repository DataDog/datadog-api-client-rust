// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A feedback metric containing user response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringSignalInvestigationFeedbackMetric {
    /// Placeholder text for the metric.
    #[serde(rename = "placeholder")]
    pub placeholder: Option<String>,
    /// The question or prompt.
    #[serde(rename = "prompt")]
    pub prompt: String,
    /// The user's response.
    #[serde(rename = "response")]
    pub response: String,
    /// The type of metric.
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringSignalInvestigationFeedbackMetric {
    pub fn new(
        prompt: String,
        response: String,
        type_: String,
    ) -> SecurityMonitoringSignalInvestigationFeedbackMetric {
        SecurityMonitoringSignalInvestigationFeedbackMetric {
            placeholder: None,
            prompt,
            response,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn placeholder(mut self, value: String) -> Self {
        self.placeholder = Some(value);
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

impl<'de> Deserialize<'de> for SecurityMonitoringSignalInvestigationFeedbackMetric {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringSignalInvestigationFeedbackMetricVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringSignalInvestigationFeedbackMetricVisitor {
            type Value = SecurityMonitoringSignalInvestigationFeedbackMetric;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut placeholder: Option<String> = None;
                let mut prompt: Option<String> = None;
                let mut response: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "placeholder" => {
                            if v.is_null() {
                                continue;
                            }
                            placeholder =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prompt" => {
                            prompt = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "response" => {
                            response = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let prompt = prompt.ok_or_else(|| M::Error::missing_field("prompt"))?;
                let response = response.ok_or_else(|| M::Error::missing_field("response"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SecurityMonitoringSignalInvestigationFeedbackMetric {
                    placeholder,
                    prompt,
                    response,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringSignalInvestigationFeedbackMetricVisitor)
    }
}
