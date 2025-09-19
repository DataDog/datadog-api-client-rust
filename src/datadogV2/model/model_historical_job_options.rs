// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Job options.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HistoricalJobOptions {
    /// The detection method.
    #[serde(rename = "detectionMethod")]
    pub detection_method: Option<crate::datadogV2::model::SecurityMonitoringRuleDetectionMethod>,
    /// A time window is specified to match when at least one of the cases matches true. This is a sliding window
    /// and evaluates in real time. For third party detection method, this field is not used.
    #[serde(rename = "evaluationWindow")]
    pub evaluation_window: Option<crate::datadogV2::model::SecurityMonitoringRuleEvaluationWindow>,
    /// Options on impossible travel detection method.
    #[serde(rename = "impossibleTravelOptions")]
    pub impossible_travel_options:
        Option<crate::datadogV2::model::SecurityMonitoringRuleImpossibleTravelOptions>,
    /// Once a signal is generated, the signal will remain "open" if a case is matched at least once within
    /// this keep alive window. For third party detection method, this field is not used.
    #[serde(rename = "keepAlive")]
    pub keep_alive: Option<crate::datadogV2::model::SecurityMonitoringRuleKeepAlive>,
    /// A signal will "close" regardless of the query being matched once the time exceeds the maximum duration.
    /// This time is calculated from the first seen timestamp.
    #[serde(rename = "maxSignalDuration")]
    pub max_signal_duration:
        Option<crate::datadogV2::model::SecurityMonitoringRuleMaxSignalDuration>,
    /// Options on new value detection method.
    #[serde(rename = "newValueOptions")]
    pub new_value_options: Option<crate::datadogV2::model::SecurityMonitoringRuleNewValueOptions>,
    /// Options on sequence detection method.
    #[serde(rename = "sequenceDetectionOptions")]
    pub sequence_detection_options:
        Option<crate::datadogV2::model::SecurityMonitoringRuleSequenceDetectionOptions>,
    /// Options on third party detection method.
    #[serde(rename = "thirdPartyRuleOptions")]
    pub third_party_rule_options:
        Option<crate::datadogV2::model::SecurityMonitoringRuleThirdPartyOptions>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HistoricalJobOptions {
    pub fn new() -> HistoricalJobOptions {
        HistoricalJobOptions {
            detection_method: None,
            evaluation_window: None,
            impossible_travel_options: None,
            keep_alive: None,
            max_signal_duration: None,
            new_value_options: None,
            sequence_detection_options: None,
            third_party_rule_options: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn detection_method(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleDetectionMethod,
    ) -> Self {
        self.detection_method = Some(value);
        self
    }

    pub fn evaluation_window(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleEvaluationWindow,
    ) -> Self {
        self.evaluation_window = Some(value);
        self
    }

    pub fn impossible_travel_options(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleImpossibleTravelOptions,
    ) -> Self {
        self.impossible_travel_options = Some(value);
        self
    }

    pub fn keep_alive(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleKeepAlive,
    ) -> Self {
        self.keep_alive = Some(value);
        self
    }

    pub fn max_signal_duration(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleMaxSignalDuration,
    ) -> Self {
        self.max_signal_duration = Some(value);
        self
    }

    pub fn new_value_options(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleNewValueOptions,
    ) -> Self {
        self.new_value_options = Some(value);
        self
    }

    pub fn sequence_detection_options(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleSequenceDetectionOptions,
    ) -> Self {
        self.sequence_detection_options = Some(value);
        self
    }

    pub fn third_party_rule_options(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleThirdPartyOptions,
    ) -> Self {
        self.third_party_rule_options = Some(value);
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

impl Default for HistoricalJobOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HistoricalJobOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HistoricalJobOptionsVisitor;
        impl<'a> Visitor<'a> for HistoricalJobOptionsVisitor {
            type Value = HistoricalJobOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut detection_method: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleDetectionMethod,
                > = None;
                let mut evaluation_window: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleEvaluationWindow,
                > = None;
                let mut impossible_travel_options: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleImpossibleTravelOptions,
                > = None;
                let mut keep_alive: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleKeepAlive,
                > = None;
                let mut max_signal_duration: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleMaxSignalDuration,
                > = None;
                let mut new_value_options: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleNewValueOptions,
                > = None;
                let mut sequence_detection_options: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleSequenceDetectionOptions,
                > = None;
                let mut third_party_rule_options: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleThirdPartyOptions,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "detectionMethod" => {
                            if v.is_null() {
                                continue;
                            }
                            detection_method =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _detection_method) = detection_method {
                                match _detection_method {
                                    crate::datadogV2::model::SecurityMonitoringRuleDetectionMethod::UnparsedObject(_detection_method) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "evaluationWindow" => {
                            if v.is_null() {
                                continue;
                            }
                            evaluation_window =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _evaluation_window) = evaluation_window {
                                match _evaluation_window {
                                    crate::datadogV2::model::SecurityMonitoringRuleEvaluationWindow::UnparsedObject(_evaluation_window) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "impossibleTravelOptions" => {
                            if v.is_null() {
                                continue;
                            }
                            impossible_travel_options =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "keepAlive" => {
                            if v.is_null() {
                                continue;
                            }
                            keep_alive = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _keep_alive) = keep_alive {
                                match _keep_alive {
                                    crate::datadogV2::model::SecurityMonitoringRuleKeepAlive::UnparsedObject(_keep_alive) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "maxSignalDuration" => {
                            if v.is_null() {
                                continue;
                            }
                            max_signal_duration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _max_signal_duration) = max_signal_duration {
                                match _max_signal_duration {
                                    crate::datadogV2::model::SecurityMonitoringRuleMaxSignalDuration::UnparsedObject(_max_signal_duration) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "newValueOptions" => {
                            if v.is_null() {
                                continue;
                            }
                            new_value_options =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sequenceDetectionOptions" => {
                            if v.is_null() {
                                continue;
                            }
                            sequence_detection_options =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "thirdPartyRuleOptions" => {
                            if v.is_null() {
                                continue;
                            }
                            third_party_rule_options =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = HistoricalJobOptions {
                    detection_method,
                    evaluation_window,
                    impossible_travel_options,
                    keep_alive,
                    max_signal_duration,
                    new_value_options,
                    sequence_detection_options,
                    third_party_rule_options,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HistoricalJobOptionsVisitor)
    }
}
