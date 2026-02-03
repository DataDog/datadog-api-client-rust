// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for investigation feedback.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringSignalInvestigationFeedbackRequestAttributes {
    /// The feedback text.
    #[serde(rename = "feedback")]
    pub feedback: String,
    /// Structured feedback content.
    #[serde(rename = "feedback_content")]
    pub feedback_content:
        Option<Vec<crate::datadogV2::model::SecurityMonitoringSignalInvestigationFeedbackSection>>,
    /// Whether the feedback is incomplete.
    #[serde(rename = "incomplete")]
    pub incomplete: Option<bool>,
    /// The rating value.
    #[serde(rename = "rating")]
    pub rating: Option<String>,
    /// The unique ID of the security signal.
    #[serde(rename = "signal_id")]
    pub signal_id: String,
    /// The type of feedback.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringSignalInvestigationFeedbackRequestAttributes {
    pub fn new(
        feedback: String,
        signal_id: String,
    ) -> SecurityMonitoringSignalInvestigationFeedbackRequestAttributes {
        SecurityMonitoringSignalInvestigationFeedbackRequestAttributes {
            feedback,
            feedback_content: None,
            incomplete: None,
            rating: None,
            signal_id,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn feedback_content(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringSignalInvestigationFeedbackSection>,
    ) -> Self {
        self.feedback_content = Some(value);
        self
    }

    pub fn incomplete(mut self, value: bool) -> Self {
        self.incomplete = Some(value);
        self
    }

    pub fn rating(mut self, value: String) -> Self {
        self.rating = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
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

impl<'de> Deserialize<'de> for SecurityMonitoringSignalInvestigationFeedbackRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringSignalInvestigationFeedbackRequestAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringSignalInvestigationFeedbackRequestAttributesVisitor {
            type Value = SecurityMonitoringSignalInvestigationFeedbackRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut feedback: Option<String> = None;
                let mut feedback_content: Option<Vec<crate::datadogV2::model::SecurityMonitoringSignalInvestigationFeedbackSection>> = None;
                let mut incomplete: Option<bool> = None;
                let mut rating: Option<String> = None;
                let mut signal_id: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "feedback" => {
                            feedback = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "feedback_content" => {
                            if v.is_null() {
                                continue;
                            }
                            feedback_content =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incomplete" => {
                            if v.is_null() {
                                continue;
                            }
                            incomplete = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rating" => {
                            if v.is_null() {
                                continue;
                            }
                            rating = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "signal_id" => {
                            signal_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let feedback = feedback.ok_or_else(|| M::Error::missing_field("feedback"))?;
                let signal_id = signal_id.ok_or_else(|| M::Error::missing_field("signal_id"))?;

                let content = SecurityMonitoringSignalInvestigationFeedbackRequestAttributes {
                    feedback,
                    feedback_content,
                    incomplete,
                    rating,
                    signal_id,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(SecurityMonitoringSignalInvestigationFeedbackRequestAttributesVisitor)
    }
}
