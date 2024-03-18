// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Options on new value rules.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringRuleNewValueOptions {
    /// The duration in days after which a learned value is forgotten.
    #[serde(rename = "forgetAfter")]
    pub forget_after:
        Option<crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsForgetAfter>,
    /// The duration in days during which values are learned, and after which signals will be generated for values that
    /// weren't learned. If set to 0, a signal will be generated for all new values after the first value is learned.
    #[serde(rename = "learningDuration")]
    pub learning_duration:
        Option<crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsLearningDuration>,
    /// The learning method used to determine when signals should be generated for values that weren't learned.
    #[serde(rename = "learningMethod")]
    pub learning_method:
        Option<crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsLearningMethod>,
    /// A number of occurrences after which signals will be generated for values that weren't learned.
    #[serde(rename = "learningThreshold")]
    pub learning_threshold:
        Option<crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsLearningThreshold>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringRuleNewValueOptions {
    pub fn new() -> SecurityMonitoringRuleNewValueOptions {
        SecurityMonitoringRuleNewValueOptions {
            forget_after: None,
            learning_duration: None,
            learning_method: None,
            learning_threshold: None,
            _unparsed: false,
        }
    }

    pub fn forget_after(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsForgetAfter,
    ) -> Self {
        self.forget_after = Some(value);
        self
    }

    pub fn learning_duration(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsLearningDuration,
    ) -> Self {
        self.learning_duration = Some(value);
        self
    }

    pub fn learning_method(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsLearningMethod,
    ) -> Self {
        self.learning_method = Some(value);
        self
    }

    pub fn learning_threshold(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsLearningThreshold,
    ) -> Self {
        self.learning_threshold = Some(value);
        self
    }
}

impl Default for SecurityMonitoringRuleNewValueOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleNewValueOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringRuleNewValueOptionsVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringRuleNewValueOptionsVisitor {
            type Value = SecurityMonitoringRuleNewValueOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut forget_after: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsForgetAfter,
                > = None;
                let mut learning_duration: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsLearningDuration,
                > = None;
                let mut learning_method: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsLearningMethod,
                > = None;
                let mut learning_threshold: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsLearningThreshold,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "forgetAfter" => {
                            if v.is_null() {
                                continue;
                            }
                            forget_after =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _forget_after) = forget_after {
                                match _forget_after {
                                    crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsForgetAfter::UnparsedObject(_forget_after) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "learningDuration" => {
                            if v.is_null() {
                                continue;
                            }
                            learning_duration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _learning_duration) = learning_duration {
                                match _learning_duration {
                                    crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsLearningDuration::UnparsedObject(_learning_duration) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "learningMethod" => {
                            if v.is_null() {
                                continue;
                            }
                            learning_method =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _learning_method) = learning_method {
                                match _learning_method {
                                    crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsLearningMethod::UnparsedObject(_learning_method) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "learningThreshold" => {
                            if v.is_null() {
                                continue;
                            }
                            learning_threshold =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _learning_threshold) = learning_threshold {
                                match _learning_threshold {
                                    crate::datadogV2::model::SecurityMonitoringRuleNewValueOptionsLearningThreshold::UnparsedObject(_learning_threshold) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = SecurityMonitoringRuleNewValueOptions {
                    forget_after,
                    learning_duration,
                    learning_method,
                    learning_threshold,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringRuleNewValueOptionsVisitor)
    }
}
