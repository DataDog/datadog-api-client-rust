// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes to change. Omitted attributes are unchanged. Which ones can change depends on the bandit's status.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsPatchBanditV2RequestDataAttributes {
    /// The aggregation window: how long after an exposure the bandit keeps counting its reward, in seconds. `null` restores the default of 3600 (1 hour). Only a draft can change it.
    #[serde(
        rename = "aggregation_window_seconds",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub aggregation_window_seconds: Option<Option<i64>>,
    /// The prior the bandit's estimates start from, in the metric's units. Omitted fields keep their current values, or their defaults on create. A change takes effect on the next refresh.
    #[serde(rename = "bayesian_prior")]
    pub bayesian_prior: Option<crate::datadogV2::model::ExperimentsBanditBayesianPriorInputV2>,
    /// The flag and environment the bandit serves on, and who it serves there. The server chooses the rule's `allocation_key`. On create, `feature_flag_id` and `environment_id` are required, and `null` is the same as omitting it. On update, omitted fields are unchanged, and `null` takes a draft off its flag, which also clears its variants and scheduled end. Moving a draft to another flag clears its variants unless the request sets them. Only a draft can change it.
    #[serde(
        rename = "datadog_flag_configuration",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub datadog_flag_configuration:
        Option<Option<crate::datadogV2::model::ExperimentsBanditFlagConfigurationInputV2>>,
    /// The bandit's description.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The exposure SQL model the bandit reads its exposures from. Only a draft can change it.
    #[serde(
        rename = "exposure_sql_model_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub exposure_sql_model_id: Option<Option<uuid::Uuid>>,
    /// How far back each refresh reads observations, in seconds. `null` restores the default of 2592000 (30 days). Only a draft can change it.
    #[serde(
        rename = "lookback_seconds",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub lookback_seconds: Option<Option<i64>>,
    /// The metric the bandit optimizes. It must read a warehouse metric source. Only a draft can change it.
    #[serde(
        rename = "metric_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub metric_id: Option<Option<uuid::Uuid>>,
    /// The bandit's name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The schedule the bandit refreshes on.
    #[serde(rename = "refresh_schedule_id")]
    pub refresh_schedule_id: Option<String>,
    /// When the bandit ends and serves its fallback. Needs a fallback variant. `null` removes the scheduled end.
    #[serde(
        rename = "scheduled_end_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub scheduled_end_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// The subject type the bandit randomizes and measures by. Only a draft can change it.
    #[serde(
        rename = "subject_type_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub subject_type_id: Option<Option<uuid::Uuid>>,
    /// Replaces the bandit's tags and keeps its teams. Allowed in any status.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Replaces the bandit's teams and keeps its tags. Allowed in any status.
    #[serde(rename = "teams")]
    pub teams: Option<Vec<String>>,
    /// Replaces the variants the bandit uses. Once the bandit has started, only `is_fallback` can change, so send the current arms with a different `is_fallback`. Only a draft can have an empty list.
    #[serde(rename = "variants")]
    pub variants: Option<Vec<crate::datadogV2::model::ExperimentsBanditVariantInputV2>>,
    /// How long the bandit serves its arms equally after it starts, in seconds. `null` removes the warmup. Only a draft can change it.
    #[serde(
        rename = "warmup_seconds",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub warmup_seconds: Option<Option<i64>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsPatchBanditV2RequestDataAttributes {
    pub fn new() -> ExperimentsPatchBanditV2RequestDataAttributes {
        ExperimentsPatchBanditV2RequestDataAttributes {
            aggregation_window_seconds: None,
            bayesian_prior: None,
            datadog_flag_configuration: None,
            description: None,
            exposure_sql_model_id: None,
            lookback_seconds: None,
            metric_id: None,
            name: None,
            refresh_schedule_id: None,
            scheduled_end_at: None,
            subject_type_id: None,
            tags: None,
            teams: None,
            variants: None,
            warmup_seconds: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn aggregation_window_seconds(mut self, value: Option<i64>) -> Self {
        self.aggregation_window_seconds = Some(value);
        self
    }

    pub fn bayesian_prior(
        mut self,
        value: crate::datadogV2::model::ExperimentsBanditBayesianPriorInputV2,
    ) -> Self {
        self.bayesian_prior = Some(value);
        self
    }

    pub fn datadog_flag_configuration(
        mut self,
        value: Option<crate::datadogV2::model::ExperimentsBanditFlagConfigurationInputV2>,
    ) -> Self {
        self.datadog_flag_configuration = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn exposure_sql_model_id(mut self, value: Option<uuid::Uuid>) -> Self {
        self.exposure_sql_model_id = Some(value);
        self
    }

    pub fn lookback_seconds(mut self, value: Option<i64>) -> Self {
        self.lookback_seconds = Some(value);
        self
    }

    pub fn metric_id(mut self, value: Option<uuid::Uuid>) -> Self {
        self.metric_id = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn refresh_schedule_id(mut self, value: String) -> Self {
        self.refresh_schedule_id = Some(value);
        self
    }

    pub fn scheduled_end_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.scheduled_end_at = Some(value);
        self
    }

    pub fn subject_type_id(mut self, value: Option<uuid::Uuid>) -> Self {
        self.subject_type_id = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn teams(mut self, value: Vec<String>) -> Self {
        self.teams = Some(value);
        self
    }

    pub fn variants(
        mut self,
        value: Vec<crate::datadogV2::model::ExperimentsBanditVariantInputV2>,
    ) -> Self {
        self.variants = Some(value);
        self
    }

    pub fn warmup_seconds(mut self, value: Option<i64>) -> Self {
        self.warmup_seconds = Some(value);
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

impl Default for ExperimentsPatchBanditV2RequestDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsPatchBanditV2RequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsPatchBanditV2RequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsPatchBanditV2RequestDataAttributesVisitor {
            type Value = ExperimentsPatchBanditV2RequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation_window_seconds: Option<Option<i64>> = None;
                let mut bayesian_prior: Option<
                    crate::datadogV2::model::ExperimentsBanditBayesianPriorInputV2,
                > = None;
                let mut datadog_flag_configuration: Option<
                    Option<crate::datadogV2::model::ExperimentsBanditFlagConfigurationInputV2>,
                > = None;
                let mut description: Option<String> = None;
                let mut exposure_sql_model_id: Option<Option<uuid::Uuid>> = None;
                let mut lookback_seconds: Option<Option<i64>> = None;
                let mut metric_id: Option<Option<uuid::Uuid>> = None;
                let mut name: Option<String> = None;
                let mut refresh_schedule_id: Option<String> = None;
                let mut scheduled_end_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut subject_type_id: Option<Option<uuid::Uuid>> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut teams: Option<Vec<String>> = None;
                let mut variants: Option<
                    Vec<crate::datadogV2::model::ExperimentsBanditVariantInputV2>,
                > = None;
                let mut warmup_seconds: Option<Option<i64>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation_window_seconds" => {
                            aggregation_window_seconds =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bayesian_prior" => {
                            if v.is_null() {
                                continue;
                            }
                            bayesian_prior =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "datadog_flag_configuration" => {
                            datadog_flag_configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exposure_sql_model_id" => {
                            exposure_sql_model_id =
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
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "refresh_schedule_id" => {
                            if v.is_null() {
                                continue;
                            }
                            refresh_schedule_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scheduled_end_at" => {
                            scheduled_end_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subject_type_id" => {
                            subject_type_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teams" => {
                            if v.is_null() {
                                continue;
                            }
                            teams = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variants" => {
                            if v.is_null() {
                                continue;
                            }
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

                let content = ExperimentsPatchBanditV2RequestDataAttributes {
                    aggregation_window_seconds,
                    bayesian_prior,
                    datadog_flag_configuration,
                    description,
                    exposure_sql_model_id,
                    lookback_seconds,
                    metric_id,
                    name,
                    refresh_schedule_id,
                    scheduled_end_at,
                    subject_type_id,
                    tags,
                    teams,
                    variants,
                    warmup_seconds,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsPatchBanditV2RequestDataAttributesVisitor)
    }
}
