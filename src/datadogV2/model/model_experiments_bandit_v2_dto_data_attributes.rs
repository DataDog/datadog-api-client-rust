// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A bandit's setup, the policy it serves, and its latest refresh.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsBanditV2DTODataAttributes {
    /// The policy the bandit serves. Null until the bandit starts.
    #[serialize_always]
    #[serde(rename = "active_policy")]
    pub active_policy: Option<crate::datadogV2::model::ExperimentsBanditPolicyV2>,
    /// How long after an exposure its reward is counted, in seconds.
    #[serde(rename = "aggregation_window_seconds")]
    pub aggregation_window_seconds: i64,
    /// The prior the bandit's estimates start from.
    #[serde(rename = "bayesian_prior")]
    pub bayesian_prior: crate::datadogV2::model::ExperimentsBanditBayesianPriorV2,
    /// The version of the bandit's configuration. Each saved change adds one.
    #[serde(rename = "config_version")]
    pub config_version: i64,
    /// When the bandit was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The flag rule the bandit serves through. Absent until a draft is put on a flag, and when the caller lacks feature flag and environment read permissions (`meta.warnings` then says so).
    #[serde(rename = "datadog_flag_configuration")]
    pub datadog_flag_configuration:
        Option<crate::datadogV2::model::ExperimentsBanditFlagConfigurationV2>,
    /// The bandit's description.
    #[serde(rename = "description")]
    pub description: String,
    /// When the bandit was stopped or reached its scheduled end. Null while it hasn't ended.
    #[serialize_always]
    #[serde(rename = "ended_at")]
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The exposure SQL model the bandit reads its exposures from. Null until set on a draft.
    #[serialize_always]
    #[serde(rename = "exposure_sql_model_id")]
    pub exposure_sql_model_id: Option<uuid::Uuid>,
    /// The most recently started refresh. Null until one starts.
    #[serialize_always]
    #[serde(rename = "latest_refresh")]
    pub latest_refresh: Option<crate::datadogV2::model::ExperimentsBanditRefreshV2>,
    /// How far back each refresh reads observations, in seconds.
    #[serde(rename = "lookback_seconds")]
    pub lookback_seconds: i64,
    /// The metric the bandit optimizes. Null until set on a draft.
    #[serialize_always]
    #[serde(rename = "metric_id")]
    pub metric_id: Option<uuid::Uuid>,
    /// The bandit's name.
    #[serde(rename = "name")]
    pub name: String,
    /// The key of the arm most likely to be best, when one arm leads and the bandit is outside warmup. Null otherwise. Matches Get bandit results.
    #[serialize_always]
    #[serde(rename = "optimal_variant_key")]
    pub optimal_variant_key: Option<String>,
    /// Why no optimal variant is reported. Absent when one is, or when the cause isn't known.
    #[serde(rename = "optimal_variant_unavailable_reason")]
    pub optimal_variant_unavailable_reason:
        Option<crate::datadogV2::model::ExperimentsBanditOptimalVariantUnavailableReason>,
    /// The schedule the bandit refreshes on.
    #[serialize_always]
    #[serde(rename = "refresh_schedule_id")]
    pub refresh_schedule_id: Option<String>,
    /// When the bandit ends and serves its fallback. Null when it has no scheduled end.
    #[serialize_always]
    #[serde(rename = "scheduled_end_at")]
    pub scheduled_end_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Why the scheduled end couldn't run. It isn't retried until the end or fallback changes. Absent when there's no failure.
    #[serde(rename = "scheduled_end_failure")]
    pub scheduled_end_failure:
        Option<crate::datadogV2::model::ExperimentsBanditScheduledEndFailureV2>,
    /// When the bandit started. Null until it starts.
    #[serialize_always]
    #[serde(rename = "started_at")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The bandit's status. STOPPED means the bandit was stopped and serves the variant chosen then. COMPLETED means it reached its scheduled end and serves its fallback.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::ExperimentsBanditStatus,
    /// The subject type the bandit randomizes and measures by. Null until set on a draft.
    #[serialize_always]
    #[serde(rename = "subject_type_id")]
    pub subject_type_id: Option<uuid::Uuid>,
    /// The bandit's tags, without team tags.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// Handles of the teams that own the bandit.
    #[serde(rename = "teams")]
    pub teams: Vec<String>,
    /// When the bandit was last changed.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// The flag variants the bandit uses: its arms in arm order, then its fallback when the fallback isn't an arm.
    #[serde(rename = "variants")]
    pub variants: Vec<crate::datadogV2::model::ExperimentsBanditVariantV2>,
    /// How long after starting the bandit serves its arms equally, in seconds. Null when it has no warmup.
    #[serialize_always]
    #[serde(rename = "warmup_seconds")]
    pub warmup_seconds: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsBanditV2DTODataAttributes {
    pub fn new(
        active_policy: Option<crate::datadogV2::model::ExperimentsBanditPolicyV2>,
        aggregation_window_seconds: i64,
        bayesian_prior: crate::datadogV2::model::ExperimentsBanditBayesianPriorV2,
        config_version: i64,
        created_at: chrono::DateTime<chrono::Utc>,
        description: String,
        ended_at: Option<chrono::DateTime<chrono::Utc>>,
        exposure_sql_model_id: Option<uuid::Uuid>,
        latest_refresh: Option<crate::datadogV2::model::ExperimentsBanditRefreshV2>,
        lookback_seconds: i64,
        metric_id: Option<uuid::Uuid>,
        name: String,
        optimal_variant_key: Option<String>,
        refresh_schedule_id: Option<String>,
        scheduled_end_at: Option<chrono::DateTime<chrono::Utc>>,
        started_at: Option<chrono::DateTime<chrono::Utc>>,
        status: crate::datadogV2::model::ExperimentsBanditStatus,
        subject_type_id: Option<uuid::Uuid>,
        tags: Vec<String>,
        teams: Vec<String>,
        updated_at: chrono::DateTime<chrono::Utc>,
        variants: Vec<crate::datadogV2::model::ExperimentsBanditVariantV2>,
        warmup_seconds: Option<i64>,
    ) -> ExperimentsBanditV2DTODataAttributes {
        ExperimentsBanditV2DTODataAttributes {
            active_policy,
            aggregation_window_seconds,
            bayesian_prior,
            config_version,
            created_at,
            datadog_flag_configuration: None,
            description,
            ended_at,
            exposure_sql_model_id,
            latest_refresh,
            lookback_seconds,
            metric_id,
            name,
            optimal_variant_key,
            optimal_variant_unavailable_reason: None,
            refresh_schedule_id,
            scheduled_end_at,
            scheduled_end_failure: None,
            started_at,
            status,
            subject_type_id,
            tags,
            teams,
            updated_at,
            variants,
            warmup_seconds,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn datadog_flag_configuration(
        mut self,
        value: crate::datadogV2::model::ExperimentsBanditFlagConfigurationV2,
    ) -> Self {
        self.datadog_flag_configuration = Some(value);
        self
    }

    pub fn optimal_variant_unavailable_reason(
        mut self,
        value: crate::datadogV2::model::ExperimentsBanditOptimalVariantUnavailableReason,
    ) -> Self {
        self.optimal_variant_unavailable_reason = Some(value);
        self
    }

    pub fn scheduled_end_failure(
        mut self,
        value: crate::datadogV2::model::ExperimentsBanditScheduledEndFailureV2,
    ) -> Self {
        self.scheduled_end_failure = Some(value);
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

impl<'de> Deserialize<'de> for ExperimentsBanditV2DTODataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsBanditV2DTODataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsBanditV2DTODataAttributesVisitor {
            type Value = ExperimentsBanditV2DTODataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut active_policy: Option<
                    Option<crate::datadogV2::model::ExperimentsBanditPolicyV2>,
                > = None;
                let mut aggregation_window_seconds: Option<i64> = None;
                let mut bayesian_prior: Option<
                    crate::datadogV2::model::ExperimentsBanditBayesianPriorV2,
                > = None;
                let mut config_version: Option<i64> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut datadog_flag_configuration: Option<
                    crate::datadogV2::model::ExperimentsBanditFlagConfigurationV2,
                > = None;
                let mut description: Option<String> = None;
                let mut ended_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut exposure_sql_model_id: Option<Option<uuid::Uuid>> = None;
                let mut latest_refresh: Option<
                    Option<crate::datadogV2::model::ExperimentsBanditRefreshV2>,
                > = None;
                let mut lookback_seconds: Option<i64> = None;
                let mut metric_id: Option<Option<uuid::Uuid>> = None;
                let mut name: Option<String> = None;
                let mut optimal_variant_key: Option<Option<String>> = None;
                let mut optimal_variant_unavailable_reason: Option<
                    crate::datadogV2::model::ExperimentsBanditOptimalVariantUnavailableReason,
                > = None;
                let mut refresh_schedule_id: Option<Option<String>> = None;
                let mut scheduled_end_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut scheduled_end_failure: Option<
                    crate::datadogV2::model::ExperimentsBanditScheduledEndFailureV2,
                > = None;
                let mut started_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut status: Option<crate::datadogV2::model::ExperimentsBanditStatus> = None;
                let mut subject_type_id: Option<Option<uuid::Uuid>> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut teams: Option<Vec<String>> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut variants: Option<Vec<crate::datadogV2::model::ExperimentsBanditVariantV2>> =
                    None;
                let mut warmup_seconds: Option<Option<i64>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "active_policy" => {
                            active_policy =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "aggregation_window_seconds" => {
                            aggregation_window_seconds =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bayesian_prior" => {
                            bayesian_prior =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "config_version" => {
                            config_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "datadog_flag_configuration" => {
                            if v.is_null() {
                                continue;
                            }
                            datadog_flag_configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ended_at" => {
                            ended_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exposure_sql_model_id" => {
                            exposure_sql_model_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "latest_refresh" => {
                            latest_refresh =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lookback_seconds" => {
                            lookback_seconds =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_id" => {
                            metric_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "optimal_variant_key" => {
                            optimal_variant_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "optimal_variant_unavailable_reason" => {
                            if v.is_null() {
                                continue;
                            }
                            optimal_variant_unavailable_reason =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _optimal_variant_unavailable_reason) =
                                optimal_variant_unavailable_reason
                            {
                                match _optimal_variant_unavailable_reason {
                                    crate::datadogV2::model::ExperimentsBanditOptimalVariantUnavailableReason::UnparsedObject(_optimal_variant_unavailable_reason) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "refresh_schedule_id" => {
                            refresh_schedule_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scheduled_end_at" => {
                            scheduled_end_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scheduled_end_failure" => {
                            if v.is_null() {
                                continue;
                            }
                            scheduled_end_failure =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "started_at" => {
                            started_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::ExperimentsBanditStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "subject_type_id" => {
                            subject_type_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teams" => {
                            teams = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variants" => {
                            variants = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warmup_seconds" => {
                            warmup_seconds =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let active_policy =
                    active_policy.ok_or_else(|| M::Error::missing_field("active_policy"))?;
                let aggregation_window_seconds = aggregation_window_seconds
                    .ok_or_else(|| M::Error::missing_field("aggregation_window_seconds"))?;
                let bayesian_prior =
                    bayesian_prior.ok_or_else(|| M::Error::missing_field("bayesian_prior"))?;
                let config_version =
                    config_version.ok_or_else(|| M::Error::missing_field("config_version"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let ended_at = ended_at.ok_or_else(|| M::Error::missing_field("ended_at"))?;
                let exposure_sql_model_id = exposure_sql_model_id
                    .ok_or_else(|| M::Error::missing_field("exposure_sql_model_id"))?;
                let latest_refresh =
                    latest_refresh.ok_or_else(|| M::Error::missing_field("latest_refresh"))?;
                let lookback_seconds =
                    lookback_seconds.ok_or_else(|| M::Error::missing_field("lookback_seconds"))?;
                let metric_id = metric_id.ok_or_else(|| M::Error::missing_field("metric_id"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let optimal_variant_key = optimal_variant_key
                    .ok_or_else(|| M::Error::missing_field("optimal_variant_key"))?;
                let refresh_schedule_id = refresh_schedule_id
                    .ok_or_else(|| M::Error::missing_field("refresh_schedule_id"))?;
                let scheduled_end_at =
                    scheduled_end_at.ok_or_else(|| M::Error::missing_field("scheduled_end_at"))?;
                let started_at = started_at.ok_or_else(|| M::Error::missing_field("started_at"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let subject_type_id =
                    subject_type_id.ok_or_else(|| M::Error::missing_field("subject_type_id"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;
                let teams = teams.ok_or_else(|| M::Error::missing_field("teams"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;
                let variants = variants.ok_or_else(|| M::Error::missing_field("variants"))?;
                let warmup_seconds =
                    warmup_seconds.ok_or_else(|| M::Error::missing_field("warmup_seconds"))?;

                let content = ExperimentsBanditV2DTODataAttributes {
                    active_policy,
                    aggregation_window_seconds,
                    bayesian_prior,
                    config_version,
                    created_at,
                    datadog_flag_configuration,
                    description,
                    ended_at,
                    exposure_sql_model_id,
                    latest_refresh,
                    lookback_seconds,
                    metric_id,
                    name,
                    optimal_variant_key,
                    optimal_variant_unavailable_reason,
                    refresh_schedule_id,
                    scheduled_end_at,
                    scheduled_end_failure,
                    started_at,
                    status,
                    subject_type_id,
                    tags,
                    teams,
                    updated_at,
                    variants,
                    warmup_seconds,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsBanditV2DTODataAttributesVisitor)
    }
}
