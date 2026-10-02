// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Controls that determine which protocol settings can be changed in an experiment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsPublicProtocolResponseDataAttributesEnforcement {
    /// LOCKED prevents changes to confidence interval method. EDITABLE permits changes.
    #[serde(rename = "confidence_interval_method")]
    pub confidence_interval_method: Option<String>,
    /// LOCKED prevents changes to confidence level. EDITABLE permits changes.
    #[serde(rename = "confidence_level")]
    pub confidence_level: Option<String>,
    /// LOCKED prevents changes to CUPED variance reduction. EDITABLE permits changes.
    #[serde(rename = "cuped_calculation")]
    pub cuped_calculation: Option<String>,
    /// LOCKED prevents changes to default duration. EDITABLE permits changes.
    #[serde(rename = "default_duration")]
    pub default_duration: Option<String>,
    /// LOCKED prevents changes to environment. EDITABLE permits changes.
    #[serde(rename = "environment")]
    pub environment: Option<String>,
    /// LOCKED prevents changes to feature flag source. EDITABLE permits changes.
    #[serde(rename = "flag_source")]
    pub flag_source: Option<String>,
    /// LOCKED prevents changes to multiple testing correction. EDITABLE permits changes.
    #[serde(rename = "multiple_testing_correction")]
    pub multiple_testing_correction: Option<String>,
    /// LOCKED prevents changes to notifications. EDITABLE permits changes.
    #[serde(rename = "notifications")]
    pub notifications: Option<String>,
    /// LOCKED prevents changes to primary metric. EDITABLE permits changes.
    #[serde(rename = "primary_metric")]
    pub primary_metric: Option<String>,
    /// LOCKED prevents changes to secondary metrics. EDITABLE permits changes.
    #[serde(rename = "secondary_metrics")]
    pub secondary_metrics: Option<String>,
    /// LOCKED prevents changes to result exploration dimensions. EDITABLE permits changes.
    #[serde(rename = "split_by_exploration_dimensions")]
    pub split_by_exploration_dimensions: Option<String>,
    /// LOCKED prevents changes to subject type. EDITABLE permits changes.
    #[serde(rename = "subject_type")]
    pub subject_type: Option<String>,
    /// LOCKED prevents changes to targeting rules. EDITABLE permits changes.
    #[serde(rename = "targeting_rules")]
    pub targeting_rules: Option<String>,
    /// LOCKED prevents changes to traffic exposure. EDITABLE permits changes.
    #[serde(rename = "traffic_exposure")]
    pub traffic_exposure: Option<String>,
    /// LOCKED prevents changes to warehouse exposure source. EDITABLE permits changes.
    #[serde(rename = "warehouse_exposure_source")]
    pub warehouse_exposure_source: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsPublicProtocolResponseDataAttributesEnforcement {
    pub fn new() -> ExperimentsPublicProtocolResponseDataAttributesEnforcement {
        ExperimentsPublicProtocolResponseDataAttributesEnforcement {
            confidence_interval_method: None,
            confidence_level: None,
            cuped_calculation: None,
            default_duration: None,
            environment: None,
            flag_source: None,
            multiple_testing_correction: None,
            notifications: None,
            primary_metric: None,
            secondary_metrics: None,
            split_by_exploration_dimensions: None,
            subject_type: None,
            targeting_rules: None,
            traffic_exposure: None,
            warehouse_exposure_source: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn confidence_interval_method(mut self, value: String) -> Self {
        self.confidence_interval_method = Some(value);
        self
    }

    pub fn confidence_level(mut self, value: String) -> Self {
        self.confidence_level = Some(value);
        self
    }

    pub fn cuped_calculation(mut self, value: String) -> Self {
        self.cuped_calculation = Some(value);
        self
    }

    pub fn default_duration(mut self, value: String) -> Self {
        self.default_duration = Some(value);
        self
    }

    pub fn environment(mut self, value: String) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn flag_source(mut self, value: String) -> Self {
        self.flag_source = Some(value);
        self
    }

    pub fn multiple_testing_correction(mut self, value: String) -> Self {
        self.multiple_testing_correction = Some(value);
        self
    }

    pub fn notifications(mut self, value: String) -> Self {
        self.notifications = Some(value);
        self
    }

    pub fn primary_metric(mut self, value: String) -> Self {
        self.primary_metric = Some(value);
        self
    }

    pub fn secondary_metrics(mut self, value: String) -> Self {
        self.secondary_metrics = Some(value);
        self
    }

    pub fn split_by_exploration_dimensions(mut self, value: String) -> Self {
        self.split_by_exploration_dimensions = Some(value);
        self
    }

    pub fn subject_type(mut self, value: String) -> Self {
        self.subject_type = Some(value);
        self
    }

    pub fn targeting_rules(mut self, value: String) -> Self {
        self.targeting_rules = Some(value);
        self
    }

    pub fn traffic_exposure(mut self, value: String) -> Self {
        self.traffic_exposure = Some(value);
        self
    }

    pub fn warehouse_exposure_source(mut self, value: String) -> Self {
        self.warehouse_exposure_source = Some(value);
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

impl Default for ExperimentsPublicProtocolResponseDataAttributesEnforcement {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsPublicProtocolResponseDataAttributesEnforcement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsPublicProtocolResponseDataAttributesEnforcementVisitor;
        impl<'a> Visitor<'a> for ExperimentsPublicProtocolResponseDataAttributesEnforcementVisitor {
            type Value = ExperimentsPublicProtocolResponseDataAttributesEnforcement;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut confidence_interval_method: Option<String> = None;
                let mut confidence_level: Option<String> = None;
                let mut cuped_calculation: Option<String> = None;
                let mut default_duration: Option<String> = None;
                let mut environment: Option<String> = None;
                let mut flag_source: Option<String> = None;
                let mut multiple_testing_correction: Option<String> = None;
                let mut notifications: Option<String> = None;
                let mut primary_metric: Option<String> = None;
                let mut secondary_metrics: Option<String> = None;
                let mut split_by_exploration_dimensions: Option<String> = None;
                let mut subject_type: Option<String> = None;
                let mut targeting_rules: Option<String> = None;
                let mut traffic_exposure: Option<String> = None;
                let mut warehouse_exposure_source: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "confidence_interval_method" => {
                            if v.is_null() {
                                continue;
                            }
                            confidence_interval_method =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "confidence_level" => {
                            if v.is_null() {
                                continue;
                            }
                            confidence_level =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cuped_calculation" => {
                            if v.is_null() {
                                continue;
                            }
                            cuped_calculation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "default_duration" => {
                            if v.is_null() {
                                continue;
                            }
                            default_duration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "environment" => {
                            if v.is_null() {
                                continue;
                            }
                            environment =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "flag_source" => {
                            if v.is_null() {
                                continue;
                            }
                            flag_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "multiple_testing_correction" => {
                            if v.is_null() {
                                continue;
                            }
                            multiple_testing_correction =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notifications" => {
                            if v.is_null() {
                                continue;
                            }
                            notifications =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "primary_metric" => {
                            if v.is_null() {
                                continue;
                            }
                            primary_metric =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "secondary_metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            secondary_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "split_by_exploration_dimensions" => {
                            if v.is_null() {
                                continue;
                            }
                            split_by_exploration_dimensions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subject_type" => {
                            if v.is_null() {
                                continue;
                            }
                            subject_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "targeting_rules" => {
                            if v.is_null() {
                                continue;
                            }
                            targeting_rules =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "traffic_exposure" => {
                            if v.is_null() {
                                continue;
                            }
                            traffic_exposure =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warehouse_exposure_source" => {
                            if v.is_null() {
                                continue;
                            }
                            warehouse_exposure_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsPublicProtocolResponseDataAttributesEnforcement {
                    confidence_interval_method,
                    confidence_level,
                    cuped_calculation,
                    default_duration,
                    environment,
                    flag_source,
                    multiple_testing_correction,
                    notifications,
                    primary_metric,
                    secondary_metrics,
                    split_by_exploration_dimensions,
                    subject_type,
                    targeting_rules,
                    traffic_exposure,
                    warehouse_exposure_source,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ExperimentsPublicProtocolResponseDataAttributesEnforcementVisitor)
    }
}
