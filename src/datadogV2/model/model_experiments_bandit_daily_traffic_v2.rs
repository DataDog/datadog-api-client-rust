// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An arm's observations and new subjects on one UTC day.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsBanditDailyTrafficV2 {
    /// The arm's new subjects from the bandit's start through this day, including days before the requested range.
    #[serde(rename = "cumulative_new_subjects")]
    pub cumulative_new_subjects: i64,
    /// The arm's observations from the bandit's start through this day, including days before the requested range.
    #[serde(rename = "cumulative_observations")]
    pub cumulative_observations: i64,
    /// A UTC date as YYYY-MM-DD.
    #[serde(rename = "date")]
    pub date: String,
    /// The arm's variant ID on the flag.
    #[serde(rename = "feature_flag_variant_id")]
    pub feature_flag_variant_id: uuid::Uuid,
    /// Subjects whose first valid observation is on this day and arm.
    #[serde(rename = "new_subjects")]
    pub new_subjects: i64,
    /// The arm's valid observations on this day.
    #[serde(rename = "observations")]
    pub observations: i64,
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

impl ExperimentsBanditDailyTrafficV2 {
    pub fn new(
        cumulative_new_subjects: i64,
        cumulative_observations: i64,
        date: String,
        feature_flag_variant_id: uuid::Uuid,
        new_subjects: i64,
        observations: i64,
        variant_key: Option<String>,
    ) -> ExperimentsBanditDailyTrafficV2 {
        ExperimentsBanditDailyTrafficV2 {
            cumulative_new_subjects,
            cumulative_observations,
            date,
            feature_flag_variant_id,
            new_subjects,
            observations,
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

impl<'de> Deserialize<'de> for ExperimentsBanditDailyTrafficV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsBanditDailyTrafficV2Visitor;
        impl<'a> Visitor<'a> for ExperimentsBanditDailyTrafficV2Visitor {
            type Value = ExperimentsBanditDailyTrafficV2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cumulative_new_subjects: Option<i64> = None;
                let mut cumulative_observations: Option<i64> = None;
                let mut date: Option<String> = None;
                let mut feature_flag_variant_id: Option<uuid::Uuid> = None;
                let mut new_subjects: Option<i64> = None;
                let mut observations: Option<i64> = None;
                let mut variant_key: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cumulative_new_subjects" => {
                            cumulative_new_subjects =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cumulative_observations" => {
                            cumulative_observations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "date" => {
                            date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "feature_flag_variant_id" => {
                            feature_flag_variant_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "new_subjects" => {
                            new_subjects =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "observations" => {
                            observations =
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
                let cumulative_new_subjects = cumulative_new_subjects
                    .ok_or_else(|| M::Error::missing_field("cumulative_new_subjects"))?;
                let cumulative_observations = cumulative_observations
                    .ok_or_else(|| M::Error::missing_field("cumulative_observations"))?;
                let date = date.ok_or_else(|| M::Error::missing_field("date"))?;
                let feature_flag_variant_id = feature_flag_variant_id
                    .ok_or_else(|| M::Error::missing_field("feature_flag_variant_id"))?;
                let new_subjects =
                    new_subjects.ok_or_else(|| M::Error::missing_field("new_subjects"))?;
                let observations =
                    observations.ok_or_else(|| M::Error::missing_field("observations"))?;
                let variant_key =
                    variant_key.ok_or_else(|| M::Error::missing_field("variant_key"))?;

                let content = ExperimentsBanditDailyTrafficV2 {
                    cumulative_new_subjects,
                    cumulative_observations,
                    date,
                    feature_flag_variant_id,
                    new_subjects,
                    observations,
                    variant_key,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsBanditDailyTrafficV2Visitor)
    }
}
