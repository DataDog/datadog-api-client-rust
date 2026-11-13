// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// What the bandit's refreshes have learned.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsBanditResultsV2DTODataAttributes {
    /// The probability mass of each confidence interval.
    #[serde(rename = "confidence_level")]
    pub confidence_level: f64,
    /// The configuration version the latest saved refresh used. It can be older than the bandit's, because changes such as a new end date don't invalidate results.
    #[serialize_always]
    #[serde(rename = "config_version")]
    pub config_version: Option<i64>,
    /// The latest warehouse data the latest saved refresh read.
    #[serialize_always]
    #[serde(rename = "data_as_of")]
    pub data_as_of: Option<chrono::DateTime<chrono::Utc>>,
    /// The direction the bandit optimizes the metric in. It's fixed when the bandit starts. Null before the first saved refresh.
    #[serialize_always]
    #[serde(rename = "desired_change")]
    pub desired_change: Option<crate::datadogV2::model::ExperimentsBanditDesiredChange>,
    /// An estimate of when the next scheduled refresh starts. Null when the bandit isn't running or the time can't be estimated. Only daily and hourly schedules get an estimate.
    #[serialize_always]
    #[serde(rename = "next_refresh_at")]
    pub next_refresh_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The key of the arm most likely to be best, when one arm leads and the bandit is outside warmup. Null otherwise.
    #[serialize_always]
    #[serde(rename = "optimal_variant_key")]
    pub optimal_variant_key: Option<String>,
    /// Why no optimal variant is reported. Absent when one is, or when the cause isn't known.
    #[serde(rename = "optimal_variant_unavailable_reason")]
    pub optimal_variant_unavailable_reason:
        Option<crate::datadogV2::model::ExperimentsBanditOptimalVariantUnavailableReason>,
    /// Each arm's point estimate from up to the latest 400 saved refreshes in the requested range, oldest first.
    #[serde(rename = "point_estimates")]
    pub point_estimates: Vec<crate::datadogV2::model::ExperimentsBanditPointEstimateV2>,
    /// The latest refresh that saved results. The estimates come from it.
    #[serialize_always]
    #[serde(rename = "refresh_id")]
    pub refresh_id: Option<uuid::Uuid>,
    /// When the latest saved refresh saved its results.
    #[serialize_always]
    #[serde(rename = "results_last_updated")]
    pub results_last_updated: Option<chrono::DateTime<chrono::Utc>>,
    /// WAITING_FOR_FIRST_REFRESH until a refresh saves results. STATS_UNAVAILABLE when the latest saved refresh has no complete probabilities of being optimal. ENDED once the bandit is stopped or completed; its last estimates stay. Otherwise WARMUP or OPTIMIZING says which policy the bandit serves. A paused bandit keeps its state.
    #[serde(rename = "state")]
    pub state: crate::datadogV2::model::ExperimentsBanditResultsState,
    /// Whether the latest saved refresh calculated its statistics. Absent for older refreshes.
    #[serde(rename = "statistics_status")]
    pub statistics_status: Option<crate::datadogV2::model::ExperimentsBanditStatisticsStatus>,
    /// One row per arm, in arm order.
    #[serde(rename = "variants")]
    pub variants: Vec<crate::datadogV2::model::ExperimentsBanditVariantResultV2>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsBanditResultsV2DTODataAttributes {
    pub fn new(
        confidence_level: f64,
        config_version: Option<i64>,
        data_as_of: Option<chrono::DateTime<chrono::Utc>>,
        desired_change: Option<crate::datadogV2::model::ExperimentsBanditDesiredChange>,
        next_refresh_at: Option<chrono::DateTime<chrono::Utc>>,
        optimal_variant_key: Option<String>,
        point_estimates: Vec<crate::datadogV2::model::ExperimentsBanditPointEstimateV2>,
        refresh_id: Option<uuid::Uuid>,
        results_last_updated: Option<chrono::DateTime<chrono::Utc>>,
        state: crate::datadogV2::model::ExperimentsBanditResultsState,
        variants: Vec<crate::datadogV2::model::ExperimentsBanditVariantResultV2>,
    ) -> ExperimentsBanditResultsV2DTODataAttributes {
        ExperimentsBanditResultsV2DTODataAttributes {
            confidence_level,
            config_version,
            data_as_of,
            desired_change,
            next_refresh_at,
            optimal_variant_key,
            optimal_variant_unavailable_reason: None,
            point_estimates,
            refresh_id,
            results_last_updated,
            state,
            statistics_status: None,
            variants,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn optimal_variant_unavailable_reason(
        mut self,
        value: crate::datadogV2::model::ExperimentsBanditOptimalVariantUnavailableReason,
    ) -> Self {
        self.optimal_variant_unavailable_reason = Some(value);
        self
    }

    pub fn statistics_status(
        mut self,
        value: crate::datadogV2::model::ExperimentsBanditStatisticsStatus,
    ) -> Self {
        self.statistics_status = Some(value);
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

impl<'de> Deserialize<'de> for ExperimentsBanditResultsV2DTODataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsBanditResultsV2DTODataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsBanditResultsV2DTODataAttributesVisitor {
            type Value = ExperimentsBanditResultsV2DTODataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut confidence_level: Option<f64> = None;
                let mut config_version: Option<Option<i64>> = None;
                let mut data_as_of: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut desired_change: Option<
                    Option<crate::datadogV2::model::ExperimentsBanditDesiredChange>,
                > = None;
                let mut next_refresh_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut optimal_variant_key: Option<Option<String>> = None;
                let mut optimal_variant_unavailable_reason: Option<
                    crate::datadogV2::model::ExperimentsBanditOptimalVariantUnavailableReason,
                > = None;
                let mut point_estimates: Option<
                    Vec<crate::datadogV2::model::ExperimentsBanditPointEstimateV2>,
                > = None;
                let mut refresh_id: Option<Option<uuid::Uuid>> = None;
                let mut results_last_updated: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut state: Option<crate::datadogV2::model::ExperimentsBanditResultsState> =
                    None;
                let mut statistics_status: Option<
                    crate::datadogV2::model::ExperimentsBanditStatisticsStatus,
                > = None;
                let mut variants: Option<
                    Vec<crate::datadogV2::model::ExperimentsBanditVariantResultV2>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "confidence_level" => {
                            confidence_level =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "config_version" => {
                            config_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_as_of" => {
                            data_as_of = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "desired_change" => {
                            desired_change =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _desired_change) = desired_change {
                                match _desired_change {
                                    Some(crate::datadogV2::model::ExperimentsBanditDesiredChange::UnparsedObject(_desired_change)) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "next_refresh_at" => {
                            next_refresh_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "point_estimates" => {
                            point_estimates =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "refresh_id" => {
                            refresh_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "results_last_updated" => {
                            results_last_updated =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _state) = state {
                                match _state {
                                    crate::datadogV2::model::ExperimentsBanditResultsState::UnparsedObject(_state) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "statistics_status" => {
                            if v.is_null() {
                                continue;
                            }
                            statistics_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _statistics_status) = statistics_status {
                                match _statistics_status {
                                    crate::datadogV2::model::ExperimentsBanditStatisticsStatus::UnparsedObject(_statistics_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "variants" => {
                            variants = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let confidence_level =
                    confidence_level.ok_or_else(|| M::Error::missing_field("confidence_level"))?;
                let config_version =
                    config_version.ok_or_else(|| M::Error::missing_field("config_version"))?;
                let data_as_of = data_as_of.ok_or_else(|| M::Error::missing_field("data_as_of"))?;
                let desired_change =
                    desired_change.ok_or_else(|| M::Error::missing_field("desired_change"))?;
                let next_refresh_at =
                    next_refresh_at.ok_or_else(|| M::Error::missing_field("next_refresh_at"))?;
                let optimal_variant_key = optimal_variant_key
                    .ok_or_else(|| M::Error::missing_field("optimal_variant_key"))?;
                let point_estimates =
                    point_estimates.ok_or_else(|| M::Error::missing_field("point_estimates"))?;
                let refresh_id = refresh_id.ok_or_else(|| M::Error::missing_field("refresh_id"))?;
                let results_last_updated = results_last_updated
                    .ok_or_else(|| M::Error::missing_field("results_last_updated"))?;
                let state = state.ok_or_else(|| M::Error::missing_field("state"))?;
                let variants = variants.ok_or_else(|| M::Error::missing_field("variants"))?;

                let content = ExperimentsBanditResultsV2DTODataAttributes {
                    confidence_level,
                    config_version,
                    data_as_of,
                    desired_change,
                    next_refresh_at,
                    optimal_variant_key,
                    optimal_variant_unavailable_reason,
                    point_estimates,
                    refresh_id,
                    results_last_updated,
                    state,
                    statistics_status,
                    variants,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsBanditResultsV2DTODataAttributesVisitor)
    }
}
