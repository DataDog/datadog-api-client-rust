// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Complete lock map. Omit this object to preserve current locks. Null is rejected; every field is required.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsReplaceProtocolEnforcementInput {
    /// Always LOCKED. The server keeps this setting locked regardless of the supplied value.
    #[serde(rename = "confidence_interval_method")]
    pub confidence_interval_method: String,
    /// LOCKED prevents changes to confidence level. EDITABLE permits changes.
    #[serde(rename = "confidence_level")]
    pub confidence_level: String,
    /// LOCKED prevents changes to CUPED variance reduction. EDITABLE permits changes.
    #[serde(rename = "cuped_calculation")]
    pub cuped_calculation: String,
    /// LOCKED prevents changes to default duration. EDITABLE permits changes.
    #[serde(rename = "default_duration")]
    pub default_duration: String,
    /// LOCKED prevents changes to environment. EDITABLE permits changes.
    #[serde(rename = "environment")]
    pub environment: String,
    /// Always LOCKED. The server keeps this setting locked regardless of the supplied value.
    #[serde(rename = "flag_source")]
    pub flag_source: String,
    /// LOCKED prevents changes to multiple testing correction. EDITABLE permits changes.
    #[serde(rename = "multiple_testing_correction")]
    pub multiple_testing_correction: String,
    /// LOCKED prevents changes to notifications. EDITABLE permits changes.
    #[serde(rename = "notifications")]
    pub notifications: String,
    /// LOCKED prevents changes to primary metric. EDITABLE permits changes.
    #[serde(rename = "primary_metric")]
    pub primary_metric: String,
    /// LOCKED prevents changes to secondary metrics. EDITABLE permits changes.
    #[serde(rename = "secondary_metrics")]
    pub secondary_metrics: String,
    /// LOCKED prevents changes to result exploration dimensions. EDITABLE permits changes.
    #[serde(rename = "split_by_exploration_dimensions")]
    pub split_by_exploration_dimensions: String,
    /// Always LOCKED. The server keeps this setting locked regardless of the supplied value.
    #[serde(rename = "subject_type")]
    pub subject_type: String,
    /// LOCKED prevents changes to targeting rules. EDITABLE permits changes.
    #[serde(rename = "targeting_rules")]
    pub targeting_rules: String,
    /// LOCKED prevents changes to traffic exposure. EDITABLE permits changes.
    #[serde(rename = "traffic_exposure")]
    pub traffic_exposure: String,
    /// LOCKED prevents changes to warehouse exposure source. EDITABLE permits changes.
    #[serde(rename = "warehouse_exposure_source")]
    pub warehouse_exposure_source: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsReplaceProtocolEnforcementInput {
    pub fn new(
        confidence_interval_method: String,
        confidence_level: String,
        cuped_calculation: String,
        default_duration: String,
        environment: String,
        flag_source: String,
        multiple_testing_correction: String,
        notifications: String,
        primary_metric: String,
        secondary_metrics: String,
        split_by_exploration_dimensions: String,
        subject_type: String,
        targeting_rules: String,
        traffic_exposure: String,
        warehouse_exposure_source: String,
    ) -> ExperimentsReplaceProtocolEnforcementInput {
        ExperimentsReplaceProtocolEnforcementInput {
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

impl<'de> Deserialize<'de> for ExperimentsReplaceProtocolEnforcementInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsReplaceProtocolEnforcementInputVisitor;
        impl<'a> Visitor<'a> for ExperimentsReplaceProtocolEnforcementInputVisitor {
            type Value = ExperimentsReplaceProtocolEnforcementInput;

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
                            confidence_interval_method =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "confidence_level" => {
                            confidence_level =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cuped_calculation" => {
                            cuped_calculation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "default_duration" => {
                            default_duration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "environment" => {
                            environment =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "flag_source" => {
                            flag_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "multiple_testing_correction" => {
                            multiple_testing_correction =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notifications" => {
                            notifications =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "primary_metric" => {
                            primary_metric =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "secondary_metrics" => {
                            secondary_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "split_by_exploration_dimensions" => {
                            split_by_exploration_dimensions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subject_type" => {
                            subject_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "targeting_rules" => {
                            targeting_rules =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "traffic_exposure" => {
                            traffic_exposure =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warehouse_exposure_source" => {
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
                let confidence_interval_method = confidence_interval_method
                    .ok_or_else(|| M::Error::missing_field("confidence_interval_method"))?;
                let confidence_level =
                    confidence_level.ok_or_else(|| M::Error::missing_field("confidence_level"))?;
                let cuped_calculation = cuped_calculation
                    .ok_or_else(|| M::Error::missing_field("cuped_calculation"))?;
                let default_duration =
                    default_duration.ok_or_else(|| M::Error::missing_field("default_duration"))?;
                let environment =
                    environment.ok_or_else(|| M::Error::missing_field("environment"))?;
                let flag_source =
                    flag_source.ok_or_else(|| M::Error::missing_field("flag_source"))?;
                let multiple_testing_correction = multiple_testing_correction
                    .ok_or_else(|| M::Error::missing_field("multiple_testing_correction"))?;
                let notifications =
                    notifications.ok_or_else(|| M::Error::missing_field("notifications"))?;
                let primary_metric =
                    primary_metric.ok_or_else(|| M::Error::missing_field("primary_metric"))?;
                let secondary_metrics = secondary_metrics
                    .ok_or_else(|| M::Error::missing_field("secondary_metrics"))?;
                let split_by_exploration_dimensions = split_by_exploration_dimensions
                    .ok_or_else(|| M::Error::missing_field("split_by_exploration_dimensions"))?;
                let subject_type =
                    subject_type.ok_or_else(|| M::Error::missing_field("subject_type"))?;
                let targeting_rules =
                    targeting_rules.ok_or_else(|| M::Error::missing_field("targeting_rules"))?;
                let traffic_exposure =
                    traffic_exposure.ok_or_else(|| M::Error::missing_field("traffic_exposure"))?;
                let warehouse_exposure_source = warehouse_exposure_source
                    .ok_or_else(|| M::Error::missing_field("warehouse_exposure_source"))?;

                let content = ExperimentsReplaceProtocolEnforcementInput {
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

        deserializer.deserialize_any(ExperimentsReplaceProtocolEnforcementInputVisitor)
    }
}
