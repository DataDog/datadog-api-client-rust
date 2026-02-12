// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Options on anomaly detection method.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringRuleAnomalyDetectionOptions {
    /// Duration in seconds of the time buckets used to aggregate events matched by the rule.
    /// Must be greater than or equal to 300.
    #[serde(rename = "bucketDuration")]
    pub bucket_duration: Option<
        crate::datadogV2::model::SecurityMonitoringRuleAnomalyDetectionOptionsBucketDuration,
    >,
    /// An optional parameter that sets how permissive anomaly detection is.
    /// Higher values require higher deviations before triggering a signal.
    #[serde(rename = "detectionTolerance")]
    pub detection_tolerance: Option<
        crate::datadogV2::model::SecurityMonitoringRuleAnomalyDetectionOptionsDetectionTolerance,
    >,
    /// When set to true, Datadog uses previous values that fall within the defined learning window to construct the baseline, enabling the system to establish an accurate baseline more rapidly rather than relying solely on gradual learning over time.
    #[serde(rename = "instantaneousBaseline")]
    pub instantaneous_baseline: Option<bool>,
    /// Learning duration in hours. Anomaly detection waits for at least this amount of historical data before it starts evaluating.
    #[serde(rename = "learningDuration")]
    pub learning_duration: Option<
        crate::datadogV2::model::SecurityMonitoringRuleAnomalyDetectionOptionsLearningDuration,
    >,
    /// An optional override baseline to apply while the rule is in the learning period. Must be greater than or equal to 0.
    #[serde(rename = "learningPeriodBaseline")]
    pub learning_period_baseline: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringRuleAnomalyDetectionOptions {
    pub fn new() -> SecurityMonitoringRuleAnomalyDetectionOptions {
        SecurityMonitoringRuleAnomalyDetectionOptions {
            bucket_duration: None,
            detection_tolerance: None,
            instantaneous_baseline: None,
            learning_duration: None,
            learning_period_baseline: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn bucket_duration(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleAnomalyDetectionOptionsBucketDuration,
    ) -> Self {
        self.bucket_duration = Some(value);
        self
    }

    pub fn detection_tolerance(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleAnomalyDetectionOptionsDetectionTolerance,
    ) -> Self {
        self.detection_tolerance = Some(value);
        self
    }

    pub fn instantaneous_baseline(mut self, value: bool) -> Self {
        self.instantaneous_baseline = Some(value);
        self
    }

    pub fn learning_duration(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleAnomalyDetectionOptionsLearningDuration,
    ) -> Self {
        self.learning_duration = Some(value);
        self
    }

    pub fn learning_period_baseline(mut self, value: i64) -> Self {
        self.learning_period_baseline = Some(value);
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

impl Default for SecurityMonitoringRuleAnomalyDetectionOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleAnomalyDetectionOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringRuleAnomalyDetectionOptionsVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringRuleAnomalyDetectionOptionsVisitor {
            type Value = SecurityMonitoringRuleAnomalyDetectionOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bucket_duration: Option<crate::datadogV2::model::SecurityMonitoringRuleAnomalyDetectionOptionsBucketDuration> = None;
                let mut detection_tolerance: Option<crate::datadogV2::model::SecurityMonitoringRuleAnomalyDetectionOptionsDetectionTolerance> = None;
                let mut instantaneous_baseline: Option<bool> = None;
                let mut learning_duration: Option<crate::datadogV2::model::SecurityMonitoringRuleAnomalyDetectionOptionsLearningDuration> = None;
                let mut learning_period_baseline: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bucketDuration" => {
                            if v.is_null() {
                                continue;
                            }
                            bucket_duration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _bucket_duration) = bucket_duration {
                                match _bucket_duration {
                                    crate::datadogV2::model::SecurityMonitoringRuleAnomalyDetectionOptionsBucketDuration::UnparsedObject(_bucket_duration) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "detectionTolerance" => {
                            if v.is_null() {
                                continue;
                            }
                            detection_tolerance =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _detection_tolerance) = detection_tolerance {
                                match _detection_tolerance {
                                    crate::datadogV2::model::SecurityMonitoringRuleAnomalyDetectionOptionsDetectionTolerance::UnparsedObject(_detection_tolerance) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "instantaneousBaseline" => {
                            if v.is_null() {
                                continue;
                            }
                            instantaneous_baseline =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "learningDuration" => {
                            if v.is_null() {
                                continue;
                            }
                            learning_duration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _learning_duration) = learning_duration {
                                match _learning_duration {
                                    crate::datadogV2::model::SecurityMonitoringRuleAnomalyDetectionOptionsLearningDuration::UnparsedObject(_learning_duration) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "learningPeriodBaseline" => {
                            if v.is_null() {
                                continue;
                            }
                            learning_period_baseline =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SecurityMonitoringRuleAnomalyDetectionOptions {
                    bucket_duration,
                    detection_tolerance,
                    instantaneous_baseline,
                    learning_duration,
                    learning_period_baseline,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringRuleAnomalyDetectionOptionsVisitor)
    }
}
