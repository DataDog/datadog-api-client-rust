// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings and defaults supplied by the protocol.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsPublicProtocolResponseDataAttributes {
    /// Default statistical settings supplied by the protocol.
    #[serde(rename = "analysis_plan")]
    pub analysis_plan: Option<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesAnalysisPlan>,
    /// Default properties supplied by the protocol's assignment source.
    #[serde(rename = "assignment_source_default_properties")]
    pub assignment_source_default_properties: Option<Vec<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesAssignmentSourceDefaultPropertiesItems>>,
    /// ID of the assignment source selected by the protocol.
    #[serde(rename = "assignment_source_id")]
    pub assignment_source_id: Option<String>,
    /// Default experiment duration supplied by the protocol, in days.
    #[serde(rename = "default_duration_days")]
    pub default_duration_days: Option<i64>,
    /// Text that explains the protocol.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Controls that determine which protocol settings can be changed in an experiment.
    #[serde(rename = "enforcement")]
    pub enforcement: Option<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesEnforcement>,
    /// ID of the feature flag environment used by the experiment.
    #[serde(rename = "environment_id")]
    pub environment_id: Option<String>,
    /// Schedule that controls traffic exposure for experiments created from the protocol.
    #[serde(rename = "exposure_schedule")]
    pub exposure_schedule: Option<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesExposureSchedule>,
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
    /// Metric groups supplied by the protocol.
    #[serde(rename = "metric_groups")]
    pub metric_groups: Vec<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesMetricGroupsItems>,
    /// Maximum number of metrics allowed by the protocol.
    #[serde(rename = "metric_limit")]
    pub metric_limit: Option<i64>,
    /// Metadata retained for resources imported from another system.
    #[serde(rename = "migration_metadata")]
    pub migration_metadata: Option<serde_json::Value>,
    /// Unit used to express the protocol's minimum duration.
    #[serde(rename = "minimum_duration_unit")]
    pub minimum_duration_unit: Option<String>,
    /// Minimum experiment duration in the specified unit.
    #[serde(rename = "minimum_duration_value")]
    pub minimum_duration_value: Option<i64>,
    /// Display name of the protocol.
    #[serde(rename = "name")]
    pub name: String,
    /// Subject type selected by the protocol.
    #[serde(rename = "primary_metric")]
    pub primary_metric: Option<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesSubjectType>,
    /// ID of the primary metric supplied by the protocol.
    #[serde(rename = "primary_metric_id")]
    pub primary_metric_id: Option<String>,
    /// Time when the protocol was published.
    #[serde(rename = "published_at")]
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Publication status of the protocol.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesStatus,
    /// Subject type selected by the protocol.
    #[serde(rename = "subject_type")]
    pub subject_type: Option<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesSubjectType>,
    /// ID of the subject type used by this configuration.
    #[serde(rename = "subject_type_id")]
    pub subject_type_id: Option<String>,
    /// Rules that select subjects for the experiment.
    #[serde(rename = "targeting_rules")]
    pub targeting_rules: Option<Vec<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesTargetingRulesItems>>,
    /// RFC3339 update time. Preserve all fractional seconds when passing this value as expected_updated_at.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsPublicProtocolResponseDataAttributes {
    pub fn new(
        is_duration_required_to_start: bool,
        is_equal_split_enforced: bool,
        is_metric_limit_enabled: bool,
        is_minimum_duration_enabled: bool,
        metric_groups: Vec<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesMetricGroupsItems>,
        name: String,
        status: crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesStatus,
        updated_at: String,
    ) -> ExperimentsPublicProtocolResponseDataAttributes {
        ExperimentsPublicProtocolResponseDataAttributes {
            analysis_plan: None,
            assignment_source_default_properties: None,
            assignment_source_id: None,
            default_duration_days: None,
            description: None,
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
            name,
            primary_metric: None,
            primary_metric_id: None,
            published_at: None,
            status,
            subject_type: None,
            subject_type_id: None,
            targeting_rules: None,
            updated_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn analysis_plan(
        mut self,
        value: crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesAnalysisPlan,
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

    pub fn assignment_source_id(mut self, value: String) -> Self {
        self.assignment_source_id = Some(value);
        self
    }

    pub fn default_duration_days(mut self, value: i64) -> Self {
        self.default_duration_days = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn enforcement(
        mut self,
        value: crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesEnforcement,
    ) -> Self {
        self.enforcement = Some(value);
        self
    }

    pub fn environment_id(mut self, value: String) -> Self {
        self.environment_id = Some(value);
        self
    }

    pub fn exposure_schedule(
        mut self,
        value: crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesExposureSchedule,
    ) -> Self {
        self.exposure_schedule = Some(value);
        self
    }

    pub fn metric_limit(mut self, value: i64) -> Self {
        self.metric_limit = Some(value);
        self
    }

    pub fn migration_metadata(mut self, value: serde_json::Value) -> Self {
        self.migration_metadata = Some(value);
        self
    }

    pub fn minimum_duration_unit(mut self, value: String) -> Self {
        self.minimum_duration_unit = Some(value);
        self
    }

    pub fn minimum_duration_value(mut self, value: i64) -> Self {
        self.minimum_duration_value = Some(value);
        self
    }

    pub fn primary_metric(
        mut self,
        value: crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesSubjectType,
    ) -> Self {
        self.primary_metric = Some(value);
        self
    }

    pub fn primary_metric_id(mut self, value: String) -> Self {
        self.primary_metric_id = Some(value);
        self
    }

    pub fn published_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.published_at = Some(value);
        self
    }

    pub fn subject_type(
        mut self,
        value: crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesSubjectType,
    ) -> Self {
        self.subject_type = Some(value);
        self
    }

    pub fn subject_type_id(mut self, value: String) -> Self {
        self.subject_type_id = Some(value);
        self
    }

    pub fn targeting_rules(
        mut self,
        value: Vec<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesTargetingRulesItems>,
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

impl<'de> Deserialize<'de> for ExperimentsPublicProtocolResponseDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsPublicProtocolResponseDataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsPublicProtocolResponseDataAttributesVisitor {
            type Value = ExperimentsPublicProtocolResponseDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut analysis_plan: Option<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesAnalysisPlan> = None;
                let mut assignment_source_default_properties: Option<Vec<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesAssignmentSourceDefaultPropertiesItems>> = None;
                let mut assignment_source_id: Option<String> = None;
                let mut default_duration_days: Option<i64> = None;
                let mut description: Option<String> = None;
                let mut enforcement: Option<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesEnforcement> = None;
                let mut environment_id: Option<String> = None;
                let mut exposure_schedule: Option<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesExposureSchedule> = None;
                let mut is_duration_required_to_start: Option<bool> = None;
                let mut is_equal_split_enforced: Option<bool> = None;
                let mut is_metric_limit_enabled: Option<bool> = None;
                let mut is_minimum_duration_enabled: Option<bool> = None;
                let mut metric_groups: Option<Vec<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesMetricGroupsItems>> = None;
                let mut metric_limit: Option<i64> = None;
                let mut migration_metadata: Option<serde_json::Value> = None;
                let mut minimum_duration_unit: Option<String> = None;
                let mut minimum_duration_value: Option<i64> = None;
                let mut name: Option<String> = None;
                let mut primary_metric: Option<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesSubjectType> = None;
                let mut primary_metric_id: Option<String> = None;
                let mut published_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut status: Option<
                    crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesStatus,
                > = None;
                let mut subject_type: Option<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesSubjectType> = None;
                let mut subject_type_id: Option<String> = None;
                let mut targeting_rules: Option<Vec<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesTargetingRulesItems>> = None;
                let mut updated_at: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "analysis_plan" => {
                            if v.is_null() {
                                continue;
                            }
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
                            if v.is_null() {
                                continue;
                            }
                            assignment_source_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "default_duration_days" => {
                            if v.is_null() {
                                continue;
                            }
                            default_duration_days =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
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
                            if v.is_null() {
                                continue;
                            }
                            environment_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exposure_schedule" => {
                            if v.is_null() {
                                continue;
                            }
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
                            if v.is_null() {
                                continue;
                            }
                            metric_limit =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "migration_metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            migration_metadata =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "minimum_duration_unit" => {
                            if v.is_null() {
                                continue;
                            }
                            minimum_duration_unit =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "minimum_duration_value" => {
                            if v.is_null() {
                                continue;
                            }
                            minimum_duration_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "primary_metric" => {
                            if v.is_null() {
                                continue;
                            }
                            primary_metric =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "primary_metric_id" => {
                            if v.is_null() {
                                continue;
                            }
                            primary_metric_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "published_at" => {
                            if v.is_null() {
                                continue;
                            }
                            published_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "subject_type" => {
                            if v.is_null() {
                                continue;
                            }
                            subject_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subject_type_id" => {
                            if v.is_null() {
                                continue;
                            }
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
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = ExperimentsPublicProtocolResponseDataAttributes {
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
                    primary_metric,
                    primary_metric_id,
                    published_at,
                    status,
                    subject_type,
                    subject_type_id,
                    targeting_rules,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsPublicProtocolResponseDataAttributesVisitor)
    }
}
