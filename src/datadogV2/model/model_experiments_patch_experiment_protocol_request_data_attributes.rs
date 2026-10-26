// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Partial protocol update. Omitted attributes remain unchanged. Published protocols accept only name, description, and migration_metadata; archived protocols reject updates.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsPatchExperimentProtocolRequestDataAttributes {
    /// Partial statistical settings. Omitted fields remain unchanged. Null clears the whole plan. Null clears nullable fields; null or an empty string clears confidence_interval_method.
    #[serde(rename = "analysis_plan", default, with = "::serde_with::rust::double_option")]
    pub analysis_plan: Option<Option<crate::datadogV2::model::ExperimentsPatchProtocolAnalysisPlanInput>>,
    /// Assignment properties. Omission or null preserves them; an array replaces them.
    #[serde(rename = "assignment_source_default_properties", default, with = "::serde_with::rust::double_option")]
    pub assignment_source_default_properties: Option<Option<Vec<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesAssignmentSourceDefaultPropertiesItems>>>,
    /// ID of the assignment source selected by the protocol. Omission or null keeps the current value. An empty string clears it.
    #[serde(rename = "assignment_source_id", default, with = "::serde_with::rust::double_option")]
    pub assignment_source_id: Option<Option<String>>,
    /// Default experiment duration supplied by the protocol, in days. Omission preserves it; null clears it. Sets analysis_plan.target_duration_days on experiments created from this protocol.
    #[serde(rename = "default_duration_days", default, with = "::serde_with::rust::double_option")]
    pub default_duration_days: Option<Option<i64>>,
    /// Description. Empty string clears it; omission or null preserves it.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option")]
    pub description: Option<Option<String>>,
    /// Partial lock map. Omitted fields remain unchanged. Null clears the whole map. Empty or null field values become `EDITABLE`, except `subject_type`, `flag_source`, and `confidence_interval_method`, which remain `LOCKED`.
    #[serde(rename = "enforcement", default, with = "::serde_with::rust::double_option")]
    pub enforcement: Option<Option<crate::datadogV2::model::ExperimentsPatchProtocolEnforcementInput>>,
    /// ID of the feature flag environment used by the experiment. Omission or null keeps the current value. An empty string clears it.
    #[serde(rename = "environment_id", default, with = "::serde_with::rust::double_option")]
    pub environment_id: Option<Option<String>>,
    /// Partial traffic schedule. Omitted fields remain unchanged. Null clears the whole schedule. Null clears nullable fields and rollout_steps; null sets strategy to an empty string and `autostart` to false.
    #[serde(rename = "exposure_schedule", default, with = "::serde_with::rust::double_option")]
    pub exposure_schedule: Option<Option<crate::datadogV2::model::ExperimentsPatchProtocolExposureScheduleInput>>,
    /// Whether the protocol requires a duration before an experiment can start. Omission or null preserves it.
    #[serde(rename = "is_duration_required_to_start", default, with = "::serde_with::rust::double_option")]
    pub is_duration_required_to_start: Option<Option<bool>>,
    /// Whether the protocol requires equal traffic allocation across variants. Omission or null preserves it.
    #[serde(rename = "is_equal_split_enforced", default, with = "::serde_with::rust::double_option")]
    pub is_equal_split_enforced: Option<Option<bool>>,
    /// Whether the protocol limits the number of metrics. Omission or null preserves it.
    #[serde(rename = "is_metric_limit_enabled", default, with = "::serde_with::rust::double_option")]
    pub is_metric_limit_enabled: Option<Option<bool>>,
    /// Whether the protocol enforces a minimum experiment duration. Omission or null preserves it.
    #[serde(rename = "is_minimum_duration_enabled", default, with = "::serde_with::rust::double_option")]
    pub is_minimum_duration_enabled: Option<Option<bool>>,
    /// Metric groups. Omission or null preserves them; an array replaces them.
    #[serde(rename = "metric_groups", default, with = "::serde_with::rust::double_option")]
    pub metric_groups: Option<Option<Vec<crate::datadogV2::model::ExperimentsProtocolMetricGroupInput>>>,
    /// Maximum number of metrics allowed by the protocol. Omission preserves it; null clears it.
    #[serde(rename = "metric_limit", default, with = "::serde_with::rust::double_option")]
    pub metric_limit: Option<Option<i64>>,
    /// Metadata for imported resources. Must encode to at most 4096 bytes. Null clears it; omission preserves it on update.
    #[serde(rename = "migration_metadata", default, with = "::serde_with::rust::double_option")]
    pub migration_metadata: Option<Option<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// Minimum duration unit, `DAYS` or `WEEKS`. Empty string clears it. Omission or null preserves it.
    #[serde(rename = "minimum_duration_unit", default, with = "::serde_with::rust::double_option")]
    pub minimum_duration_unit: Option<Option<String>>,
    /// Minimum experiment duration in the specified unit. Omission preserves it; null clears it.
    #[serde(rename = "minimum_duration_value", default, with = "::serde_with::rust::double_option")]
    pub minimum_duration_value: Option<Option<i64>>,
    /// Protocol name. Leading and trailing spaces are removed. The trimmed name must contain 1 to 255 characters.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option")]
    pub name: Option<Option<String>>,
    /// ID of the primary metric supplied by the protocol. Omission or null keeps the current value. An empty string clears it.
    #[serde(rename = "primary_metric_id", default, with = "::serde_with::rust::double_option")]
    pub primary_metric_id: Option<Option<String>>,
    /// ID of the subject type used by this configuration. Omission or null keeps the current value. An empty string clears it.
    #[serde(rename = "subject_type_id", default, with = "::serde_with::rust::double_option")]
    pub subject_type_id: Option<Option<String>>,
    /// Targeting rules. Omission or null preserves them; an array replaces them.
    #[serde(rename = "targeting_rules", default, with = "::serde_with::rust::double_option")]
    pub targeting_rules: Option<Option<Vec<crate::datadogV2::model::ExperimentsProtocolTargetingRuleInput>>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsPatchExperimentProtocolRequestDataAttributes {
    pub fn new() -> ExperimentsPatchExperimentProtocolRequestDataAttributes {
        ExperimentsPatchExperimentProtocolRequestDataAttributes {
            analysis_plan: None,
            assignment_source_default_properties: None,
            assignment_source_id: None,
            default_duration_days: None,
            description: None,
            enforcement: None,
            environment_id: None,
            exposure_schedule: None,
            is_duration_required_to_start: None,
            is_equal_split_enforced: None,
            is_metric_limit_enabled: None,
            is_minimum_duration_enabled: None,
            metric_groups: None,
            metric_limit: None,
            migration_metadata: None,
            minimum_duration_unit: None,
            minimum_duration_value: None,
            name: None,
            primary_metric_id: None,
            subject_type_id: None,
            targeting_rules: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn analysis_plan(
        mut self,
        value: Option<crate::datadogV2::model::ExperimentsPatchProtocolAnalysisPlanInput>,
    ) -> Self {
        self.analysis_plan = Some(value);
        self
    }

    pub fn assignment_source_default_properties(
        mut self,
        value: Option<Vec<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesAssignmentSourceDefaultPropertiesItems>>,
    ) -> Self {
        self.assignment_source_default_properties = Some(value);
        self
    }

    pub fn assignment_source_id(mut self, value: Option<String>) -> Self {
        self.assignment_source_id = Some(value);
        self
    }

    pub fn default_duration_days(mut self, value: Option<i64>) -> Self {
        self.default_duration_days = Some(value);
        self
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn enforcement(
        mut self,
        value: Option<crate::datadogV2::model::ExperimentsPatchProtocolEnforcementInput>,
    ) -> Self {
        self.enforcement = Some(value);
        self
    }

    pub fn environment_id(mut self, value: Option<String>) -> Self {
        self.environment_id = Some(value);
        self
    }

    pub fn exposure_schedule(
        mut self,
        value: Option<crate::datadogV2::model::ExperimentsPatchProtocolExposureScheduleInput>,
    ) -> Self {
        self.exposure_schedule = Some(value);
        self
    }

    pub fn is_duration_required_to_start(mut self, value: Option<bool>) -> Self {
        self.is_duration_required_to_start = Some(value);
        self
    }

    pub fn is_equal_split_enforced(mut self, value: Option<bool>) -> Self {
        self.is_equal_split_enforced = Some(value);
        self
    }

    pub fn is_metric_limit_enabled(mut self, value: Option<bool>) -> Self {
        self.is_metric_limit_enabled = Some(value);
        self
    }

    pub fn is_minimum_duration_enabled(mut self, value: Option<bool>) -> Self {
        self.is_minimum_duration_enabled = Some(value);
        self
    }

    pub fn metric_groups(
        mut self,
        value: Option<Vec<crate::datadogV2::model::ExperimentsProtocolMetricGroupInput>>,
    ) -> Self {
        self.metric_groups = Some(value);
        self
    }

    pub fn metric_limit(mut self, value: Option<i64>) -> Self {
        self.metric_limit = Some(value);
        self
    }

    pub fn migration_metadata(
        mut self,
        value: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.migration_metadata = Some(value);
        self
    }

    pub fn minimum_duration_unit(mut self, value: Option<String>) -> Self {
        self.minimum_duration_unit = Some(value);
        self
    }

    pub fn minimum_duration_value(mut self, value: Option<i64>) -> Self {
        self.minimum_duration_value = Some(value);
        self
    }

    pub fn name(mut self, value: Option<String>) -> Self {
        self.name = Some(value);
        self
    }

    pub fn primary_metric_id(mut self, value: Option<String>) -> Self {
        self.primary_metric_id = Some(value);
        self
    }

    pub fn subject_type_id(mut self, value: Option<String>) -> Self {
        self.subject_type_id = Some(value);
        self
    }

    pub fn targeting_rules(
        mut self,
        value: Option<Vec<crate::datadogV2::model::ExperimentsProtocolTargetingRuleInput>>,
    ) -> Self {
        self.targeting_rules = Some(value);
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

impl Default for ExperimentsPatchExperimentProtocolRequestDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsPatchExperimentProtocolRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsPatchExperimentProtocolRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsPatchExperimentProtocolRequestDataAttributesVisitor {
            type Value = ExperimentsPatchExperimentProtocolRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut analysis_plan: Option<
                    Option<crate::datadogV2::model::ExperimentsPatchProtocolAnalysisPlanInput>,
                > = None;
                let mut assignment_source_default_properties: Option<Option<Vec<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesAssignmentSourceDefaultPropertiesItems>>> = None;
                let mut assignment_source_id: Option<Option<String>> = None;
                let mut default_duration_days: Option<Option<i64>> = None;
                let mut description: Option<Option<String>> = None;
                let mut enforcement: Option<
                    Option<crate::datadogV2::model::ExperimentsPatchProtocolEnforcementInput>,
                > = None;
                let mut environment_id: Option<Option<String>> = None;
                let mut exposure_schedule: Option<
                    Option<crate::datadogV2::model::ExperimentsPatchProtocolExposureScheduleInput>,
                > = None;
                let mut is_duration_required_to_start: Option<Option<bool>> = None;
                let mut is_equal_split_enforced: Option<Option<bool>> = None;
                let mut is_metric_limit_enabled: Option<Option<bool>> = None;
                let mut is_minimum_duration_enabled: Option<Option<bool>> = None;
                let mut metric_groups: Option<
                    Option<Vec<crate::datadogV2::model::ExperimentsProtocolMetricGroupInput>>,
                > = None;
                let mut metric_limit: Option<Option<i64>> = None;
                let mut migration_metadata: Option<
                    Option<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut minimum_duration_unit: Option<Option<String>> = None;
                let mut minimum_duration_value: Option<Option<i64>> = None;
                let mut name: Option<Option<String>> = None;
                let mut primary_metric_id: Option<Option<String>> = None;
                let mut subject_type_id: Option<Option<String>> = None;
                let mut targeting_rules: Option<
                    Option<Vec<crate::datadogV2::model::ExperimentsProtocolTargetingRuleInput>>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "analysis_plan" => {
                            analysis_plan =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "assignment_source_default_properties" => {
                            assignment_source_default_properties =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "assignment_source_id" => {
                            assignment_source_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "default_duration_days" => {
                            default_duration_days =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enforcement" => {
                            enforcement =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "environment_id" => {
                            environment_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exposure_schedule" => {
                            exposure_schedule =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_duration_required_to_start" => {
                            is_duration_required_to_start =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_equal_split_enforced" => {
                            is_equal_split_enforced =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_metric_limit_enabled" => {
                            is_metric_limit_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_minimum_duration_enabled" => {
                            is_minimum_duration_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_groups" => {
                            metric_groups =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_limit" => {
                            metric_limit =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "migration_metadata" => {
                            migration_metadata =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "minimum_duration_unit" => {
                            minimum_duration_unit =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "minimum_duration_value" => {
                            minimum_duration_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "primary_metric_id" => {
                            primary_metric_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subject_type_id" => {
                            subject_type_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "targeting_rules" => {
                            targeting_rules =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsPatchExperimentProtocolRequestDataAttributes {
                    analysis_plan,
                    assignment_source_default_properties,
                    assignment_source_id,
                    default_duration_days,
                    description,
                    enforcement,
                    environment_id,
                    exposure_schedule,
                    is_duration_required_to_start,
                    is_equal_split_enforced,
                    is_metric_limit_enabled,
                    is_minimum_duration_enabled,
                    metric_groups,
                    metric_limit,
                    migration_metadata,
                    minimum_duration_unit,
                    minimum_duration_value,
                    name,
                    primary_metric_id,
                    subject_type_id,
                    targeting_rules,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsPatchExperimentProtocolRequestDataAttributesVisitor)
    }
}
