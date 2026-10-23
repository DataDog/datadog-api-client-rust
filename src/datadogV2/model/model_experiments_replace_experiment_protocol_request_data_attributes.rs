// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Replace a published protocol template. Omitted optional template fields are cleared. Omitted enforcement and migration_metadata are preserved. Enabled limits require their values; allocation settings require environment_id.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsReplaceExperimentProtocolRequestDataAttributes {
    /// Statistical settings. Use `default_duration_days` for duration; `target_duration_days` is not accepted here.
    #[serde(rename = "analysis_plan", default, with = "::serde_with::rust::double_option")]
    pub analysis_plan: Option<Option<crate::datadogV2::model::ExperimentsProtocolAnalysisPlanInput>>,
    /// Assignment properties.
    #[serde(rename = "assignment_source_default_properties")]
    pub assignment_source_default_properties: Option<Vec<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesAssignmentSourceDefaultPropertiesItems>>,
    /// ID of the assignment source selected by the protocol.
    #[serde(rename = "assignment_source_id", default, with = "::serde_with::rust::double_option")]
    pub assignment_source_id: Option<Option<uuid::Uuid>>,
    /// Default experiment duration supplied by the protocol, in days.
    #[serde(rename = "default_duration_days", default, with = "::serde_with::rust::double_option")]
    pub default_duration_days: Option<Option<i64>>,
    /// Complete lock map. Omit this object to preserve current locks. Null is rejected; every field is required.
    #[serde(rename = "enforcement")]
    pub enforcement: Option<crate::datadogV2::model::ExperimentsReplaceProtocolEnforcementInput>,
    /// ID of the feature flag environment used by the experiment.
    #[serde(rename = "environment_id", default, with = "::serde_with::rust::double_option")]
    pub environment_id: Option<Option<uuid::Uuid>>,
    /// Traffic schedule. Publishing and replacement require `NO_ROLLOUT` with exactly one step, or `UNIFORM_INTERVALS` with a positive selection interval and at least one step. Requires `environment_id`.
    #[serde(rename = "exposure_schedule", default, with = "::serde_with::rust::double_option")]
    pub exposure_schedule: Option<Option<crate::datadogV2::model::ExperimentsProtocolExposureScheduleInput>>,
    /// Whether the protocol requires a duration before an experiment can start.
    #[serde(rename = "is_duration_required_to_start")]
    pub is_duration_required_to_start: bool,
    /// Whether the protocol requires equal traffic allocation across variants.
    #[serde(rename = "is_equal_split_enforced")]
    pub is_equal_split_enforced: bool,
    /// Whether the protocol limits the number of metrics.
    #[serde(rename = "is_metric_limit_enabled")]
    pub is_metric_limit_enabled: bool,
    /// Whether the protocol enforces a minimum experiment duration.
    #[serde(rename = "is_minimum_duration_enabled")]
    pub is_minimum_duration_enabled: bool,
    /// Metric groups.
    #[serde(rename = "metric_groups")]
    pub metric_groups: Vec<crate::datadogV2::model::ExperimentsProtocolMetricGroupInput>,
    /// Maximum number of metrics allowed by the protocol.
    #[serde(rename = "metric_limit", default, with = "::serde_with::rust::double_option")]
    pub metric_limit: Option<Option<i64>>,
    /// Metadata for imported resources. Must encode to at most 4096 bytes. Null clears it; omission preserves it on update.
    #[serde(rename = "migration_metadata", default, with = "::serde_with::rust::double_option")]
    pub migration_metadata: Option<Option<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// Minimum duration unit, DAYS or WEEKS.
    #[serde(rename = "minimum_duration_unit", default, with = "::serde_with::rust::double_option")]
    pub minimum_duration_unit: Option<Option<String>>,
    /// Minimum experiment duration in the specified unit.
    #[serde(rename = "minimum_duration_value", default, with = "::serde_with::rust::double_option")]
    pub minimum_duration_value: Option<Option<i64>>,
    /// ID of the primary metric supplied by the protocol.
    #[serde(rename = "primary_metric_id", default, with = "::serde_with::rust::double_option")]
    pub primary_metric_id: Option<Option<uuid::Uuid>>,
    /// ID of the subject type used by this configuration.
    #[serde(rename = "subject_type_id")]
    pub subject_type_id: uuid::Uuid,
    /// Targeting rules.
    #[serde(rename = "targeting_rules")]
    pub targeting_rules: Option<Vec<crate::datadogV2::model::ExperimentsProtocolTargetingRuleInput>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsReplaceExperimentProtocolRequestDataAttributes {
    pub fn new(
        is_duration_required_to_start: bool,
        is_equal_split_enforced: bool,
        is_metric_limit_enabled: bool,
        is_minimum_duration_enabled: bool,
        metric_groups: Vec<crate::datadogV2::model::ExperimentsProtocolMetricGroupInput>,
        subject_type_id: uuid::Uuid,
    ) -> ExperimentsReplaceExperimentProtocolRequestDataAttributes {
        ExperimentsReplaceExperimentProtocolRequestDataAttributes {
            analysis_plan: None,
            assignment_source_default_properties: None,
            assignment_source_id: None,
            default_duration_days: None,
            enforcement: None,
            environment_id: None,
            exposure_schedule: None,
            is_duration_required_to_start,
            is_equal_split_enforced,
            is_metric_limit_enabled,
            is_minimum_duration_enabled,
            metric_groups,
            metric_limit: None,
            migration_metadata: None,
            minimum_duration_unit: None,
            minimum_duration_value: None,
            primary_metric_id: None,
            subject_type_id,
            targeting_rules: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn analysis_plan(
        mut self,
        value: Option<crate::datadogV2::model::ExperimentsProtocolAnalysisPlanInput>,
    ) -> Self {
        self.analysis_plan = Some(value);
        self
    }

    pub fn assignment_source_default_properties(
        mut self,
        value: Vec<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesAssignmentSourceDefaultPropertiesItems>,
    ) -> Self {
        self.assignment_source_default_properties = Some(value);
        self
    }

    pub fn assignment_source_id(mut self, value: Option<uuid::Uuid>) -> Self {
        self.assignment_source_id = Some(value);
        self
    }

    pub fn default_duration_days(mut self, value: Option<i64>) -> Self {
        self.default_duration_days = Some(value);
        self
    }

    pub fn enforcement(
        mut self,
        value: crate::datadogV2::model::ExperimentsReplaceProtocolEnforcementInput,
    ) -> Self {
        self.enforcement = Some(value);
        self
    }

    pub fn environment_id(mut self, value: Option<uuid::Uuid>) -> Self {
        self.environment_id = Some(value);
        self
    }

    pub fn exposure_schedule(
        mut self,
        value: Option<crate::datadogV2::model::ExperimentsProtocolExposureScheduleInput>,
    ) -> Self {
        self.exposure_schedule = Some(value);
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

    pub fn primary_metric_id(mut self, value: Option<uuid::Uuid>) -> Self {
        self.primary_metric_id = Some(value);
        self
    }

    pub fn targeting_rules(
        mut self,
        value: Vec<crate::datadogV2::model::ExperimentsProtocolTargetingRuleInput>,
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

impl<'de> Deserialize<'de> for ExperimentsReplaceExperimentProtocolRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsReplaceExperimentProtocolRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsReplaceExperimentProtocolRequestDataAttributesVisitor {
            type Value = ExperimentsReplaceExperimentProtocolRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut analysis_plan: Option<
                    Option<crate::datadogV2::model::ExperimentsProtocolAnalysisPlanInput>,
                > = None;
                let mut assignment_source_default_properties: Option<Vec<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesAssignmentSourceDefaultPropertiesItems>> = None;
                let mut assignment_source_id: Option<Option<uuid::Uuid>> = None;
                let mut default_duration_days: Option<Option<i64>> = None;
                let mut enforcement: Option<
                    crate::datadogV2::model::ExperimentsReplaceProtocolEnforcementInput,
                > = None;
                let mut environment_id: Option<Option<uuid::Uuid>> = None;
                let mut exposure_schedule: Option<
                    Option<crate::datadogV2::model::ExperimentsProtocolExposureScheduleInput>,
                > = None;
                let mut is_duration_required_to_start: Option<bool> = None;
                let mut is_equal_split_enforced: Option<bool> = None;
                let mut is_metric_limit_enabled: Option<bool> = None;
                let mut is_minimum_duration_enabled: Option<bool> = None;
                let mut metric_groups: Option<
                    Vec<crate::datadogV2::model::ExperimentsProtocolMetricGroupInput>,
                > = None;
                let mut metric_limit: Option<Option<i64>> = None;
                let mut migration_metadata: Option<
                    Option<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut minimum_duration_unit: Option<Option<String>> = None;
                let mut minimum_duration_value: Option<Option<i64>> = None;
                let mut primary_metric_id: Option<Option<uuid::Uuid>> = None;
                let mut subject_type_id: Option<uuid::Uuid> = None;
                let mut targeting_rules: Option<
                    Vec<crate::datadogV2::model::ExperimentsProtocolTargetingRuleInput>,
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
                            if v.is_null() {
                                continue;
                            }
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
                        "enforcement" => {
                            if v.is_null() {
                                continue;
                            }
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
                        "primary_metric_id" => {
                            primary_metric_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subject_type_id" => {
                            subject_type_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "targeting_rules" => {
                            if v.is_null() {
                                continue;
                            }
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
                let is_duration_required_to_start = is_duration_required_to_start
                    .ok_or_else(|| M::Error::missing_field("is_duration_required_to_start"))?;
                let is_equal_split_enforced = is_equal_split_enforced
                    .ok_or_else(|| M::Error::missing_field("is_equal_split_enforced"))?;
                let is_metric_limit_enabled = is_metric_limit_enabled
                    .ok_or_else(|| M::Error::missing_field("is_metric_limit_enabled"))?;
                let is_minimum_duration_enabled = is_minimum_duration_enabled
                    .ok_or_else(|| M::Error::missing_field("is_minimum_duration_enabled"))?;
                let metric_groups =
                    metric_groups.ok_or_else(|| M::Error::missing_field("metric_groups"))?;
                let subject_type_id =
                    subject_type_id.ok_or_else(|| M::Error::missing_field("subject_type_id"))?;

                let content = ExperimentsReplaceExperimentProtocolRequestDataAttributes {
                    analysis_plan,
                    assignment_source_default_properties,
                    assignment_source_id,
                    default_duration_days,
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
                    primary_metric_id,
                    subject_type_id,
                    targeting_rules,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ExperimentsReplaceExperimentProtocolRequestDataAttributesVisitor)
    }
}
