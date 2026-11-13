// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An arm's estimates from the latest saved refresh.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsBanditVariantResultV2 {
    /// A central interval around point_estimate at confidence_level. Null when the estimate is unavailable.
    #[serialize_always]
    #[serde(rename = "confidence_interval")]
    pub confidence_interval:
        Option<crate::datadogV2::model::ExperimentsBanditNullableConfidenceIntervalV2>,
    /// Where the arm's reward variance came from, or why it's unavailable.
    #[serde(rename = "data_status")]
    pub data_status: Option<crate::datadogV2::model::ExperimentsBanditArmDataStatus>,
    /// The arm's variant ID on the flag.
    #[serde(rename = "feature_flag_variant_id")]
    pub feature_flag_variant_id: uuid::Uuid,
    /// The arm's valid observations whose aggregation window has closed. The estimate uses these.
    #[serde(rename = "mature_observations")]
    pub mature_observations: i64,
    /// The arm's estimated mean of the metric, in the metric's units. It isn't a lift.
    #[serialize_always]
    #[serde(rename = "point_estimate")]
    pub point_estimate: Option<f64>,
    /// Why the arm's estimate and interval are unavailable. Absent when they are available.
    #[serde(rename = "posterior_unavailable_reason")]
    pub posterior_unavailable_reason:
        Option<crate::datadogV2::model::ExperimentsBanditPosteriorUnavailableReason>,
    /// The probability that this arm is the best.
    #[serialize_always]
    #[serde(rename = "probability_of_being_optimal")]
    pub probability_of_being_optimal: Option<f64>,
    /// The arm's variant key.
    #[serialize_always]
    #[serde(rename = "variant_key")]
    pub variant_key: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsBanditVariantResultV2 {
    pub fn new(
        confidence_interval: Option<
            crate::datadogV2::model::ExperimentsBanditNullableConfidenceIntervalV2,
        >,
        feature_flag_variant_id: uuid::Uuid,
        mature_observations: i64,
        point_estimate: Option<f64>,
        probability_of_being_optimal: Option<f64>,
        variant_key: Option<String>,
    ) -> ExperimentsBanditVariantResultV2 {
        ExperimentsBanditVariantResultV2 {
            confidence_interval,
            data_status: None,
            feature_flag_variant_id,
            mature_observations,
            point_estimate,
            posterior_unavailable_reason: None,
            probability_of_being_optimal,
            variant_key,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn data_status(
        mut self,
        value: crate::datadogV2::model::ExperimentsBanditArmDataStatus,
    ) -> Self {
        self.data_status = Some(value);
        self
    }

    pub fn posterior_unavailable_reason(
        mut self,
        value: crate::datadogV2::model::ExperimentsBanditPosteriorUnavailableReason,
    ) -> Self {
        self.posterior_unavailable_reason = Some(value);
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

impl<'de> Deserialize<'de> for ExperimentsBanditVariantResultV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsBanditVariantResultV2Visitor;
        impl<'a> Visitor<'a> for ExperimentsBanditVariantResultV2Visitor {
            type Value = ExperimentsBanditVariantResultV2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut confidence_interval: Option<
                    Option<crate::datadogV2::model::ExperimentsBanditNullableConfidenceIntervalV2>,
                > = None;
                let mut data_status: Option<
                    crate::datadogV2::model::ExperimentsBanditArmDataStatus,
                > = None;
                let mut feature_flag_variant_id: Option<uuid::Uuid> = None;
                let mut mature_observations: Option<i64> = None;
                let mut point_estimate: Option<Option<f64>> = None;
                let mut posterior_unavailable_reason: Option<
                    crate::datadogV2::model::ExperimentsBanditPosteriorUnavailableReason,
                > = None;
                let mut probability_of_being_optimal: Option<Option<f64>> = None;
                let mut variant_key: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "confidence_interval" => {
                            confidence_interval =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_status" => {
                            if v.is_null() {
                                continue;
                            }
                            data_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_status) = data_status {
                                match _data_status {
                                    crate::datadogV2::model::ExperimentsBanditArmDataStatus::UnparsedObject(_data_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "feature_flag_variant_id" => {
                            feature_flag_variant_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mature_observations" => {
                            mature_observations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "point_estimate" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            point_estimate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "posterior_unavailable_reason" => {
                            if v.is_null() {
                                continue;
                            }
                            posterior_unavailable_reason =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _posterior_unavailable_reason) =
                                posterior_unavailable_reason
                            {
                                match _posterior_unavailable_reason {
                                    crate::datadogV2::model::ExperimentsBanditPosteriorUnavailableReason::UnparsedObject(_posterior_unavailable_reason) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "probability_of_being_optimal" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            probability_of_being_optimal =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variant_key" => {
                            variant_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let confidence_interval = confidence_interval
                    .ok_or_else(|| M::Error::missing_field("confidence_interval"))?;
                let feature_flag_variant_id = feature_flag_variant_id
                    .ok_or_else(|| M::Error::missing_field("feature_flag_variant_id"))?;
                let mature_observations = mature_observations
                    .ok_or_else(|| M::Error::missing_field("mature_observations"))?;
                let point_estimate =
                    point_estimate.ok_or_else(|| M::Error::missing_field("point_estimate"))?;
                let probability_of_being_optimal = probability_of_being_optimal
                    .ok_or_else(|| M::Error::missing_field("probability_of_being_optimal"))?;
                let variant_key =
                    variant_key.ok_or_else(|| M::Error::missing_field("variant_key"))?;

                let content = ExperimentsBanditVariantResultV2 {
                    confidence_interval,
                    data_status,
                    feature_flag_variant_id,
                    mature_observations,
                    point_estimate,
                    posterior_unavailable_reason,
                    probability_of_being_optimal,
                    variant_key,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsBanditVariantResultV2Visitor)
    }
}
