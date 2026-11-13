// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// What the bandit has observed, as of its latest saved refresh.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsBanditTrafficSummaryV2DTODataAttributes {
    /// Valid observations per arm on the UTC day of their anchor time, for each day from the bandit's start to the data cutoff that overlaps the requested range. Days without any have zero counts. Empty before the first saved refresh.
    #[serde(rename = "daily")]
    pub daily: Vec<crate::datadogV2::model::ExperimentsBanditDailyTrafficV2>,
    /// The latest warehouse data the latest saved refresh read.
    #[serialize_always]
    #[serde(rename = "data_as_of")]
    pub data_as_of: Option<chrono::DateTime<chrono::Utc>>,
    /// How many observations the latest saved refresh saw, whatever the range. Null when its counts can't be read.
    #[serialize_always]
    #[serde(rename = "observation_counts")]
    pub observation_counts: Option<crate::datadogV2::model::ExperimentsBanditObservationCountsV2>,
    /// The latest refresh that saved results. The counts come from it.
    #[serialize_always]
    #[serde(rename = "refresh_id")]
    pub refresh_id: Option<uuid::Uuid>,
    /// Distinct subjects with a valid observation, whatever the range.
    #[serde(rename = "total_subjects")]
    pub total_subjects: i64,
    /// One row per arm, whatever the range.
    #[serde(rename = "variants")]
    pub variants: Vec<crate::datadogV2::model::ExperimentsBanditTrafficVariantV2>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsBanditTrafficSummaryV2DTODataAttributes {
    pub fn new(
        daily: Vec<crate::datadogV2::model::ExperimentsBanditDailyTrafficV2>,
        data_as_of: Option<chrono::DateTime<chrono::Utc>>,
        observation_counts: Option<crate::datadogV2::model::ExperimentsBanditObservationCountsV2>,
        refresh_id: Option<uuid::Uuid>,
        total_subjects: i64,
        variants: Vec<crate::datadogV2::model::ExperimentsBanditTrafficVariantV2>,
    ) -> ExperimentsBanditTrafficSummaryV2DTODataAttributes {
        ExperimentsBanditTrafficSummaryV2DTODataAttributes {
            daily,
            data_as_of,
            observation_counts,
            refresh_id,
            total_subjects,
            variants,
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

impl<'de> Deserialize<'de> for ExperimentsBanditTrafficSummaryV2DTODataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsBanditTrafficSummaryV2DTODataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsBanditTrafficSummaryV2DTODataAttributesVisitor {
            type Value = ExperimentsBanditTrafficSummaryV2DTODataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut daily: Option<
                    Vec<crate::datadogV2::model::ExperimentsBanditDailyTrafficV2>,
                > = None;
                let mut data_as_of: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut observation_counts: Option<
                    Option<crate::datadogV2::model::ExperimentsBanditObservationCountsV2>,
                > = None;
                let mut refresh_id: Option<Option<uuid::Uuid>> = None;
                let mut total_subjects: Option<i64> = None;
                let mut variants: Option<
                    Vec<crate::datadogV2::model::ExperimentsBanditTrafficVariantV2>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "daily" => {
                            daily = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_as_of" => {
                            data_as_of = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "observation_counts" => {
                            observation_counts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "refresh_id" => {
                            refresh_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_subjects" => {
                            total_subjects =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let daily = daily.ok_or_else(|| M::Error::missing_field("daily"))?;
                let data_as_of = data_as_of.ok_or_else(|| M::Error::missing_field("data_as_of"))?;
                let observation_counts = observation_counts
                    .ok_or_else(|| M::Error::missing_field("observation_counts"))?;
                let refresh_id = refresh_id.ok_or_else(|| M::Error::missing_field("refresh_id"))?;
                let total_subjects =
                    total_subjects.ok_or_else(|| M::Error::missing_field("total_subjects"))?;
                let variants = variants.ok_or_else(|| M::Error::missing_field("variants"))?;

                let content = ExperimentsBanditTrafficSummaryV2DTODataAttributes {
                    daily,
                    data_as_of,
                    observation_counts,
                    refresh_id,
                    total_subjects,
                    variants,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsBanditTrafficSummaryV2DTODataAttributesVisitor)
    }
}
