// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Partial lock map. Omitted fields remain unchanged. Null clears the whole map. Empty or null field values become `EDITABLE`, except `subject_type`, `flag_source`, and `confidence_interval_method`, which remain `LOCKED`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsPatchProtocolEnforcementInput {
    /// Always `LOCKED`. The server keeps this setting locked regardless of the supplied value.
    #[serde(
        rename = "confidence_interval_method",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub confidence_interval_method: Option<Option<String>>,
    /// LOCKED prevents changes to confidence level. EDITABLE permits changes.
    #[serde(
        rename = "confidence_level",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub confidence_level: Option<Option<String>>,
    /// LOCKED prevents changes to CUPED variance reduction. EDITABLE permits changes.
    #[serde(
        rename = "cuped_calculation",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub cuped_calculation: Option<Option<String>>,
    /// LOCKED prevents changes to default duration. EDITABLE permits changes.
    #[serde(
        rename = "default_duration",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub default_duration: Option<Option<String>>,
    /// LOCKED prevents changes to environment. EDITABLE permits changes.
    #[serde(
        rename = "environment",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub environment: Option<Option<String>>,
    /// Always LOCKED. The server keeps this setting locked regardless of the supplied value.
    #[serde(
        rename = "flag_source",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub flag_source: Option<Option<String>>,
    /// LOCKED prevents changes to multiple testing correction. EDITABLE permits changes.
    #[serde(
        rename = "multiple_testing_correction",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub multiple_testing_correction: Option<Option<String>>,
    /// LOCKED prevents changes to notifications. EDITABLE permits changes.
    #[serde(
        rename = "notifications",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub notifications: Option<Option<String>>,
    /// LOCKED prevents changes to primary metric. EDITABLE permits changes.
    #[serde(
        rename = "primary_metric",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub primary_metric: Option<Option<String>>,
    /// LOCKED prevents changes to secondary metrics. EDITABLE permits changes.
    #[serde(
        rename = "secondary_metrics",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub secondary_metrics: Option<Option<String>>,
    /// LOCKED prevents changes to result exploration dimensions. EDITABLE permits changes.
    #[serde(
        rename = "split_by_exploration_dimensions",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub split_by_exploration_dimensions: Option<Option<String>>,
    /// Always LOCKED. The server keeps this setting locked regardless of the supplied value.
    #[serde(
        rename = "subject_type",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub subject_type: Option<Option<String>>,
    /// LOCKED prevents changes to targeting rules. EDITABLE permits changes.
    #[serde(
        rename = "targeting_rules",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub targeting_rules: Option<Option<String>>,
    /// LOCKED prevents changes to traffic exposure. EDITABLE permits changes.
    #[serde(
        rename = "traffic_exposure",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub traffic_exposure: Option<Option<String>>,
    /// LOCKED prevents changes to warehouse exposure source. EDITABLE permits changes.
    #[serde(
        rename = "warehouse_exposure_source",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub warehouse_exposure_source: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsPatchProtocolEnforcementInput {
    pub fn new() -> ExperimentsPatchProtocolEnforcementInput {
        ExperimentsPatchProtocolEnforcementInput {
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

    pub fn confidence_interval_method(mut self, value: Option<String>) -> Self {
        self.confidence_interval_method = Some(value);
        self
    }

    pub fn confidence_level(mut self, value: Option<String>) -> Self {
        self.confidence_level = Some(value);
        self
    }

    pub fn cuped_calculation(mut self, value: Option<String>) -> Self {
        self.cuped_calculation = Some(value);
        self
    }

    pub fn default_duration(mut self, value: Option<String>) -> Self {
        self.default_duration = Some(value);
        self
    }

    pub fn environment(mut self, value: Option<String>) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn flag_source(mut self, value: Option<String>) -> Self {
        self.flag_source = Some(value);
        self
    }

    pub fn multiple_testing_correction(mut self, value: Option<String>) -> Self {
        self.multiple_testing_correction = Some(value);
        self
    }

    pub fn notifications(mut self, value: Option<String>) -> Self {
        self.notifications = Some(value);
        self
    }

    pub fn primary_metric(mut self, value: Option<String>) -> Self {
        self.primary_metric = Some(value);
        self
    }

    pub fn secondary_metrics(mut self, value: Option<String>) -> Self {
        self.secondary_metrics = Some(value);
        self
    }

    pub fn split_by_exploration_dimensions(mut self, value: Option<String>) -> Self {
        self.split_by_exploration_dimensions = Some(value);
        self
    }

    pub fn subject_type(mut self, value: Option<String>) -> Self {
        self.subject_type = Some(value);
        self
    }

    pub fn targeting_rules(mut self, value: Option<String>) -> Self {
        self.targeting_rules = Some(value);
        self
    }

    pub fn traffic_exposure(mut self, value: Option<String>) -> Self {
        self.traffic_exposure = Some(value);
        self
    }

    pub fn warehouse_exposure_source(mut self, value: Option<String>) -> Self {
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

impl Default for ExperimentsPatchProtocolEnforcementInput {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsPatchProtocolEnforcementInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsPatchProtocolEnforcementInputVisitor;
        impl<'a> Visitor<'a> for ExperimentsPatchProtocolEnforcementInputVisitor {
            type Value = ExperimentsPatchProtocolEnforcementInput;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut confidence_interval_method: Option<Option<String>> = None;
                let mut confidence_level: Option<Option<String>> = None;
                let mut cuped_calculation: Option<Option<String>> = None;
                let mut default_duration: Option<Option<String>> = None;
                let mut environment: Option<Option<String>> = None;
                let mut flag_source: Option<Option<String>> = None;
                let mut multiple_testing_correction: Option<Option<String>> = None;
                let mut notifications: Option<Option<String>> = None;
                let mut primary_metric: Option<Option<String>> = None;
                let mut secondary_metrics: Option<Option<String>> = None;
                let mut split_by_exploration_dimensions: Option<Option<String>> = None;
                let mut subject_type: Option<Option<String>> = None;
                let mut targeting_rules: Option<Option<String>> = None;
                let mut traffic_exposure: Option<Option<String>> = None;
                let mut warehouse_exposure_source: Option<Option<String>> = None;
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

                let content = ExperimentsPatchProtocolEnforcementInput {
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

        deserializer.deserialize_any(ExperimentsPatchProtocolEnforcementInputVisitor)
    }
}
