// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An arm's point estimate from one saved refresh.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsBanditPointEstimateV2 {
    /// A central interval around point_estimate at confidence_level.
    #[serde(rename = "confidence_interval")]
    pub confidence_interval: crate::datadogV2::model::ExperimentsBanditConfidenceIntervalV2,
    /// The latest warehouse data the refresh read.
    #[serialize_always]
    #[serde(rename = "data_as_of")]
    pub data_as_of: Option<chrono::DateTime<chrono::Utc>>,
    /// The arm's variant ID on the flag.
    #[serde(rename = "feature_flag_variant_id")]
    pub feature_flag_variant_id: uuid::Uuid,
    /// The arm's estimated mean of the metric, in the metric's units.
    #[serde(rename = "point_estimate")]
    pub point_estimate: f64,
    /// The refresh the estimate comes from.
    #[serde(rename = "refresh_id")]
    pub refresh_id: uuid::Uuid,
    /// When the refresh saved its results.
    #[serde(rename = "results_last_updated")]
    pub results_last_updated: chrono::DateTime<chrono::Utc>,
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

impl ExperimentsBanditPointEstimateV2 {
    pub fn new(
        confidence_interval: crate::datadogV2::model::ExperimentsBanditConfidenceIntervalV2,
        data_as_of: Option<chrono::DateTime<chrono::Utc>>,
        feature_flag_variant_id: uuid::Uuid,
        point_estimate: f64,
        refresh_id: uuid::Uuid,
        results_last_updated: chrono::DateTime<chrono::Utc>,
        variant_key: Option<String>,
    ) -> ExperimentsBanditPointEstimateV2 {
        ExperimentsBanditPointEstimateV2 {
            confidence_interval,
            data_as_of,
            feature_flag_variant_id,
            point_estimate,
            refresh_id,
            results_last_updated,
            variant_key,
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

impl<'de> Deserialize<'de> for ExperimentsBanditPointEstimateV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsBanditPointEstimateV2Visitor;
        impl<'a> Visitor<'a> for ExperimentsBanditPointEstimateV2Visitor {
            type Value = ExperimentsBanditPointEstimateV2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut confidence_interval: Option<
                    crate::datadogV2::model::ExperimentsBanditConfidenceIntervalV2,
                > = None;
                let mut data_as_of: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut feature_flag_variant_id: Option<uuid::Uuid> = None;
                let mut point_estimate: Option<f64> = None;
                let mut refresh_id: Option<uuid::Uuid> = None;
                let mut results_last_updated: Option<chrono::DateTime<chrono::Utc>> = None;
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
                        "data_as_of" => {
                            data_as_of = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "feature_flag_variant_id" => {
                            feature_flag_variant_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "point_estimate" => {
                            point_estimate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "refresh_id" => {
                            refresh_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "results_last_updated" => {
                            results_last_updated =
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
                let data_as_of = data_as_of.ok_or_else(|| M::Error::missing_field("data_as_of"))?;
                let feature_flag_variant_id = feature_flag_variant_id
                    .ok_or_else(|| M::Error::missing_field("feature_flag_variant_id"))?;
                let point_estimate =
                    point_estimate.ok_or_else(|| M::Error::missing_field("point_estimate"))?;
                let refresh_id = refresh_id.ok_or_else(|| M::Error::missing_field("refresh_id"))?;
                let results_last_updated = results_last_updated
                    .ok_or_else(|| M::Error::missing_field("results_last_updated"))?;
                let variant_key =
                    variant_key.ok_or_else(|| M::Error::missing_field("variant_key"))?;

                let content = ExperimentsBanditPointEstimateV2 {
                    confidence_interval,
                    data_as_of,
                    feature_flag_variant_id,
                    point_estimate,
                    refresh_id,
                    results_last_updated,
                    variant_key,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsBanditPointEstimateV2Visitor)
    }
}
